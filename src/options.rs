use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate the value of the key field for a Meilisearch API key before its generation from the master-key and its uid.")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Generate `key` value
    Generate {
        /// API key uid
        uid: String,
        /// Meilisearch master key
        master_key: String
    }
}