use aes::Aes128;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{AeadInPlace, Aes128Gcm};
use aes_gcm::{KeyInit, Nonce};
use anyhow::anyhow;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, StreamCipher};
use cbc::{Decryptor, Encryptor};
use cipher::consts::U12;
use ctr::Ctr128BE;
use log::info;
use utils::error::app_error::AppResult;
fn aes_ctr_encrypt(bit_length: usize, key: &[u8], iv: &[u8], plain: &[u8]) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || iv.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Ctr128BE::<Aes128>::new(key.into(), iv.into());
    let mut encrypted: Vec<u8> = vec![];
    let chunks = plain.chunks(block_size);
    for chunk in chunks {
        let mut buffer = chunk.to_vec();
        cipher.apply_keystream(&mut buffer);
        encrypted.append(&mut buffer);
    }

    Ok(encrypted)
}

fn aes_ctr_decrypt(
    bit_length: usize,
    key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> AppResult<Vec<u8>> {
    aes_ctr_encrypt(bit_length, key, iv, encrypted)
}

fn aes_cbc_encrypt(bit_length: usize, key: &[u8], iv: &[u8], plain: &[u8]) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || iv.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Encryptor::<Aes128>::new(key.into(), iv.into());
    let mut encrypted: Vec<u8> = vec![];
    let chunks = plain.chunks(block_size);
    let need_pad_final = plain.len() % block_size == 0;
    let mut buffer = vec![0; block_size];
    for chunk in chunks {
        buffer.fill(0u8);
        let buf = buffer.as_mut_slice();
        // buffer.extend(repeat(0u8).take(block_len)); // [0u8; block_len];
        buf[..chunk.len()].copy_from_slice(chunk);
        if chunk.len() < block_size {
            // padding PKCS7
            let pad = (block_size - chunk.len()) as u8;
            buf[chunk.len()..block_size].iter_mut().for_each(|item| {
                *item = pad;
            });
        }
        cipher.encrypt_block_mut(buf.into());
        encrypted.extend_from_slice(&buf);
    }
    // need to pad final block
    if need_pad_final {
        buffer.fill(block_size as u8);
        let buffer = buffer.as_mut_slice();
        cipher.encrypt_block_mut(buffer.into());
        encrypted.extend_from_slice(buffer);
    }
    Ok(encrypted)
}

fn aes_cbc_decrypt(
    bit_length: usize,
    key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || iv.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key = key.as_ref();
    let iv = iv.as_ref();
    let mut cipher = Decryptor::<Aes128>::new(key.into(), iv.into());
    let mut decrypted: Vec<u8> = vec![];
    let chunks = encrypted.chunks(block_size);

    for chunk in chunks {
        let buffer = &mut *chunk.to_vec();
        cipher.decrypt_block_mut(buffer.into());
        decrypted.extend_from_slice(buffer);
    }

    // Remove PKCS7 padding
    info!("Before unpadding decrypted: {:?}", decrypted);
    if let Some(&pad) = decrypted.last() {
        let pad_len = pad as usize;
        let len = decrypted.len();
        if pad_len <= block_size && pad_len <= len {
            decrypted.truncate(len - pad_len);
        }
    }
    Ok(decrypted)
}

fn aes_gcm_encrypt(
    bit_length: usize,
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    plain: &[u8],
) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || nonce.len() != block_size * 3 / 4 || aad.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let nonce: &GenericArray<u8, U12> = Nonce::from_slice(nonce);
    let cipher = Aes128Gcm::new(key.into());
    let mut encrypted: Vec<u8> = vec![];
    let mut buffer = vec![0u8; plain.len()];
    buffer[..plain.len()].copy_from_slice(plain);

    // let chunks = plain.chunks(block_size);
    // for chunk in chunks {
    //     buffer.fill(0u8);
    //     // let mut buff = buffer.as_mut_slice();
    //     // let payload = Payload {
    //     //     msg: chunk,
    //     //     aad: aad,
    //     // };
    //     // let buff = cipher.e(nonce, payload).expect("Can not encrypt");
    //     buffer[..chunk.len()].copy_from_slice(chunk);
    //     cipher
    //         .encrypt_in_place(nonce, b"", &mut buffer)
    //         .map_err(|e| anyhow!(e))?;
    //     encrypted.extend(&buffer);
    // }
    cipher
        .encrypt_in_place(nonce, aad, &mut buffer)
        .map_err(|e| anyhow!(e))?;
    encrypted.extend_from_slice(&buffer);
    Ok(encrypted)
}

