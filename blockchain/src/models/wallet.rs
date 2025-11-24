
use secp256k1::{Secp256k1, SecretKey, PublicKey as Secp256k1PublicKey, Message, ecdsa::Signature as EcdsaSignature};
use sha2::{Sha256, Digest};
//use std::fmt;
use rand::rngs::OsRng;
//use serde::{Serialize,Deserialize};



#[derive( Debug, Clone, PartialEq, Eq)]  
pub struct Wallet{
    private_key:SecretKey,
    pub public_key:Secp256k1PublicKey,
    address: [u8; 20],
    balance: u64,
    nonce: u64,

}

pub struct Signature{
    signature : EcdsaSignature
}

//Métodos da Private Key
impl Wallet{
    pub fn generate() -> SecretKey{
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        secret_key
        //self.private_key = secret_key;
        
        
    }

    pub fn  sign(&self,message: &[u8]) -> Signature{
        let secp = Secp256k1::new();

        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();

        let message = Message::from_digest_slice(&hash)
            .expect("Hash tem 32 bytes, sempre válido");
        
        let sig = secp.sign_ecdsa(&message, &self.private_key);
        Signature { signature: sig }
    }



}

//Métodos da public Key
impl Wallet{

    //gerar chave pública
    pub fn public_key(private_key:SecretKey) -> Secp256k1PublicKey{
    let secp = Secp256k1::new();
    let pub_key = Secp256k1PublicKey::from_secret_key(&secp,&private_key);
    pub_key
    }
    
    //converte chave pública da rede de bytes para string
    

    //criar um endereço de carteira à partir da chave pública 
        pub fn address(public_key:Secp256k1PublicKey) -> [u8; 20]{
        // Serializa chave pública (formato não comprimido: 65 bytes)
        let pub_bytes = public_key.serialize_uncompressed();
        
        // Remove o primeiro byte (0x04 prefix)
        let pub_key_without_prefix = &pub_bytes[1..];
        
        // Hash SHA256 (em produção Ethereum usa Keccak256)
        let mut hasher = Sha256::new();
        hasher.update(pub_key_without_prefix);
        let hash = hasher.finalize();
        
        // Pega os últimos 20 bytes (Ethereum style)
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..32]);
        addr
    }
}

//Método para Criar carteira
impl Wallet{
    pub fn create_wallet() -> Wallet{
        //primeiro criar chave pública
        let mut secret_key = Self::generate();
        let mut pub_key = Self::public_key(secret_key);
        let mut addr = Self::address(pub_key);
        let new = Wallet{
            private_key:secret_key,
            public_key:pub_key,
            address:addr,
            balance:0,
            nonce:0,
        };
        new
    }
}

