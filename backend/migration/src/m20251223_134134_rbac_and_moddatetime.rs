use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS moddatetime;")
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("workspace_member")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("workspace_id"))
                    .col(text("user_id"))
                    .col(text("status"))
                    .col(text("invited_by"))
                    .col(text("role").default("guest"))
                    .col(timestamp("updated_at").default(Expr::current_timestamp()))
                    .col(timestamp("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from("workspace_member", "workspace_id")
                            .to("workspace", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("workspace_member", "user_id")
                            .to("user", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("workspace_member", "invited_by")
                            .to("user", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            CREATE TRIGGER workspace_member_moddatetime
                BEFORE UPDATE on workspace_member
                FOR EACH ROW
                EXECUTE PROCEDURE moddatetime (updated_at);
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER form_submission_moddatetime 
                BEFORE UPDATE on form_submission 
                FOR EACH ROW
                EXECUTE PROCEDURE moddatetime (updated_at);
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table("workspace_member").to_owned())
            .await
    }
}
