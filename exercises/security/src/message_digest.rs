use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use sha2::{Digest, Sha256};
use utils::error::app_error::AppResult;

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut sha256 = Sha256::default();
    for chunk in data.chunks(32) {
        sha256.update(chunk);
    }
    sha256.finalize().into()
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> AppResult<[u8; 32]> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(key)?;
    for chunk in data.chunks(32) {
        hmac.update(chunk);
    }
    Ok(hmac.finalize().into_bytes().into())
}

fn pbkdf2_hmac_sha256(
    password: &[u8],
    salt: &[u8],
    rounds: u32,
    out_len: usize,
) -> AppResult<Vec<u8>> {
    let mut buffer = vec![0u8; out_len];
    pbkdf2::<Hmac<Sha256>>(password, salt, rounds, &mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use crate::common::{gen_random_byte_arr, to_base64, to_hex};
    use crate::message_digest::{hmac_sha256, pbkdf2_hmac_sha256, sha256};
    use log::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_sha256() {
        init_logger();
        let str_data = "Đây là dữ liệu cần tests thử".as_bytes();
        let msg_digest = sha256(str_data);
        info!("Sha256 {:?}", to_hex(msg_digest.as_ref()));
    }

    #[test]
    fn test_hmac_sha256() {
        init_logger();
        let data = "Đây là dữ liệu cần được tests thử".as_bytes();
        let secret_key_data = "secret key".as_bytes();
        let hmac = hmac_sha256(secret_key_data, data).unwrap();
        info!(
            "HmacSHA256:{:?} {:?}",
            to_hex(hmac.as_ref()),
            to_base64(hmac.as_ref())
        );
        // OzRlpfvM9jvO1lh1zEH5bgZ8Jr0nIwQ5ONk6ZHBB86g=
    }

    #[test]
    fn test_pbkdf2() {
        init_logger();
        let bit_len: usize = 256usize;
        // const bit_len: usize = bit_len_inp;
        let password = b"P@55w0rd".as_slice();
        let mut salt: Vec<u8> = vec![0u8; bit_len / 8];
        let rounds: u32 = 1_000;
        let out_len: usize = 1_024;
        gen_random_byte_arr(&mut salt).unwrap();
        let salt = salt.as_slice();
        let out_data = pbkdf2_hmac_sha256(password, salt, rounds, out_len).unwrap();
        let out_data = out_data.as_slice();
        info!("Before:Salt:{}", to_hex(salt));
        info!("After :Salt:{}", to_hex(salt));
        info!("After :Key :{}", to_hex(password));
        info!("Hash  :{:?}", to_hex(out_data));
    }
}
