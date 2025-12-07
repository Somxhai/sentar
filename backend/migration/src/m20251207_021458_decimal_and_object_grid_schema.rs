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
                    .table(EventObjectPosition::Table)
                    .if_not_exists()
                    .col(uuid(EventObjectPosition::Id).primary_key())
                    .col(uuid(EventObjectPosition::EventObjectId))
                    .col(double(EventObjectPosition::PositionX).default(0))
                    .col(double(EventObjectPosition::PositionY).default(0))
                    .col(double(EventObjectPosition::Rotation).default(0))
                    .col(
                        timestamp_with_time_zone(EventObjectPosition::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(EventObjectPosition::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event_object_position-event_object_id")
                            .from(
                                EventObjectPosition::Table,
                                EventObjectPosition::EventObjectId,
                            )
                            .to(EventObject::Table, EventObject::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-event_object_position-event_object_id")
                    .table(EventObjectPosition::Table)
                    .col(EventObjectPosition::EventObjectId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(EventObjectPosition::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventObjectPosition {
    Table,
    Id,
    EventObjectId,
    PositionX,
    PositionY,
    Rotation,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum EventObject {
    Table,
    Id,
}
