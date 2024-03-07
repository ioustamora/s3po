use rand::prelude::*;

pub(crate) fn random_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.gen()).collect()
}

pub(crate) fn bytes_to_mnemonic(bytes: Vec<u8>) -> String {
    mnemonic::to_string(&bytes).replace("--", "-")
}

pub(crate) fn random_mnemonic() -> String {
    bytes_to_mnemonic(random_bytes())
}