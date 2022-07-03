mod options;

use options::{ Command, Options };
use structopt::StructOpt;

use uuid::Uuid;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

fn main() {
    let opt = Options::from_args();
    match opt.command {
        Command::Generate {
            master_key,
            uid
        } => {
            generate_key(
                master_key,
                uid
            );
        }
    }
}

fn generate_key(master_key: String, uid: String) -> () {
    let uuid = Uuid::parse_str(&uid).unwrap();

    let master_key_sha = Sha256::digest(master_key.as_bytes());
    let mut mac = Hmac::<Sha256>::new_from_slice(master_key_sha.as_slice()).unwrap();
    mac.update(uuid.as_bytes());

    let result = mac.finalize();
    let base64 = base64::encode_config(result.into_bytes(), base64::URL_SAFE_NO_PAD);

    println!("🔑 {}", base64);
}