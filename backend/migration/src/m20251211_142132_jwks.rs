use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Jwks::Table)
                    .if_not_exists()
                    .col(string(Jwks::Id).primary_key())
                    .col(string(Jwks::PublicKey))
                    .col(string(Jwks::PrivateKey))
                    .col(timestamp(Jwks::CreatedAt))
                    .col(timestamp_null(Jwks::ExpiresAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Jwks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Jwks {
    Table,
    Id,
    PublicKey,
    PrivateKey,
    CreatedAt,
    ExpiresAt,
}
