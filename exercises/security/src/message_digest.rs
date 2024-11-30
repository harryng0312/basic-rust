use std::error::Error;
use std::io::{Cursor, Read, Write};
use base64_stream::{ToBase64Reader};
use libc::rand;
use log::info;
use openssl::hash::{hash, Hasher, MessageDigest};
use openssl::pkcs5::pbkdf2_hmac;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use rand::Rng;
use utils::log::configuration::init_logger;
use crate::common::{gen_random_byte_arr, to_base64};

#[test]
fn test_sha256() {
    init_logger();
    let str_data = "Đây là dữ liệu cần test thử".as_bytes();
    if let Some(msg_digest)= MessageDigest::from_name("sha256") {
        if let Ok(mut hasher) = Hasher::new(msg_digest) {
            for byte in str_data {
                hasher.update(&[*byte]).unwrap();
            }
            let md_val = hasher.finish().unwrap();
            let md_hashed_b64 = to_base64(md_val.as_ref()).unwrap();
            let b64_size = md_hashed_b64.len();
            info!("Hashed in chunked:[{}->{}]:{}", str_data.len(), b64_size, md_hashed_b64);
        }
        let md_hashed = hash(msg_digest, str_data).unwrap();
        let md_hashed_b64 = to_base64(&md_hashed).unwrap();
        let b64_size = md_hashed_b64.len();
        info!("Hashed by whole:[{}->{}]:{}", str_data.len(), b64_size, md_hashed_b64);
    }
}

#[test]
fn test_hmac_sha256() {
    init_logger();
    let data = "Đây là dữ liệu cần được test thử".as_bytes();
    let secret_key_data = "secret key".as_bytes();
    // let hmac_result = Vec::<u8>::new();
    // create secret key
    let secret_key = PKey::hmac(secret_key_data).expect("Fail to create secret key!");
    // create msg_digest
    let msg_digest = MessageDigest::sha256();
    // create signer
    let mut signer = Signer::new(msg_digest, &secret_key).expect("Fail to create HMAC signer");

    // Update the signer with the data to be hashed
    signer.update(data).expect("Failed to update data");
    signer.flush().expect("Failed to calculate HMAC");
    // Finalize the HMAC
    let hmac_result = signer.sign_to_vec().expect("Failed to finalize HMAC computation");
    let hmac_result_str = to_base64(hmac_result.as_slice()).unwrap();
    info!("HMACSha256 result:{}", hmac_result_str);
    // OzRlpfvM9jvO1lh1zEH5bgZ8Jr0nIwQ5ONk6ZHBB86g=
}

#[test]
fn test_pbkdf2() {
    init_logger();
    let bit_len: usize = 256usize;
    // const bit_len: usize = bit_len_inp;
    let password = b"P@55w0rd";
    let mut salt: Vec<u8>= vec![0u8; bit_len/8];
    gen_random_byte_arr(&mut salt).unwrap();
    let iterator_num = 1_024;
    let key_len = bit_len/8;
    let msg_digest = MessageDigest::sha256();

    // the key that will be derivated
    let mut key_data = vec![0u8; key_len];

    info!("Before\t:Salt:{}", to_base64(salt.as_slice()).unwrap());
    pbkdf2_hmac(password, salt.as_slice(), iterator_num, msg_digest, &mut key_data).expect("PBKDF2 failed!");
    info!("After\t:Salt:{}", to_base64(salt.as_slice()).unwrap());
    info!("After\t:Key :{}", to_base64(key_data.as_slice()).unwrap());
    // info!("Salt: {:?}", salt.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
    // info!("Key: {:?}", key_data.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
}
