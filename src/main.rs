use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;

use starkware_indexer::{
    db::{Database, DatabaseError},
    indexer::{Indexer, IndexerError},
    subscription::Subscription,
    types::*,
};

#[tokio::main]
async fn main() -> Result<(), IndexerError> {
    let database = Arc::new(Database::new()?);

    let (mut indexer, mut subscription) = Indexer::new(database).await?;
    let (tx, mut rx) = mpsc::channel(1);
    subscription.subscribe(tx);

    // Start the indexer task in the background.
    let indexer_handle = task::spawn(async move {
        while let Some(block) = rx.recv().await {
            if let Err(e) = indexer.index(block).await {
                println!("Error indexing block: {}", e);
            }
        }
    });

    // Wait for the indexer to finish.
    indexer_handle.await?;
    Ok(())
}
