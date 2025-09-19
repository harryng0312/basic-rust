#[cfg(test)]
mod test {
    use crate::common::{from_base64, to_base64};
    use openssl::bn::BigNum;
    use openssl::derive::Deriver;
    use openssl::dh::Dh;
    use openssl::ec::{EcGroup, EcKey};
    use openssl::nid::Nid;
    use openssl::pkey::{Id, PKey};
    use tracing::info;
    use utils::log::configuration::init_logger;

    #[test]
    /// Suppose there are 02 people: Alice & Bob
    ///
    fn test_ecdh() {
        init_logger();
        // create EC group,
        let group = EcGroup::from_curve_name(Nid::SECP256K1).expect("Failed to create EC group");
        // Alice creates key_pair and publishes the public_key
        let mut a_priv_key: String;
        let mut a_pub_key: String;
        {
            let ec_key = EcKey::generate(&group).expect("Can not generate EC Key");
            let priv_key_der = ec_key.private_key_to_der().unwrap();
            let pub_key_der = ec_key.public_key_to_der().unwrap();
            a_priv_key = to_base64(priv_key_der.as_slice()).unwrap();
            a_pub_key = to_base64(pub_key_der.as_slice()).unwrap();
            info!("Alice's private key: {}", a_priv_key);
            info!("Alice's public key: {}", a_pub_key);
        }

        // Bob creates key_pair and publishes the public_key
        let b_priv_key: String;
        let b_pub_key: String;
        {
            let ec_key = EcKey::generate(&group).expect("Can not generate EC Key");
            let priv_key = ec_key.private_key_to_der().unwrap();
            let pub_key = ec_key.public_key_to_der().unwrap();
            let priv_key_str = to_base64(priv_key.as_slice()).unwrap();
            b_priv_key = to_base64(&priv_key).unwrap();
            b_pub_key = to_base64(&pub_key).unwrap();
            info!("Bob's private key: {}", priv_key_str);
            info!("Bob's public key: {}", b_pub_key);
        }

        // Alice side: Bob sends public_key to Alice. Alice generates the shared secret
        {
            let a_priv_key_der = from_base64(&b_priv_key).unwrap();
            let b_pub_key_der = from_base64(&a_pub_key).unwrap();
            let priv_key = PKey::private_key_from_der(a_priv_key_der.as_slice()).unwrap();
            let partner_pub_key = PKey::public_key_from_der(b_pub_key_der.as_slice()).unwrap();
            let mut deriver = Deriver::new(&priv_key).unwrap();
            deriver.set_peer(&partner_pub_key).unwrap();
            let shared_secret = deriver.derive_to_vec().unwrap();
            info!(
                "Alice's shared secret: {}, len: {}",
                to_base64(shared_secret.as_slice()).unwrap(),
                shared_secret.len()
            );
        }

        // Bob side: Alice sends public_key to Bob. Bob generates the shared secret
        {
            let b_priv_key_der = from_base64(&b_priv_key).unwrap();
            let a_pub_key_der = from_base64(&a_pub_key).unwrap();
            let priv_key = PKey::private_key_from_der(b_priv_key_der.as_slice()).unwrap();
            let partner_pub_key = PKey::public_key_from_der(a_pub_key_der.as_slice()).unwrap();
            let mut deriver = Deriver::new(&priv_key).unwrap();
            deriver.set_peer(&partner_pub_key).unwrap();
            let shared_secret = deriver.derive_to_vec().unwrap();
            info!(
                "Bob's shared secret: {}, len: {}",
                to_base64(shared_secret.as_slice()).unwrap(),
                shared_secret.len()
            );
        }
    }

    #[test]
    /// Suppose there are 03 people: Alice, Bob & Charles
    ///
    fn test_dh_3parties() {
        init_logger();
        // Create DH params
        let dh = Dh::get_2048_256().unwrap();
        let p = dh.prime_p().to_owned().unwrap();
        let g = dh.generator().to_owned().unwrap();

        // Alice create key_pair
        let alice_dh = Dh::from_pqg(p.to_owned().unwrap(), None, g.to_owned().unwrap()).unwrap();
        let alice_key = PKey::from_dh(alice_dh.generate_key().unwrap()).unwrap();

        // Bob create key_pair
        let bob_dh = Dh::from_pqg(p.to_owned().unwrap(), None, g.to_owned().unwrap()).unwrap();
        let bob_key = PKey::from_dh(bob_dh.generate_key().unwrap()).unwrap();

        // Carol create key_pair
        let carol_dh = Dh::from_pqg(p.to_owned().unwrap(), None, g.to_owned().unwrap()).unwrap();
        let carol_key = PKey::from_dh(carol_dh.generate_key().unwrap()).unwrap();

        // Alice calcualtes shared_key with Bob
        let mut deriver_alice_bob = Deriver::new(&alice_key).unwrap();
        deriver_alice_bob.set_peer(&bob_key).unwrap();
        let shared_ab = deriver_alice_bob.derive_to_vec().unwrap();

        // Bob calcualtes shared_key with Carol
        let mut deriver_bob_carol = Deriver::new(&bob_key).unwrap();
        deriver_bob_carol.set_peer(&carol_key).unwrap();
        // let shared_bc = deriver_bob_carol.derive_to_vec().unwrap();
        // deriver_bob_carol.set_peer_ex(&carol_key, true).unwrap();
        let shared_bc = deriver_bob_carol.derive_to_vec().unwrap();

        // Carol calcualtes shared_key with Alice
        let mut deriver_carol_alice = Deriver::new(&carol_key).unwrap();
        deriver_carol_alice.set_peer(&alice_key).unwrap();
        let shared_ca = deriver_carol_alice.derive_to_vec().unwrap();

        // Calcualte final shared_key
        // TODO: re-check
        let shared_abc: Vec<u8> = shared_ab
            .iter()
            .zip(shared_bc.iter())
            .map(|(x, y)| x ^ y)
            .collect();
        let final_shared_secret: Vec<u8> = shared_abc
            .iter()
            .zip(shared_ca.iter())
            .map(|(x, y)| x ^ y)
            .collect();
        info!(
            "Shared secret: {:?}",
            to_base64(final_shared_secret.as_slice())
        );

        // let mut deriver_a_final: Deriver = Deriver::new(&alice_key).unwrap();
        // let shared_bc_ex_pub_key = PKey::public_key_from_raw_bytes(shared_bc.as_slice(), Id::DH).unwrap();
        // deriver_a_final.set_peer(&shared_bc_ex_pub_key).unwrap();
        // let final_shared = deriver_a_final.derive_to_vec().unwrap();
        // info!("Shared secret: {:?}", to_base64(final_shared.as_slice()));
    }
}
