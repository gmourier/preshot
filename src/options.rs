use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Know the value of the key field of a Meilisearch API key before its generation.")]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Discover key fields value for a list of UIDs and a master key
    DiscoverKeys {
        /// Meilisearch master key
        master_key: String,
        /// API key uids
        uids: Vec<String>,
    },
    /// Generate (uid, key) tuples
    GenerateKeys {
        /// Meilisearch master key
        master_key: String,
        /// Number of (uid, key) tuple to generate
        #[clap(default_value = "1")]
        count: usize,
    },
    /// Generate uuid(s) V4
    GenerateUuids {
        /// Number of uuids V4 to generate
        #[clap(default_value = "1")]
        count: usize,
    }
}
