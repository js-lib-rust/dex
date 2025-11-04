mod dex;
mod error;
mod kb;
mod model;
mod util;
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
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
        let _ = kb.insert(&definition).await?;
    }

    Ok(())
}
