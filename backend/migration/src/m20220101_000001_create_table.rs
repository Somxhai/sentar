use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Create the reusable trigger function for updating timestamps.
        // This will only be used by Workspace and WorkspaceObject now.
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

        // 2. Create tables and apply triggers only where needed.

        // Create User table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key())
                    .col(string(User::Name).not_null())
                    .col(string(User::Email).unique_key().not_null())
                    .col(boolean(User::EmailVerified).default(false).not_null())
                    .col(string_null(User::Image))
                    .col(
                        timestamp(User::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Session table
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(string(Session::Id).primary_key())
                    .col(string(Session::UserId).not_null())
                    .col(string(Session::Token).not_null())
                    .col(timestamp(Session::ExpiresAt).not_null())
                    .col(string_null(Session::IpAddress))
                    .col(string_null(Session::UserAgent))
                    .col(
                        timestamp(Session::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Session::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_user")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Account table
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(string(Account::Id).primary_key())
                    .col(string(Account::UserId).not_null())
                    .col(string(Account::AccountId).not_null())
                    .col(string(Account::ProviderId).not_null())
                    .col(string_null(Account::AccessToken))
                    .col(string_null(Account::RefreshToken))
                    .col(timestamp_null(Account::AccessTokenExpiresAt))
                    .col(timestamp_null(Account::RefreshTokenExpiresAt))
                    .col(string_null(Account::Scope))
                    .col(string_null(Account::IdToken))
                    .col(string_null(Account::Password))
                    .col(
                        timestamp(Account::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Account::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_user")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Verification table
        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(string(Verification::Id).primary_key())
                    .col(string(Verification::Identifier).not_null())
                    .col(string(Verification::Value).not_null())
                    .col(timestamp(Verification::ExpiresAt).not_null())
                    .col(
                        timestamp(Verification::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Verification::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Workspace table with new timestamp columns
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workspace::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Workspace::Title).not_null())
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

        // Apply the trigger to the Workspace table
        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_workspace_updated_at
            BEFORE UPDATE ON "workspace"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        // Create WorkspaceObject table with new timestamp columns
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceObject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkspaceObject::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WorkspaceObject::WorkspaceId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WorkspaceObject::X).float().not_null())
                    .col(ColumnDef::new(WorkspaceObject::Y).float().not_null())
                    .col(ColumnDef::new(WorkspaceObject::Rotation).float().not_null())
                    .col(
                        timestamp(WorkspaceObject::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(WorkspaceObject::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_object_workspace")
                            .from(WorkspaceObject::Table, WorkspaceObject::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Apply the trigger to the WorkspaceObject table
        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_workspace_object_updated_at
            BEFORE UPDATE ON "workspace_object"
            FOR EACH ROW
            EXECUTE PROCEDURE update_updated_at_col();
            "#,
        )
        .await?;

        // Create Seat table
        manager
            .create_table(
                Table::create()
                    .table(Seat::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Seat::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Seat::ObjectId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Seat::ReservedForUserId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_seat_object")
                            .from(Seat::Table, Seat::ObjectId)
                            .to(WorkspaceObject::Table, WorkspaceObject::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_seat_user")
                            .from(Seat::Table, Seat::ReservedForUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Drop tables in reverse order of creation
        manager
            .drop_table(Table::drop().table(Seat::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WorkspaceObject::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Verification::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Finally, drop the trigger function.
        // Using CASCADE will automatically drop the triggers that depend on it.
        db.execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_col() CASCADE")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    EmailVerified,
    Image,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Workspace {
    Table,
    Id,
    Title,
    OwnerId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
    IpAddress,
    UserAgent,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    UserId,
    AccountId,
    ProviderId,
    AccessToken,
    RefreshToken,
    AccessTokenExpiresAt,
    RefreshTokenExpiresAt,
    Scope,
    IdToken,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    Id,
    Identifier,
    Value,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Seat {
    Table,
    Id,
    ObjectId,
    ReservedForUserId,
}

#[derive(DeriveIden)]
enum WorkspaceObject {
    Table,
    Id,
    WorkspaceId,
    X,
    Y,
    Rotation,
    CreatedAt,
    UpdatedAt,
}
