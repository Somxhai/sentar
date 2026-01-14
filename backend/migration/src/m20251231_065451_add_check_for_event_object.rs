use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Ensure column type/default are correct
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("event_object"))
                    .modify_column(string(Alias::new("status")).default("available"))
                    .to_owned(),
            )
            .await?;

        // 2. Add the CHECK constraint manually
        // Note: We use raw SQL because SeaORM's `modify_column` cannot attach
        // a CHECK constraint to an existing column in PostgreSQL.
        let db = manager.get_connection();

        // We assume PostgreSQL based on the error.
        // If using MySQL, the syntax is slightly different (DROP CHECK vs DROP CONSTRAINT).
        db.execute_unprepared(r#"ALTER TABLE "event_object" ADD CONSTRAINT "chk_status_valid" CHECK ("status" IN ('available', 'reserved'))"#).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Drop the constraint
        db.execute_unprepared(r#"ALTER TABLE "event_object" DROP CONSTRAINT "chk_status_valid""#)
            .await?;

        // 2. Revert column definition (if needed)
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("event_object"))
                    .modify_column(string(Alias::new("status")).default("available"))
                    .to_owned(),
            )
            .await
    }
}
