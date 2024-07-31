/// Used to embed client's identity public key in agent
/// so here we are generating an identity keypair for the client.
pub fn run() {
    let mut rand_generator = rand::rngs::OsRng {};
    // Usage of Cryptographically Secure Pseudo-Random Number Generator underneath.
    // Remember the characteristics of it:
    //          1. Unpredictability
    //          2. Resistance to Backward Prediction
    //          3. High Entropy (the most important for me)
    let identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let encoded_private_key = base64::encode(identity_keypair.secret.to_bytes());
    println!("private key: {}", encoded_private_key);

    let encoded_public_key = base64::encode(identity_keypair.public.to_bytes());
    println!("public key: {}", encoded_public_key);
}
