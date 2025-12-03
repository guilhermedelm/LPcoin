use secp256k1::{Secp256k1, SecretKey, PublicKey as Secp256k1PublicKey, Message, ecdsa::Signature as EcdsaSignature};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: Secp256k1PublicKey,
    pub address: [u8; 20],
    pub balance: u64,
    pub nonce: u64,
}

pub struct Signature {
    pub signature: EcdsaSignature,
}

impl Wallet {
    pub fn generate() -> SecretKey {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        secret_key
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();

        let message_struct = Message::from_digest_slice(&hash)
            .expect("Hash tem 32 bytes, sempre válido");
        
        let sig = secp.sign_ecdsa(&message_struct, &self.private_key);
        Signature { signature: sig }
    }

    pub fn public_key(private_key: SecretKey) -> Secp256k1PublicKey {
        let secp = Secp256k1::new();
        Secp256k1PublicKey::from_secret_key(&secp, &private_key)
    }

    pub fn address(public_key: Secp256k1PublicKey) -> [u8; 20] {
        let pub_bytes = public_key.serialize_uncompressed();

        let pub_key_without_prefix = &pub_bytes[1..];

        let mut hasher = Sha256::new();
        hasher.update(pub_key_without_prefix);
        let hash = hasher.finalize();

        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..32]);
        addr
    }

    pub fn create_wallet() -> Wallet {
        let secret_key = Self::generate();
        let pub_key = Self::public_key(secret_key);
        let addr = Self::address(pub_key);
        
        Wallet {
            private_key: secret_key,
            public_key: pub_key,
            address: addr,
            balance: 0,
            nonce: 0,
        }
    }
}