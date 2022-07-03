mod options;

use clap::Parser;
use options::{Command, Options};

use uuid::Uuid;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use prettytable::{format, Cell, Row, Table};

fn main() {
    let opt = Options::from_args();
    match opt.command {
        Command::Generate { master_key, uids } => {
            generate_keys(master_key, uids);
        }
    }
}

struct APIKey {
    uid: String,
    key: String,
}

fn generate_keys(master_key: String, uids: Vec<String>) -> () {
    let master_key_sha = Sha256::digest(master_key.as_bytes());

    let mut keys: Vec<APIKey> = Vec::new();

    for uid in uids {
        let uuid = Uuid::parse_str(&uid).unwrap();

        let mut mac = Hmac::<Sha256>::new_from_slice(master_key_sha.as_slice()).unwrap();
        mac.update(uuid.as_bytes());

        let result = mac.finalize();
        let base64 = base64::encode_config(result.into_bytes(), base64::URL_SAFE_NO_PAD);

        keys.push(APIKey {
            uid: uid,
            key: base64,
        });
    }

    print_keys(&keys);
}

fn print_keys(keys: &Vec<APIKey>) -> () {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![Cell::new("uid"), Cell::new("🔑 key")
]));

    for api_key in keys {
        table.add_row(Row::new(vec![
            Cell::new(&api_key.uid.to_string()),
            Cell::new(&api_key.key.to_string()),
        ]));
    }

    table.printstd();
}
