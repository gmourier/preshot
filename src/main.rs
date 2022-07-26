mod options;

use clap::Parser;
use options::{Command, Options};

use uuid::Uuid;

use hmac::{Hmac, Mac};
use sha2::{Sha256};

use prettytable::{format, Cell, Row, Table};

fn main() {
    let opt = Options::from_args();
    match opt.command {
        Command::DiscoverKeys { master_key, uids } => {
            discover_keys(master_key, uids);
        },
        Command::GenerateKeys { master_key, count } => {
            generate_keys(master_key, count);
        }
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
    let mut keys: Vec<APIKey> = Vec::new();

    for uid in uids {
        let mut mac = Hmac::<Sha256>::new_from_slice(master_key.as_bytes()).unwrap();
        mac.update(uid.as_bytes());

        let result = mac.finalize();

        keys.push(APIKey {
            uid: uid,
            key: format!("{:x}", result.into_bytes()),
        });
    }

    print_keys(&keys);
}

fn generate_keys(master_key: String, mut count: usize) -> () {
    let mut keys: Vec<APIKey> = Vec::new();

    while count > 0 {
        let uuid = Uuid::new_v4();

        let mut mac = Hmac::<Sha256>::new_from_slice(master_key.as_bytes()).unwrap();
        mac.update(uuid.as_bytes());

        let result = mac.finalize();

        keys.push(APIKey {
            uid: uuid.to_string(),
            key: format!("{:x}", result.into_bytes()),
        });

        count -= 1;
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

