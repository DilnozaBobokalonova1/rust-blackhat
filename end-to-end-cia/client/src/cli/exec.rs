use std::time::Duration;

use crate::{api::Client, config::Config, Error};
use blake2::digest::Update;
use blake2::digest::VariableOutput;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};
use common::{
    api::{self, Agent},
    crypto,
};
use ed25519_dalek::{ExpandedSecretKey, PublicKey, Signature, Verifier};
use rand::RngCore;
use serde_json;
use uuid::Uuid;
use x25519_dalek::x25519;
use zeroize::Zeroize;

pub fn run(api_client: &Client, agent_id: &str, command: &str, conf: Config) -> Result<(), Error> {
    let agent_id = Uuid::parse_str(agent_id)?;
    let sleep_for = Duration::from_millis(550);

    let mut command_with_args: Vec<String> = command
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    if command_with_args.is_empty() {
        return Err(Error::Internal("Command is not valid".to_string()));
    }
    let command = command_with_args.remove(0);
    let args = command_with_args;

    // get agent info
    let agent: api::Agent = api_client.get_agent(agent_id)?;
    let agent_identity_public_key: PublicKey =
        ed25519_dalek::PublicKey::from_bytes(&agent.identity_public_key)?;

    // encrypt job
    let (input, mut job_ephemeral_private_key) = encrypt_and_sign_job(
        &conf,
        command,
        args,
        agent.id,
        agent.public_prekey,
        &agent.public_prekey_signature,
        &agent_identity_public_key,
    )?;

    let job_id = api_client.create_job(input)?;

    Ok(())
}

fn encrypt_and_sign_job(
    conf: &Config,
    command: String,
    args: Vec<String>,
    agent_id: Uuid,
    agent_public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    agent_public_prekey_signature: &Vec<u8>,
    agent_identity_public_key: &PublicKey,
) -> Result<(api::CreateJob, [u8; crypto::X25519_PRIVATE_KEY_SIZE]), Error> {
    if agent_public_prekey_signature.len() != crypto::ED25519_SIGNATURE_SIZE {
        return Err(Error::Internal(
            "Agent's prekey signature size is not valid".to_string(),
        ));
    }

    // agent's prekey verification
    let agent_public_prekey_buffer = agent_public_prekey.to_vec();
    let signature: Signature =
        ed25519_dalek::Signature::try_from(&agent_public_prekey_signature[0..64])?;
    if agent_identity_public_key
        .verify(&agent_public_prekey_buffer, &signature) // passing in public prekey for verification of signature
        .is_err()
    {
        return Err(Error::Internal(
            "Agent's prekey Signature is not valid".to_string(),
        ));
    }

    let mut rand_generator = rand::rngs::OsRng {};

    // generate ephemeral keypair for job encryption
    let mut job_ephemeral_private_key = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut job_ephemeral_private_key);
    let job_ephemeral_public_key = x25519(
        job_ephemeral_private_key.clone(),
        x25519_dalek::X25519_BASEPOINT_BYTES,
    );

    // generate ephemeral keypair for job result encryption
    let mut job_result_ephemeral_private_key = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut job_result_ephemeral_private_key);
    let job_result_ephemeral_public_key = x25519(
        job_result_ephemeral_private_key,
        x25519_dalek::X25519_BASEPOINT_BYTES,
    );

    // key exchange for job encryption
    let mut shared_secret: [u8; 32] = x25519(job_ephemeral_private_key, agent_public_prekey);

    // generating nonce
    let mut nonce = [0u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);

    // derive key using kdf
    let mut kdf =
        blake2::VarBlake2b::new_keyed(&shared_secret, crypto::XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(&nonce);
    let mut key: Box<[u8]> = kdf.finalize_boxed();

    // serialize job
    let encrypted_job_payload = api::JobPayload {
        command,
        args,
        result_ephemeral_public_key: job_result_ephemeral_public_key, // [u8; 32]
    };
    let encrypted_job_json: Vec<u8> = serde_json::to_vec(&encrypted_job_payload)?;

    // encrypt a job
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let encrypted_job = cipher.encrypt(&nonce.into(), encrypted_job_json.as_ref())?;

    shared_secret.zeroize();
    key.zeroize();

    // other input data
    let job_id = Uuid::new_v4();
    let mut buffer_for_signature = job_id.as_bytes().to_vec();
    buffer_for_signature.append(&mut agent_id.as_bytes().to_vec());
    buffer_for_signature.append(&mut encrypted_job.clone());
    buffer_for_signature.append(&mut job_ephemeral_public_key.to_vec());
    buffer_for_signature.append(&mut nonce.to_vec());

    let identity: ExpandedSecretKey =
        ed25519_dalek::ExpandedSecretKey::from(&conf.identity_private_key);
    let signature = identity.sign(&buffer_for_signature, &conf.identity_public_key);

    Ok((
        // note that the signature is done for the whole included body of job meta
        api::CreateJob {
            job_id,
            agent_id,
            encrypted_job,
            ephemeral_public_key: job_ephemeral_public_key,
            nonce,
            signature: signature.to_bytes().to_vec(),
        },
        job_result_ephemeral_private_key,
    ))
}
