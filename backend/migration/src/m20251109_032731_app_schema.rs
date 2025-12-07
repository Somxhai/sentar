use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION update_updated_at_col()
            RETURNS TRIGGER AS $$
            BEGIN
               NEW.updated_at = now();
               RETURN NEW;
            END;
            $$ language 'plpgsql';
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(uuid(Workspace::Id).primary_key())
                    .col(string(Workspace::Name).not_null())
                    .col(string(Workspace::OwnerId).not_null())
                    .col(
                        timestamp(Workspace::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Workspace::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_owner")
                            .from(Workspace::Table, Workspace::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_workspace_updated_at
            BEFORE UPDATE ON "workspace"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(uuid(Event::Id).primary_key())
                    .col(string(Event::Title).not_null())
                    .col(uuid(Event::WorkspaceId).not_null())
                    .col(string_null(Event::Description))
                    .col(json_binary_null(Event::Settings))
                    .col(timestamp_with_time_zone_null(Event::StartsAt))
                    .col(timestamp_with_time_zone_null(Event::EndsAt))
                    .col(
                        timestamp(Event::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Event::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_workspace")
                            .from(Event::Table, Event::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_event_updated_at
            BEFORE UPDATE ON "event"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(Form::Table)
                    .if_not_exists()
                    .col(uuid(Form::Id).primary_key())
                    .col(uuid(Form::EventId).not_null())
                    .col(json_binary_null(Form::Schema))
                    .col(json_null(Form::Settings))
                    .col(string_null(Form::Title))
                    .col(string_null(Form::Description))
                    .col(
                        timestamp(Form::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Form::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_event")
                            .from(Form::Table, Form::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_form_updated_at
            BEFORE UPDATE ON "form"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(Section::Table)
                    .if_not_exists()
                    .col(uuid(Section::Id).primary_key())
                    .col(uuid(Section::EventId).not_null())
                    .col(string(Section::Title).not_null())
                    .col(double(Section::Price).default(0.0).not_null())
                    .col(
                        timestamp(Section::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Section::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_section_event")
                            .from(Section::Table, Section::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_section_updated_at
            BEFORE UPDATE ON "section"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(EventObject::Table)
                    .if_not_exists()
                    .col(uuid(EventObject::Id).primary_key())
                    .col(string(EventObject::ObjectType).not_null())
                    .col(uuid(EventObject::EventId).not_null())
                    .col(uuid_null(EventObject::SectionId))
                    .col(string_null(EventObject::Label))
                    .col(boolean(EventObject::IsEnable).default(true).not_null())
                    .col(string(EventObject::Status).default("available").not_null())
                    .col(
                        timestamp(EventObject::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(EventObject::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_object_event")
                            .from(EventObject::Table, EventObject::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_object_section")
                            .from(EventObject::Table, EventObject::SectionId)
                            .to(Section::Table, Section::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_event_object_updated_at
            BEFORE UPDATE ON "event_object"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

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
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_event_object_grid_updated_at
            BEFORE UPDATE ON "event_object_grid"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reservation::Table)
                    .if_not_exists()
                    .col(uuid(Reservation::Id).primary_key())
                    .col(string(Reservation::UserId).not_null())
                    .col(uuid(Reservation::EventId).not_null())
                    .col(string(Reservation::Status).not_null())
                    .col(double(Reservation::TotalPrice).default(0.0).not_null())
                    .col(timestamp_with_time_zone_null(Reservation::ExpiresAt))
                    .col(
                        timestamp(Reservation::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Reservation::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reservation_user")
                            .from(Reservation::Table, Reservation::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reservation_event")
                            .from(Reservation::Table, Reservation::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_reservation_updated_at
            BEFORE UPDATE ON "reservation"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReservationItem::Table)
                    .if_not_exists()
                    .col(uuid(ReservationItem::Id).primary_key())
                    .col(uuid(ReservationItem::ReservationId).not_null())
                    .col(uuid(ReservationItem::EventObjectId).not_null())
                    .col(
                        double(ReservationItem::PriceAtBooking)
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        timestamp(ReservationItem::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(ReservationItem::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reservation_item_reservation")
                            .from(ReservationItem::Table, ReservationItem::ReservationId)
                            .to(Reservation::Table, Reservation::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reservation_item_event_object")
                            .from(ReservationItem::Table, ReservationItem::EventObjectId)
                            .to(EventObject::Table, EventObject::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_reservation_item_updated_at
            BEFORE UPDATE ON "reservation_item"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReservationItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reservation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventObjectGrid::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventObject::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Section::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Form::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_col() CASCADE")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Workspace {
    Table,
    Id,
    Name,
    OwnerId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Title,
    WorkspaceId,
    Description,
    Settings,
    StartsAt,
    EndsAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
    EventId,
    Schema,
    Settings,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Section {
    Table,
    Id,
    EventId,
    Title,
    Price,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum EventObject {
    Table,
    Id,
    ObjectType,
    EventId,
    SectionId,
    Label,
    IsEnable,
    Status,
    CreatedAt,
    UpdatedAt,
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

#[derive(DeriveIden)]
enum Reservation {
    Table,
    Id,
    UserId,
    EventId,
    Status,
    TotalPrice,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ReservationItem {
    Table,
    Id,
    ReservationId,
    EventObjectId,
    PriceAtBooking,
    CreatedAt,
    UpdatedAt,
}
