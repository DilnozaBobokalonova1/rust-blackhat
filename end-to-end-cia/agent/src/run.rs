use crate::config;
pub use crate::Error;
use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};
use common::api::{self, UpdateJobResult};
use common::{
    api::{AgentJob, JobPayload, Response as APIResponse},
    crypto,
};
use ed25519_dalek::{ExpandedSecretKey, Signature, Verifier};
use rand::RngCore;
use std::process::Command;
use std::{thread::sleep, time::Duration};
use uuid::Uuid;
use x25519_dalek::x25519;
use zeroize::Zeroize;

pub fn run(api_client: &ureq::Agent, conf: config::Config) -> ! {
    let sleep_for = Duration::from_secs(1);
    let get_job_route = format!("{}/api/agents/{}/job", config::SERVER_URL, conf.agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", config::SERVER_URL);

    loop {
        let server_res: ureq::Response = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error greeting job from server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let api_res: APIResponse<AgentJob> = match server_res.into_json() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error parsing JSON: {}", err); // maybe add tracing instead
                sleep(sleep_for);
                continue;
            }
        };

        log::debug!("API response successfully received");

        let encrypted_job: AgentJob = match api_res.data {
            Some(job) => job,
            None => {
                log::debug!("No job found. Trying again in: {:?}", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };

        let (job_id, job): (Uuid, JobPayload) = match decrypt_and_verify_job(&conf, encrypted_job) {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error decrypting job: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let output = execute_command_agent(job.command, job.args);

        let job_result = match encrypt_and_sign_job_result(
            &conf,
            job_id,
            output,
            job.result_ephemeral_public_key,
        ) {
            Ok(res) => res,
            Err(e) => {
                log::debug!("Error encrypting job result: {}", e);
                sleep(sleep_for);
                continue;
            }
        };

        match api_client
            .post(post_job_result_route.as_str())
            .send_json(ureq::json!(job_result))
        {
            Ok(_) => {}
            Err(e) => {
                log::debug!("Error sending job's result back to client: {}", e);
            }
        }
    }
}

/// XChaCha20Poly1305 requires a specific key size (256-bit for XChaCha20).
/// A KDF is used to generate a key of this exact size and randomness from
/// a potentially less random or differently sized shared secret. In our case,
/// the KDF is used to derive the encryption key from the shared secret
/// created through the X25519 key exchange protocol.
fn encrypt_and_sign_job_result(
    conf: &config::Config,
    job_id: Uuid,
    output: String,
    job_result_ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
) -> Result<UpdateJobResult, Error> {
    let mut rand_generator = rand::rngs::OsRng {};

    // generate ephemeral keypair for job result encryption
    let mut ephemeral_private_key = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut ephemeral_private_key);

    let ephemeral_public_key: [u8; 32] = x25519(
        ephemeral_private_key.clone(),
        x25519_dalek::X25519_BASEPOINT_BYTES,
    );

    // key exchange for job result encryption
    let mut shared_secret: [u8; 32] =
        x25519(ephemeral_private_key, job_result_ephemeral_public_key);

    // generate nonce
    let mut nonce = [0u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);
    // derive key
    let mut kdf =
        blake2::VarBlake2b::new_keyed(&shared_secret, crypto::XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(&nonce);
    let mut key: Box<[u8]> = kdf.finalize_boxed();

    // serialization step
    let job_res_payload = api::JobResult { output };
    let job_res_payload_json: Vec<u8> = serde_json::to_vec(&job_res_payload)?;

    // encrypt the job
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let encrypted_job_result = cipher.encrypt(&nonce.into(), job_res_payload_json.as_ref())?;

    shared_secret.zeroize();
    key.zeroize();

    let mut buffer_for_signature = job_id.as_bytes().to_vec();
    buffer_for_signature.append(&mut conf.agent_id.as_bytes().to_vec());
    buffer_for_signature.append(&mut encrypted_job_result.clone());
    buffer_for_signature.append(&mut ephemeral_public_key.to_vec());
    buffer_for_signature.append(&mut nonce.to_vec());

    let identity: ExpandedSecretKey =
        ed25519_dalek::ExpandedSecretKey::from(&conf.identity_private_key);
    let signature = identity.sign(&buffer_for_signature, &conf.identity_public_key);

    return Ok(UpdateJobResult {
        job_id,
        encrypted_job_result,
        ephemeral_public_key,
        nonce,
        signature: signature.to_bytes().to_vec(),
    });
}

fn execute_command_agent(command: String, args: Vec<String>) -> String {
    let mut ret = String::new();

    let output = match Command::new(command).args(&args).output() {
        Ok(output) => output,
        Err(err) => {
            log::debug!("Error executing command: {}", err);
            return ret;
        }
    };

    // result after executing command as a child process
    ret = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(e) => {
            log::debug!("Error converting command's output to String: {}", e);
            return ret;
        }
    };

    ret
}

/// The code below outlines a cryptographic flow for agent job involving the
/// creation of a signature verification buffer, the validation of a signature,
/// the establishment of a shared secret key using elliptic curve cryptography,
/// and the decryption of encrypted data.
///
/// Note on KDF:
///
///     1. Key Strengthening: To strengthen keys derived from passwords or other not
///     entirely random sources against brute-force attacks.
///
///     2. Key Diversification: To generate multiple keys from a single secret,
///     ensuring that different keys are used for different purposes (encryption,
///     authentication), which enhances security by compartmentalizing potential
///     vulnerabilities.
///
///     3. Format Conformity: To produce keys of the required length and format
///     for specific cryptographic algorithms.
///
/// And as mentioned earlier, - X25519 is a large number but lacks the necessary
/// randomness to be used directly as an encryption key. Hence, we pass it through Blake2b KDF.

fn decrypt_and_verify_job(
    conf: &config::Config,
    job: AgentJob,
) -> Result<(Uuid, JobPayload), Error> {
    if job.signature.len() != crypto::ED25519_SIGNATURE_SIZE {
        return Err(Error::Internal(
            "Error! Job's signature size is not valid".to_string(),
        ));
    }

    let AgentJob {
        id,
        encrypted_job,
        ephemeral_public_key,
        nonce,
        signature,
    } = job;

    let mut verification_buffer = id.as_bytes().to_vec();
    verification_buffer.append(&mut conf.agent_id.as_bytes().to_vec());
    verification_buffer.append(&mut encrypted_job.clone());
    verification_buffer.append(&mut ephemeral_public_key.to_vec());
    verification_buffer.append(&mut nonce.to_vec());

    let signature: Signature = ed25519_dalek::Signature::try_from(&signature[0..64])?;
    if conf
        .client_identity_public_key
        .verify(&verification_buffer, &signature)
        .is_err()
    {
        return Err(Error::Internal(
            "Error. Agent's prekey Signature is not valid".to_string(),
        ));
    }

    // key exchange
    let mut shared_secret: [u8; 32] = x25519(conf.private_prekey, job.ephemeral_public_key);
    // derive the key
    let mut kdf: VarBlake2b =
        blake2::VarBlake2b::new_keyed(&shared_secret, crypto::XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(&job.nonce);

    // Note: Box<[u8]> is used instead of Vec<u8> to save stack space, since they have size of 2 (heap sp, length sp) and 3 words (+ cap sp) respectively.
    let mut key: Box<[u8]> = kdf.finalize_boxed();

    // time to decrypt the job
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let decrypted_job_bytes: Vec<u8> = cipher.decrypt(&nonce.into(), encrypted_job.as_ref())?;

    // to ensure security
    shared_secret.zeroize();
    key.zeroize();

    // deserialization of job
    let job_payload: JobPayload = serde_json::from_slice(&decrypted_job_bytes)?;

    Ok((id, job_payload))
}
