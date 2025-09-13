use aes::Aes128;
use anyhow::anyhow;
use bytes::Bytes;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, StreamCipher};
use cbc::{Decryptor, Encryptor};
use ctr::Ctr128BE;
use log::info;
use std::ops::{Deref, DerefMut};
use utils::error::app_error::AppResult;
fn aes_ctr_encrypt(bit_length: usize, key: Bytes, iv: Bytes, plain: Bytes) -> AppResult<Bytes> {
    let byte_len: usize = bit_length / 8;
    if key.len() != byte_len || iv.len() != byte_len {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Ctr128BE::<Aes128>::new(key.into(), iv.into());
    let mut encrypted: Vec<u8> = vec![];
    let chunks = plain.chunks(byte_len);
    for chunk in chunks {
        let mut buffer = chunk.to_vec();
        cipher.apply_keystream(&mut buffer);
        encrypted.append(&mut buffer);
    }

    Ok(Bytes::from(encrypted))
}

fn aes_ctr_decrypt(bit_length: usize, key: Bytes, iv: Bytes, encryped: Bytes) -> AppResult<Bytes> {
    aes_ctr_encrypt(bit_length, key, iv, encryped)
}

fn aes_cbc_encrypt(bit_length: usize, key: Bytes, iv: Bytes, plain: Bytes) -> AppResult<Bytes> {
    let block_len: usize = bit_length / 8;
    if key.len() != block_len || iv.len() != block_len {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Encryptor::<Aes128>::new(key.into(), iv.into());
    let mut encrypted: Vec<u8> = vec![];
    let chunks = plain.chunks(block_len);
    let need_pad_final = plain.len() % block_len == 0;
    let mut buffer = vec![0; block_len];
    for chunk in chunks {
        buffer.fill(0u8);
        let buf = buffer.as_mut_slice();
        // buffer.extend(repeat(0u8).take(block_len)); // [0u8; block_len];
        buf[..chunk.len()].copy_from_slice(chunk);
        if chunk.len() < block_len {
            // padding PKCS7
            let pad = (block_len - chunk.len()) as u8;
            buf[chunk.len()..block_len].iter_mut().for_each(|item| {
                *item = pad;
            });
        }
        cipher.encrypt_block_mut(buf.into());
        encrypted.extend_from_slice(&buf);
    }
    // need to pad final block
    if need_pad_final {
        buffer.fill(block_len as u8);
        let buffer = buffer.as_mut_slice();
        cipher.encrypt_block_mut(buffer.into());
        encrypted.extend_from_slice(buffer);
    }
    Ok(Bytes::from(encrypted))
}

fn aes_cbc_decrypt(bit_length: usize, key: Bytes, iv: Bytes, encrypted: Bytes) -> AppResult<Bytes> {
    let block_len: usize = bit_length / 8;
    if key.len() != block_len || iv.len() != block_len {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Decryptor::<Aes128>::new(key.into(), iv.into());
    let mut decrypted: Vec<u8> = vec![];
    let chunks = encrypted.chunks(block_len);

    for chunk in chunks {
        let buffer = &mut *chunk.to_vec();
        cipher.decrypt_block_mut(buffer.into());
        decrypted.extend_from_slice(buffer);
    }

    // Remove PKCS7 padding
    info!("Last decrypted: {:?}", decrypted);
    if let Some(&pad) = decrypted.last() {
        let pad_len = pad as usize;
        let len = decrypted.len();
        if pad_len <= block_len && pad_len <= len {
            decrypted.truncate(len - pad_len);
        }
        // info!(
        //     "Removed Padding, pad_len: {}, decrypted_len: {}",
        //     pad_len,
        //     decrypted.len()
        // );
    }
    Ok(Bytes::from(decrypted))
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use rand_core::{OsRng, RngCore};
    use utils::log::configuration::init_logger;
    #[test]
    fn test_aes_ctr() {
        init_logger();
        let mut rand_bytes = vec![0u8; 32];
        OsRng.fill_bytes(rand_bytes.as_mut_slice());
        let plain = Bytes::from(format!(
            "hello world hello world hello world: {}",
            rand_bytes.len()
        ));
        let key = Bytes::from(rand_bytes[0..16].to_vec());
        let iv = Bytes::from(rand_bytes[16..32].to_vec());
        info!("Plain: {:?}", plain);
        let encrypted = aes_ctr_encrypt(128, key.clone(), iv.clone(), plain);
        let encrypted = encrypted.unwrap();
        info!(
            "Cipher: {:?} len:{}",
            encrypted.as_ref(),
            encrypted.as_ref().len()
        );
        let plain = aes_ctr_decrypt(128, key, iv, encrypted);
        info!("Plain after decrypt: {:?}", plain.unwrap());
    }

    #[test]
    fn test_aes_cbc() {
        init_logger();
        let mut rand_bytes = vec![0u8; 32];
        OsRng.fill_bytes(rand_bytes.as_mut_slice());
        let plain = Bytes::from(format!(
            "hello world hello world hello: {}",
            rand_bytes.len()
        ));
        let key = Bytes::from(rand_bytes[0..16].to_vec());
        let iv = Bytes::from(rand_bytes[16..32].to_vec());
        info!("Plain: {:?} {}", plain, plain.len());
        let encrypted = aes_cbc_encrypt(128, key.clone(), iv.clone(), plain);
        let encrypted = encrypted.unwrap();
        info!(
            "Cipher: {:?} len:{}",
            encrypted.as_ref(),
            encrypted.as_ref().len()
        );
        let plain = aes_cbc_decrypt(128, key, iv, encrypted);
        let plain = plain.unwrap();
        info!("Plain after decrypt: {:?}, len:{}", plain, plain.len());
    }
}
