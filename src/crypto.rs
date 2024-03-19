use rand::prelude::*;
use std::{fs, str};
use std::ops::Add;
use ecies::{decrypt, encrypt, utils::generate_keypair};
use libsecp256k1::{Message, sign, Signature, verify};
use bs58;
use colored::Colorize;
use crate::config::S3Config;
use crate::console::{ask};

pub(crate) fn random_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.gen()).collect()
}

pub(crate) fn bytes_to_mnemonic(bytes: Vec<u8>) -> String {
    mnemonic::to_string(&bytes)
}

pub(crate) fn mnemonic_to_bytes(mnemonic_string: String) -> Vec<u8> {
    let mut dest = Vec::<u8>::new();
    mnemonic::decode(mnemonic_string, dest.clone()).unwrap();
    dest
}

pub(crate) fn generate_keys() -> ([u8; 32], [u8; 65]) {
    let (sk, pk) = generate_keypair();
    (sk.serialize(), pk.serialize())
}

pub(crate) fn keys_bytes_to_bs58(sk: [u8; 32], pk: [u8; 65]) -> (String, String) {
    (bs58::encode(sk).into_string(), bs58::encode(pk).into_string())
}

pub(crate) fn keys_bs58_to_bytes(sk: String, pk: String) -> ([u8; 32], [u8; 65]) {
    let sk_vec = bs58::decode(sk).into_vec().unwrap();
    let pk_vec = bs58::decode(pk).into_vec().unwrap();
    (<[u8; 32]>::try_from(sk_vec.as_slice()).unwrap(), <[u8; 65]>::try_from(pk_vec.as_slice()).unwrap())
}

pub(crate) fn new_keys() -> (String, String) {
    let (sk, pk) = generate_keys();
    keys_bytes_to_bs58(sk, pk)
}

pub(crate) fn gen_new_keys(mut cfg: S3Config) -> S3Config {
    let (sk_bs58, pk_bs58) = new_keys();
    println!("{}","New cryptographic keys generated and will be saved in config".red());
    println!("{}: {}", "secret key".blue(), sk_bs58);
    println!("{}: {}", "public key".blue(), pk_bs58);
    cfg.sk_bs58 = sk_bs58;
    cfg.pk_bs58 = pk_bs58;
    cfg
}

pub(crate) fn random_mnemonic() -> String {
    bytes_to_mnemonic(random_bytes())
}

pub(crate) fn encrypt_config(cfg: S3Config) {
    encrypt_file(cfg, String::from("/home/inspeere/.config/s3po/default-config.toml"));
}

pub(crate) fn encrypt_file(cfg: S3Config, local_path: String) {
    let file_content = fs::read(local_path.to_string())
        .expect("can't open file for encryption");
    let encrypted_content = encrypt_bytes(cfg, file_content);
    let encrypted_path = local_path.clone() + ".x";
    fs::write( encrypted_path.clone(), &encrypted_content).expect("error writing encrypted file");
    println!("File {} encrypted successfully! Check {}.", local_path, encrypted_path);
}

pub(crate) fn encrypt_bytes(cfg: S3Config, file_content: Vec<u8>) -> Vec<u8> {
    let sk_vec = bs58::decode(cfg.sk_bs58).into_vec().unwrap();
    let sk_bytes = sk_vec.as_slice();
    let pk_vec = bs58::decode(cfg.pk_bs58).into_vec().unwrap();
    let pk_bytes = pk_vec.as_slice();
    encrypt(pk_bytes, &file_content).expect("file encryption error")
}

pub(crate) fn decrypt_config(cfg: S3Config) {
    decrypt_file(cfg, String::from("/home/inspeere/.config/s3po/default-config.toml.x"));
}

pub(crate) fn decrypt_file(cfg: S3Config, encrypted_path: String) {
    //check file extension .x exists or fail
    if !encrypted_path.ends_with(".x") {
        println!("{}", "error ... encrypted file must have .x extension in the end of file name".yellow());
        return;
    }
    let file_content = fs::read(encrypted_path.clone())
        .expect("can't open file for decryption");
    let decrypted_content = decrypt_bytes(cfg, file_content);
    let decrypted_bytes= decrypted_content.as_slice();
    println!("{}: {:?}", "decrypted file", str::from_utf8(decrypted_bytes).unwrap());
    let decrypted_path = encrypted_path.strip_suffix(".x").expect("error stripping encrypted extension");
    fs::write(decrypted_path.clone(), &decrypted_bytes).expect("error writing decrypted file");
    println!("File {} decrypted to {} successfully!", encrypted_path, decrypted_path);
}

pub(crate) fn decrypt_bytes(cfg: S3Config, file_content: Vec<u8>) -> Vec<u8> {
    let sk_vec = bs58::decode(cfg.sk_bs58).into_vec().unwrap();
    let sk_bytes = sk_vec.as_slice();
    let pk_vec = bs58::decode(cfg.pk_bs58).into_vec().unwrap();
    let pk_bytes = pk_vec.as_slice();
    let decrypted_content = decrypt(sk_bytes, &file_content).expect("error file decryption decryption");
    decrypted_content.as_slice().to_vec()
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

    let msg_bytes = MSG.as_bytes();
    //encrypt message
    let encrypted = encrypt(pk_bytes, msg_bytes).unwrap();
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
        msg_bytes,
        decrypted_bytes
    );
    println!(
        "decrypted message: {:?}",
        str::from_utf8(decrypted_bytes).unwrap()
    )
}