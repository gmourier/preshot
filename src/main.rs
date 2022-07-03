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
        Command::DiscoverKeys { master_key, uids } => {
            discover_keys(master_key, uids);
        },
        Command::GenerateUuids { count } => {
            generate_uuids(count);
        },
    }
}

struct APIKey {
    uid: String,
    key: String,
}

fn discover_keys(master_key: String, uids: Vec<String>) -> () {
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

    table.add_row(Row::new(vec![Cell::new("uid"), Cell::new("🔑 key")]));

    for api_key in keys {
        table.add_row(Row::new(vec![
            Cell::new(&api_key.uid.to_string()),
            Cell::new(&api_key.key.to_string()),
        ]));
    }

    table.printstd();
}

/// Generate UUIDv4s
fn generate_uuids(mut count: usize) -> () {
    let mut uuids: Vec<Uuid> = Vec::new();

    while count > 0 {
        let uuid = Uuid::new_v4();
        uuids.push(uuid);
        count -= 1;
    }

    print_uuids(&uuids);
}

fn print_uuids(uuids: &Vec<Uuid>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![Cell::new("uid")]));

    for uuid in uuids {
        table.add_row(Row::new(vec![
            Cell::new(&uuid.to_string())
        ]));
    }

    table.printstd();
}