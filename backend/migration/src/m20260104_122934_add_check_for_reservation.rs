use sea_orm_migration::{prelude::*, schema::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_raw(Statement::from_string(
            db.get_database_backend(),
            "UPDATE reservation SET status = 'on_hold' WHERE status NOT IN ('on_hold', 'canceled', 'confirmed', 'expired')"
        )).await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reservation"))
                    .modify_column(string(Alias::new("status")).default("on_hold"))
                    .add_column(string_null("approved_by"))
                    .add_column(date_time_null("approved_at"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl("reservation")
                            .from_col("approved_by")
                            .to_tbl("user")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_raw(Statement::from_string(
            db.get_database_backend(),
            r#"ALTER TABLE "reservation" ADD CONSTRAINT "chk_status_valid" CHECK ("status" IN ('on_hold', 'canceled', 'confirmed', 'expired'))"#,
        )).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Drop the constraint
        db.execute_raw(Statement::from_string(
            db.get_database_backend(),
            r#"ALTER TABLE "reservation" DROP CONSTRAINT "chk_status_valid""#,
        ))
        .await?;

        // 2. Revert column definition (if needed)
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("reservation"))
                    .modify_column(string(Alias::new("status")))
                    .drop_column("approved_by")
                    .drop_column("approved_at")
                    .to_owned(),
            )
            .await
    }
}
