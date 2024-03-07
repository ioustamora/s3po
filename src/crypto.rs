use rand::prelude::*;
use std::{io, str};
use ecies::{decrypt, encrypt, utils::generate_keypair};
use libsecp256k1::{Message, sign, Signature, verify};
use bs58;

pub(crate) fn random_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.gen()).collect()
}

pub(crate) fn bytes_to_mnemonic(bytes: Vec<u8>) -> String {
    mnemonic::to_string(&bytes).replace("--", "-")
}

pub(crate) fn generate_keys_bs58() -> (String, String) {
    let (sk, pk) = generate_keypair();
    let (sk_bytes, pk_bytes) = (&sk.serialize(), &pk.serialize());
    (bs58::encode(sk_bytes).into_string(), bs58::encode(pk_bytes).into_string())
}

pub(crate) fn random_mnemonic() -> String {
    bytes_to_mnemonic(random_bytes())
}

pub(crate) fn test_crypto() {
    const MSG: &str = "helloworld";
    println!("initial message: {}", MSG);
    let (sk, pk) = generate_keypair();
    //serialising keys to bytes
    let (sk_bytes, pk_bytes) = (&sk.serialize(), &pk.serialize());
    //encoding keys to base58
    let sk_b58 = bs58::encode(sk_bytes).into_string();
    let pk_b58 = bs58::encode(pk_bytes).into_string();
    println!("base58 secret key: {:?}", sk_b58);
    println!("base58 public key: {:?}", pk_b58);
    //converting base58 keys to bytes back
    let sk_vec = bs58::decode(sk_b58).into_vec().unwrap();
    let sk_bytes = sk_vec.as_slice();
    let pk_vec = bs58::decode(pk_b58).into_vec().unwrap();
    let pk_bytes = pk_vec.as_slice();

    let msg = MSG.as_bytes();
    //encrypt message
    let encrypted = encrypt(pk_bytes, msg).unwrap();
    let encrypted_bytes = encrypted.as_slice();

    //encode encrypted message to base58
    let encrypted_b58 = bs58::encode(encrypted_bytes).into_string();
    println!("XChaCha20-Poly1305 encrypted message in base58: {:?}", encrypted_b58);
    //sign encrypted message and verify signature
    let msg_hash = Message::parse_slice(&encrypted_bytes[..32]).unwrap();
    let (sig, _) = sign(&msg_hash, &sk);
    let sig_bytes = sig.serialize();
    let sig_b58 = bs58::encode(sig_bytes).into_string();
    println!("ECDSA signature in base58: {:?}", sig_b58);
    let sig_vec = bs58::decode(sig_b58).into_vec().unwrap();
    let sig_bytes = sig_vec.as_slice();
    let sig = Signature::parse_standard_slice(&sig_bytes).unwrap();
    let verified = verify(&msg_hash, &sig, &pk);
    println!("Signature verification result: {}", verified);
    //decrypting message
    let decrypted = decrypt(sk_bytes, encrypted_bytes).unwrap();
    let decrypted_bytes= decrypted.as_slice();
    assert_eq!(
        msg,
        decrypted_bytes
    );
    println!(
        "decrypted message: {:?}",
        str::from_utf8(decrypted_bytes).unwrap()
    )
}