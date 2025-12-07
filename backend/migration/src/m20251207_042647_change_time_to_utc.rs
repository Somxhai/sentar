use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(EventObjectPosition::Table)
                    .modify_column(
                        ColumnDef::new(EventObjectPosition::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(EventObjectPosition::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Reservation::Table)
                    .modify_column(
                        ColumnDef::new(Reservation::ExpiresAt)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .modify_column(
                        ColumnDef::new(Event::StartsAt)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(Event::EndsAt)
                            .timestamp()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(EventObjectPosition::Table)
                    .modify_column(
                        ColumnDef::new(EventObjectPosition::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(EventObjectPosition::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Reservation::Table)
                    .modify_column(
                        ColumnDef::new(Reservation::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .modify_column(
                        ColumnDef::new(Event::StartsAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .modify_column(
                        ColumnDef::new(Event::EndsAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum EventObjectPosition {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Reservation {
    Table,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    StartsAt,
    EndsAt,
}