fn aes_gcm_decrypt(
    bit_length: usize,
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    encrypted: &[u8],
) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || nonce.len() != block_size * 3 / 4 || aad.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let nonce: &GenericArray<u8, U12> = Nonce::from_slice(nonce);
    let cipher = Aes128Gcm::new(key.into());
    let mut decrypted: Vec<u8> = vec![];
    let mut buffer = vec![0u8; encrypted.len()];
    buffer[..encrypted.len()].copy_from_slice(encrypted);
    cipher
        .decrypt_in_place(nonce, aad, &mut buffer)
        .map_err(|e| anyhow!(e))?;
    decrypted.extend_from_slice(&buffer);
    Ok(decrypted)
}

#[cfg(test)]
mod tests {

    #![allow(clippy::too_many_arguments, unused_variables, dead_code)]
    use super::*;
    use crate::common::to_hex;
    use log::info;
    use rand_core::{OsRng, RngCore};
    use utils::log::configuration::init_logger;

    #[test]
    fn test_aes_ctr() {
        init_logger();
        let mut rand_bytes = vec![0u8; 32];
        OsRng.fill_bytes(rand_bytes.as_mut_slice());
        let plain = format!("hello world hello world hello world: {}", rand_bytes.len());
        let key = rand_bytes[0..16].to_vec();
        let iv = rand_bytes[16..32].to_vec();
        info!(
            "Key: {:?}, IV: {:?}",
            to_hex(key.as_ref()),
            to_hex(iv.as_ref())
        );
        info!("Plain: {:?} {}", to_hex(plain.as_ref()), plain.len());
        let encrypted = aes_ctr_encrypt(128, key.as_ref(), iv.as_ref(), plain.as_ref());
        let encrypted = encrypted.unwrap();
        info!(
            "Encrypted: {:?}, len:{}",
            to_hex(encrypted.as_ref()),
            encrypted.len()
        );
        let decrypted = aes_ctr_decrypt(128, key.as_ref(), iv.as_ref(), encrypted.as_slice());
        let decrypted = decrypted.unwrap();
        info!(
            "Decrypted: {:?}, len:{}",
            to_hex(decrypted.as_ref()),
            decrypted.len()
        );
    }

    #[test]
    fn test_aes_cbc() {
        init_logger();
        let mut rand_bytes = vec![0u8; 32];
        OsRng.fill_bytes(rand_bytes.as_mut_slice());
        let plain = format!("hello world hello world hello: {}", rand_bytes.len());
        let key_len = 128;
        let key = rand_bytes[0..16].to_vec();
        let iv = rand_bytes[16..32].to_vec();
        info!(
            "Key: {:?}, IV: {:?}",
            to_hex(key.as_ref()),
            to_hex(iv.as_ref())
        );
        info!("Plain: {:?} {}", to_hex(plain.as_ref()), plain.len());
        let encrypted = aes_cbc_encrypt(key_len, key.as_ref(), iv.as_ref(), plain.as_ref());
        let encrypted = encrypted.unwrap();
        info!(
            "Encrypted: {:?}, len:{}",
            to_hex(encrypted.as_ref()),
            encrypted.len()
        );
        let decrypted = aes_cbc_decrypt(key_len, key.as_ref(), iv.as_ref(), encrypted.as_ref());
        let decrypted = decrypted.unwrap();
        info!(
            "Decrypted: {:?}, len:{}",
            to_hex(decrypted.as_ref()),
            decrypted.len()
        );
    }

    #[test]
    fn test_aes_gcm() {
        init_logger();
        let mut rand_bytes = vec![0u8; 48];
        OsRng.fill_bytes(rand_bytes.as_mut_slice());
        let plain = format!("hello world hello world hello: {}", rand_bytes.len());
        let key_len = 128;
        let key = rand_bytes[0..16].to_vec();
        let iv = rand_bytes[16..28].to_vec();
        let aad = rand_bytes[28..44].to_vec();
        info!(
            "Key: {:?}, IV: {:?}",
            to_hex(key.as_ref()),
            to_hex(iv.as_ref())
        );
        info!("Plain: {:?} {}", to_hex(plain.as_ref()), plain.len());
        let encrypted = aes_gcm_encrypt(
            key_len,
            key.as_ref(),
            iv.as_ref(),
            aad.as_ref(),
            plain.as_ref(),
        );
        let encrypted = encrypted.unwrap();
        info!(
            "Encrypted: {:?}, len:{}",
            to_hex(encrypted.as_ref()),
            encrypted.len()
        );
        let decrypted = aes_gcm_decrypt(
            key_len,
            key.as_ref(),
            iv.as_ref(),
            aad.as_ref(),
            encrypted.as_ref(),
        );
        let decrypted = decrypted.unwrap();
        info!(
            "Decrypted: {:?}, len:{}",
            to_hex(decrypted.as_ref()),
            decrypted.len()
        );
    }
}
