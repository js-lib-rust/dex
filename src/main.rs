mod dex;
mod error;
mod kb;
mod logger;
mod model;
mod util;
use clap::Parser;
use log::{info, trace};

use crate::error::Result;

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        long,
        default_value = "off",
        help = "logging level: off, error, warn, info, debug, trace"
    )]
    log_level: String,

    #[arg(
        long,
        help = "logging file path -- if not specified print logs to console"
    )]
    log_file: Option<String>,

    #[arg(
        long,
        help = "do not insert records into knowledge database"
    )]
    dry: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    logger::init(&args.log_level, &args.log_file);
    trace!("main()");

    let dex_url = "mysql://root:mami1964@localhost:3306/dex";
    let mut dex = dex::Database::try_new(dex_url)?;

    let kb_url = "mongodb://localhost:27017";
    let kb = kb::Database::try_new(kb_url).await?;

    let mut base_id = 0;
    loop {
        let Some((id, word)) = dex.next_word(base_id) else {
            break;
        };
        let definition = dex.query(id, word)?;
        base_id = id;
        if !args.dry {
            let _ = kb.insert(&definition).await?;
        }
    }

    info!("DEX import successfully ended");
    Ok(())
}
