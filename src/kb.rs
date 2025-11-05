use crate::{error::Result, model::Definition};
use log::trace;
use mongodb::{
    Client, Collection,
    bson::{Document, to_document},
};

pub struct Database {
    collection: Collection<Document>,
}

impl Database {
    const DATABASE: &'static str = "kb";
    const COLLECTION: &'static str = "data";

    pub async fn try_new(url: &str) -> Result<Self> {
        trace!("kb::Database::try_new(url: &str) -> Result<Self>");
        let client = Client::with_uri_str(url).await?;
        let database = client.database(Database::DATABASE);
        let collection = database.collection::<Document>(Database::COLLECTION);

        Ok(Self { collection })
    }

    pub async fn insert(&self, definition: &Definition) -> Result<()> {
        trace!("kb::Database::insert(&self, definition: &Definition) -> Result<()>");
        let doc = to_document(definition)?;
        let _ = self.collection.insert_one(doc, None).await?;
        Ok(())
    }
}
