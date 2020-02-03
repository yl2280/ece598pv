use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use std::slice;
use bincode::{serialize, deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    input: std::string::String,
    output: std::string::String,
    signature: std::string::String 
}

// impl Transaction {
//     fn signed(&mut self, sig: *str) -> () {
//         self.signature = true
//     }
// }

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    //unimplemented!()
   
    // t.signature = true;

    let t_s = serialize(&t).unwrap();
    let sig = key.sign(&t_s);
    return sig;

}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    //unimplemented!()
    let t_s = serialize(&t).unwrap();
    // let ret = public_key.verify(slice, signature);
    let peer_public_key_bytes = public_key.as_ref();
    let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, peer_public_key_bytes);
    // signature::verify()
    let ret = peer_public_key.verify(&t_s, signature.as_ref());
    // try!(peer_public_key.verify(&t_s, signature.as_ref()));
    if(ret == Ok(())) { return true;}
    else {return false;}

}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> Transaction {
        // Default::default()
        //unimplemented!()
        let input: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

        let output: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
        let s = "";
        let signature = s.to_string();
        // signature = signature.to_string();

        let ret = Transaction{input, output, signature};

        return ret;

    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
