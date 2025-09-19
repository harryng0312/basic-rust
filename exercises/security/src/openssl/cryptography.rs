use crate::common::gen_random_byte_arr;
use openssl::symm::{Cipher, Crypter, Mode};
use std::cmp::min;
use tracing::info;
use utils::log::configuration::init_logger;

// #[tests]
pub fn test_aes_ctr() {
    init_logger();
    // prepare
    let cipher = Cipher::aes_128_ctr();
    let key_size = cipher.key_len();
    let iv_size = cipher.iv_len().unwrap();
    info!("key_size:{}, iv_size:{}", key_size, iv_size);
    info!("block_size:{} bytes", cipher.block_size());

    let mut key_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut key_bin).unwrap();
    let mut iv_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut iv_bin).unwrap();
    let data = "Đây là dữ liệu thử nghiệm mã hoá".as_bytes();
    info!("Plain[{}]:{:?}", data.len(), data);
    // encrypt
    let mut cipher_data: Vec<u8> = vec![]; // vec![0u8; data.len() + block_size];
    {
        let mut encryptor = Crypter::new(
            cipher,
            Mode::Encrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        let mut count = 0usize;
        // count += encryptor.update(data, &mut cipher_data).unwrap();
        let mut buff = vec![0u8; key_size];
        for i in (0..data.len()).step_by(key_size) {
            let _r = min(i + key_size, data.len());
            buff.fill(0u8);
            count += encryptor.update(&data[i.._r], &mut buff).unwrap();
            cipher_data.extend(buff.as_slice());
        }
        buff.fill(0u8);
        let f_count = encryptor.finalize(&mut buff).unwrap();
        if f_count > 0 {
            count += f_count;
            cipher_data.extend(buff.as_slice());
        }
        cipher_data.truncate(count);
        info!("Encrypted[{}]: {:?}", count, cipher_data);
    }
    // decrypt
    {
        let mut plain_data: Vec<u8> = vec![0u8; cipher_data.len()];
        let mut decryptor = Crypter::new(
            cipher,
            Mode::Decrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        let mut count = 0usize;
        count += decryptor
            .update(cipher_data.as_slice(), &mut plain_data)
            .unwrap();
        let mut buff: Vec<u8> = vec![0u8; key_size];
        let f_count = decryptor.finalize(&mut *buff).unwrap();
        if f_count > 0 {
            count += f_count;
            plain_data.extend(buff.as_slice());
        }
        plain_data.truncate(count);
        info!("Decrypted[{}]: {:?}", count, plain_data);
        info!("Plain:{}", String::from_utf8_lossy(&plain_data));
    }
}

#[test]
fn test_aes_gcm() {
    init_logger();
    // prepare
    let cipher = Cipher::aes_256_gcm();
    let key_size = cipher.key_len();
    let iv_size = cipher.iv_len().unwrap();
    info!("key_size:{}, iv_size:{}", key_size, iv_size);
    info!("block_size:{} bytes", cipher.block_size());

    let mut key_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut key_bin).unwrap();
    let mut iv_bin: Vec<u8> = vec![0u8; iv_size];
    gen_random_byte_arr(&mut iv_bin).unwrap();
    let mut aad_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut aad_bin).unwrap();
    let data = "Đây là dữ liệu thử nghiệm mã hoá".as_bytes();
    info!("Plain[{}]:{:?}", data.len(), data);
    // encrypt
    let mut cipher_data: Vec<u8> = vec![]; // vec![0u8; data.len() + block_size];
    let mut tag_data: Vec<u8> = vec![0u8; 128 / 8]; // tag length is fixed to 128bit
    {
        let mut encryptor = Crypter::new(
            cipher,
            Mode::Encrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        // encryptor.set_tag_len(key_size).unwrap();
        // tag_data.resize(key_size, 0u8);
        let mut count = 0usize;
        // count += encryptor.update(data, &mut cipher_data).unwrap();
        encryptor.aad_update(aad_bin.as_slice()).unwrap();
        let mut buff = vec![0u8; key_size];
        for i in (0..data.len()).step_by(key_size) {
            let b_right = min(i + key_size, data.len());
            count += encryptor.update(&data[i..b_right], &mut buff).unwrap();
            cipher_data.extend(buff.as_slice());
        }
        buff.fill(0u8);
        let f_count = encryptor.finalize(&mut buff).unwrap();
        if f_count > 0 {
            count += f_count;
            cipher_data.extend(buff.as_slice());
        }
        cipher_data.truncate(count);
        encryptor.get_tag(&mut tag_data).unwrap();
        info!("Encrypted[{}]: {:?}", count, cipher_data);
        info!("Encrypted tag len: {}", tag_data.len());
    }
    // decrypt
    {
        let mut plain_data: Vec<u8> = vec![0u8; cipher_data.len()];
        let mut decryptor = Crypter::new(
            cipher,
            Mode::Decrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        let mut count = 0usize;
        decryptor.aad_update(aad_bin.as_slice()).unwrap();
        decryptor.set_tag(tag_data.as_slice()).unwrap();
        count += decryptor
            .update(cipher_data.as_slice(), &mut plain_data)
            .unwrap();
        let mut buff: Vec<u8> = vec![0u8; key_size];
        let f_count = decryptor.finalize(&mut *buff).unwrap();
        if f_count > 0 {
            count += f_count;
            plain_data.extend(buff.as_slice());
        }
        plain_data.truncate(count);
        info!("Decrypted[{}]: {:?}", count, plain_data);
        info!("Plain:{}", String::from_utf8_lossy(&plain_data));
    }
}

#[test]
fn test_aes_xts() {
    init_logger();
    // prepare
    let cipher = Cipher::aes_256_xts();
    let key_size = cipher.key_len();
    let iv_size = cipher.iv_len().unwrap();
    info!("key_size:{}, iv_size:{}", key_size, iv_size);
    info!("block_size:{} bytes", cipher.block_size());

    let mut key_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut key_bin).unwrap();
    let mut iv_bin: Vec<u8> = vec![0u8; iv_size];
    gen_random_byte_arr(&mut iv_bin).unwrap();
    // let mut aad_bin: Vec<u8> = vec![0u8; key_size];
    // gen_random_byte_arr(&mut aad_bin).unwrap();
    let data = "Đây là dữ liệu thử nghiệm mã hoá".as_bytes();
    info!("Plain[{}]:{:?}", data.len(), data);
    // encrypt
    let mut cipher_data: Vec<u8> = vec![]; // vec![0u8; data.len() + block_size];
                                           // let mut tag_data: Vec<u8> = vec![0u8; 16];
    {
        let mut encryptor = Crypter::new(
            cipher,
            Mode::Encrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        cipher.key_len();
        let mut count = 0usize;
        // count += encryptor.update(data, &mut cipher_data).unwrap();
        // encryptor.aad_update(aad_bin.as_slice()).expect("Cannot add AAD!");
        let mut buff = vec![0u8; key_size];
        for i in (0..data.len()).step_by(key_size) {
            let b_right = min(i + key_size, data.len());
            count += encryptor.update(&data[i..b_right], &mut buff).unwrap();
            cipher_data.extend(buff.as_slice());
        }
        buff.fill(0u8);
        let f_count = encryptor.finalize(&mut buff).unwrap();
        if f_count > 0 {
            count += f_count;
            cipher_data.extend(buff.as_slice());
        }
        cipher_data.truncate(count);
        // encryptor.get_tag(&mut tag_data).unwrap();
        info!("Encrypted[{}]: {:?}", count, cipher_data);
    }
    // decrypt
    {
        let mut plain_data: Vec<u8> = vec![0u8; cipher_data.len()];
        let mut decryptor = Crypter::new(
            cipher,
            Mode::Decrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        let mut count = 0usize;
        // decryptor.aad_update(aad_bin.as_slice()).unwrap();
        // decryptor.set_tag(tag_data.as_slice()).unwrap();
        count += decryptor
            .update(cipher_data.as_slice(), &mut plain_data)
            .unwrap();
        let mut buff: Vec<u8> = vec![0u8; key_size];
        let f_count = decryptor.finalize(&mut *buff).unwrap();
        if f_count > 0 {
            count += f_count;
            plain_data.extend(buff.as_slice());
        }
        plain_data.truncate(count);
        info!("Decrypted[{}]: {:?}", count, plain_data);
        info!("Plain:{}", String::from_utf8_lossy(&plain_data));
    }
}

#[test]
fn test_chacha20_poly1305() {
    init_logger();
    // prepare
    let cipher = Cipher::chacha20_poly1305();
    let key_size = cipher.key_len();
    let iv_size = cipher.iv_len().unwrap();
    info!("key_size:{}, iv_size:{}", key_size, iv_size);
    info!("block_size:{} bytes", cipher.block_size());

    let mut key_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut key_bin).unwrap();
    let mut iv_bin: Vec<u8> = vec![0u8; iv_size];
    gen_random_byte_arr(&mut iv_bin).unwrap();
    let mut aad_bin: Vec<u8> = vec![0u8; key_size];
    gen_random_byte_arr(&mut aad_bin).unwrap();
    let data = "Đây là dữ liệu thử nghiệm mã hoá".as_bytes();
    info!("Plain[{}]: {:?}", data.len(), data);
    // encrypt
    let mut cipher_data: Vec<u8> = vec![]; // vec![0u8; data.len() + block_size];
    let mut tag_data: Vec<u8> = vec![0u8; 16];
    {
        let mut encryptor = Crypter::new(
            cipher,
            Mode::Encrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        cipher.key_len();
        let mut count = 0usize;
        // count += encryptor.update(data, &mut cipher_data).unwrap();
        encryptor
            .aad_update(aad_bin.as_slice())
            .expect("Cannot add AAD!");
        let mut buff = vec![0u8; key_size];
        for i in (0..data.len()).step_by(key_size) {
            let b_right = min(i + key_size, data.len());
            count += encryptor.update(&data[i..b_right], &mut buff).unwrap();
            cipher_data.extend(buff.as_slice());
        }
        buff.fill(0u8);
        let f_count = encryptor.finalize(&mut buff).unwrap();
        if f_count > 0 {
            count += f_count;
            cipher_data.extend(buff.as_slice());
        }
        cipher_data.truncate(count);
        encryptor.get_tag(&mut tag_data).unwrap();
        info!("Encrypted[{}]: {:?}", count, cipher_data);
        info!("Tag size:{}", tag_data.len());
    }
    // decrypt
    {
        let mut plain_data: Vec<u8> = vec![0u8; cipher_data.len()];
        let mut decryptor = Crypter::new(
            cipher,
            Mode::Decrypt,
            key_bin.as_slice(),
            Some(iv_bin.as_slice()),
        )
        .unwrap();
        let mut count = 0usize;
        decryptor.aad_update(aad_bin.as_slice()).unwrap();
        decryptor.set_tag(tag_data.as_slice()).unwrap();
        count += decryptor
            .update(cipher_data.as_slice(), &mut plain_data)
            .unwrap();
        let mut buff: Vec<u8> = vec![0u8; key_size];
        let f_count = decryptor.finalize(&mut buff).unwrap();
        if f_count > 0 {
            count += f_count;
            cipher_data.extend(buff.as_slice());
        }
        plain_data.truncate(count);
        info!("Decrypted[{}]: {:?}", count, plain_data);
        info!("Plain: {}", String::from_utf8_lossy(&plain_data));
    }
}
