use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .add_column_if_not_exists(string(Event::Status).default("open").check(
                        Expr::col(Event::Status).is_in(["open", "closed", "completed", "canceled"]),
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .drop_column(Event::Status)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Event {
    Table,
    Status,
}
