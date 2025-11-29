use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use hex::encode as hex_encode;
//sure that TGE is Targem Game Encoding
struct TGEncoding {
    sign: u32,
    size: u32,
    enc_data: Vec<u8>,
}

//todo: decode/encode tge


//chacha is TGE encryption
type ChaCha20Cipher = ChaCha20;

const key: [u8; 32] = [
    0x38, 0x79, 0x2F, 0x42, 0x3F, 0x45, 0x28, 0x48, 0x2B, 0x4D, 0x62, 0x51, 0x65, 0x54, 0x68, 0x57,
    0x6D, 0x5A, 0x71, 0x34, 0x74, 0x37, 0x77, 0x39, 0x7A, 0x24, 0x43, 0x26, 0x46, 0x29, 0x4A, 0x40,
];
const nonce: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x2B, 0x4D, 0x62, 0x51, 0x65, 0x54, 0x68, 0x57,
];
fn chacha_decrypt(data_c: &[u8]) -> Vec<u8> {
    let mut cipher = ChaCha20Cipher::new(&key.into(), &nonce.into());
    let mut buffer = data_c.to_vec();
    cipher.apply_keystream(&mut buffer);
    buffer
}
fn chacha_encrypt(data: &[u8]) -> Vec<u8> {
    let mut cipher = ChaCha20Cipher::new(&key.into(), &nonce.into());
    let mut buffer = data.to_vec();
    cipher.apply_keystream(&mut buffer);
    buffer
}