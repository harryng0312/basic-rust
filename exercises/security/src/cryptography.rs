use aes::Aes128;
use aes_gcm_stream::{Aes128GcmStreamDecryptor, Aes128GcmStreamEncryptor};
use anyhow::anyhow;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, StreamCipher};
use cbc::{Decryptor, Encryptor};
use cipher::consts::U12;
use ctr::Ctr128BE;
use tracing::info;
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
    if key.len() != block_size || nonce.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key: [u8; 16] = key.try_into()?;
    let mut cipher = Aes128GcmStreamEncryptor::new(key, nonce);
    let mut encrypted: Vec<u8> = vec![];
    cipher.init_adata(aad);
    for chunk in plain.chunks(block_size) {
        encrypted.extend(cipher.update(chunk));
    }
    let (enc_data, tag) = cipher.finalize();
    encrypted.extend(enc_data);
    encrypted.extend(tag);
    Ok(encrypted)
}

fn aes_gcm_decrypt(
    bit_length: usize,
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    encrypted_tag: &[u8],
) -> AppResult<Vec<u8>> {
    let block_size: usize = bit_length / 8;
    if key.len() != block_size || nonce.len() != block_size {
        return Err(anyhow!(
            "Can not encrypt with invalid key size:{}",
            key.len()
        ));
    }
    let key: [u8; 16] = key.try_into()?;
    let mut cipher = Aes128GcmStreamDecryptor::new(key, nonce);
    let mut decrypted: Vec<u8> = vec![];
    cipher.init_adata(aad);
    // let encrypted = &encrypted_tag[encrypted_tag.len() - 16..];
    // let tag = &encrypted_tag[..encrypted_tag.len() - 16];
    let encrypted = encrypted_tag;
    for chunk in encrypted.chunks(block_size) {
        decrypted.extend(cipher.update(chunk));
    }
    let rs = cipher.finalize();
    match rs {
        Ok(rs) => {
            decrypted.extend(rs);
            Ok(decrypted)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_arguments, unused_variables, dead_code)]
    use super::*;
    use crate::common::to_hex;
    use rand_core::{OsRng, RngCore};
    use tracing::info;
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
        let iv = rand_bytes[16..32].to_vec();
        let aad = rand_bytes[32..44].to_vec();
        info!(
            "Key: {:?}, IV: {:?} AAD:{:?}",
            to_hex(key.as_ref()),
            to_hex(iv.as_ref()),
            to_hex(aad.as_ref())
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
