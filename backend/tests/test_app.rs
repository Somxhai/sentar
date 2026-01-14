mod common;

use crate::common::container::PgContainer;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

#[tokio::test]
async fn test_container_db() -> eyre::Result<()> {
    let db = PgContainer::create_test_db().await?;
    db.ping().await?;
    Ok(())
}
