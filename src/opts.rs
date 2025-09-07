use anyhow::Result;
use clap::Parser;

fn stov(v: &str) -> Result<Vec<String>> {
    Ok(v.split(',').map(|f| f.trim().to_string()).collect())
}

#[derive(Debug, Parser)]
pub struct CliOpts {
    /// The coins that should be watched in a list like BTC-USDC,SOL-USDC
    #[arg(short = 'w', long = "watching", default_value = "SOL-USDC", value_delimiter = ',')]
    pub watching: Vec<String>,
}
