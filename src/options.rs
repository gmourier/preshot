use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Know the value of the key field of a Meilisearch API key before its generation.")]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Generate key field value
    Generate {
        /// Meilisearch master key
        master_key: String,
        /// API key uids
        uids: Vec<String>,
    },
}
