use clap::{Args, Subcommand, Parser};
use serde::Deserialize;



#[derive(Parser)]
pub struct AppArgs {
    
    #[clap(subcommand)]
    /// Searcher type
    pub searcher_type: SearcherType, 
}


#[derive(Subcommand)]
pub enum SearcherType {
    
    /// Global IP searcher
    Global(SearchGlobalCommand),

    /// Local IP searcher
    Local(SearchLocalCommand),
}


#[derive(Args)]
pub struct SearchGlobalCommand {

    /// Global IP address itself
    pub global_ip_addr: String,
}


#[derive(Args)]
pub struct SearchLocalCommand {}


#[derive(Deserialize)]
pub struct SearchGlobalResult {
    pub ip: String,
    pub continent_code: String,
    pub network: String,
    pub country_name: String,
    pub city: String,
    pub region: String,
    pub asn: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
}
