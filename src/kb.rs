use crate::{error::Result, model::Definition};
use mongodb::{
    Client, Collection,
    bson::{Document, to_document},
};

pub struct Database {
    collection: Collection<Document>,
}

impl Database {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = Client::with_uri_str(url).await?;
        let database = client.database("kb");
        let collection = database.collection::<Document>("data");

        Ok(Self { collection })
    }

    pub async fn insert(&self, definition: &Definition) -> Result<()> {
        let doc = to_document(definition)?;
        let result = self.collection.insert_one(doc, None).await?;
        println!("{result:?}");
        Ok(())
    }
}
