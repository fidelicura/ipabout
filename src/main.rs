mod drawer;
mod parser;
mod data;

use data::{AppArgs, SearcherType};
use clap::Parser;
use parser::{get_global_ip_response, get_local_ip_response};



#[tokio::main]
async fn main() {
    let args = AppArgs::parse();

    match args.searcher_type {
        SearcherType::Global(cmd) => {
            get_global_ip_response(cmd).await;
        },
        SearcherType::Local(_) => {
            get_local_ip_response();
        },
    }
}
