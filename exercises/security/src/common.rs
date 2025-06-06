use base64_stream::{FromBase64Reader, ToBase64Reader};
use openssl::rand::rand_bytes;
use rand::Rng;
use std::error::Error;
use std::io::{Cursor, Read};

use rand_core::{OsRng, RngCore};

pub fn to_base64(data: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut reader = ToBase64Reader::new(Cursor::new(data));
    // let mut reader = ToBase64Reader::new(data);
    let mut buff = String::new();
    let _ = reader.read_to_string(&mut buff)?;
    Ok(buff)
}

pub fn from_base64(b64_str: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buff: Vec<u8> = vec![];
    let mut reader = FromBase64Reader::new(Cursor::new(b64_str));
    let _ = reader.read_to_end(&mut buff);
    Ok(buff)
}

pub fn gen_random_byte_arr(rand_v: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    // let mut thread_rng = rand::thread_rng();
    // let mut rand_v: Vec<u8>= vec![0u8; arr_len];
    // thread_rng.fill(&mut *rand_v);
    rand_bytes(&mut *rand_v).map_err(|e| Box::new(e) as Box<dyn Error>)
    // Ok(())
}

pub fn gen_secured_random_byte_arr(arr_inp: &mut [u8]) -> Result<(), Box<dyn Error>> {
    // let mut rand_v: Vec<u8> = vec![0u8; arr_len];
    OsRng.fill_bytes(arr_inp);
    // Ok(rand_v)
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::common::{from_base64, gen_random_byte_arr, to_base64};
    use log::info;
    // use log::kv::ToValue;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_gen_random_byte_arr() {
        init_logger();
        let mut bin_data: Vec<u8> = vec![0u8; 32];
        if let Ok(_) = gen_random_byte_arr(&mut bin_data) {
            if let Ok(b64_data) = to_base64(bin_data.as_slice()) {
                info!("b64 data:{}", b64_data);
            } else {
                info!("Error converting data to base64");
            }
        } else {
            info!("Error getting random bytes");
        }
    }

    #[test]
    fn test_from_base64() {
        init_logger();
        let b64_data = "oD77OgkcOQuHKiTq3l0W5j8k3n2CNaORgj/gNsAzbqc=".to_string();
        let bin_data = from_base64(&b64_data).unwrap();
        info!("bin data:{:?}", bin_data.as_slice());
    }
}
