use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "Know the value of the key field of a Meilisearch API key before its generation.")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Generate key field value
    Generate {
        /// Meilisearch master key
        master_key: String,
        /// API key uids
        uids: Vec<String>
    }
}