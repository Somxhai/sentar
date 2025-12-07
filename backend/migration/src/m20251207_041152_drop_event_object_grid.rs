use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(EventObjectGrid::Table).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(EventObjectGrid::Table)
                    .if_not_exists()
                    .col(uuid(EventObjectGrid::Id).primary_key())
                    .col(uuid(EventObjectGrid::EventId).not_null())
                    .col(json_binary_null(EventObjectGrid::Grid))
                    .col(
                        timestamp(EventObjectGrid::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(EventObjectGrid::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_object_grid_event")
                            .from(EventObjectGrid::Table, EventObjectGrid::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum EventObjectGrid {
    Table,
    Id,
    EventId,
    Grid,
    CreatedAt,
    UpdatedAt,
}
