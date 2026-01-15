use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation
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

#[allow(clippy::enum_variant_names)]
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
