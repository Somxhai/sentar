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
                    .table("form_submission")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text("respondent_id"))
                    .col(uuid("form_id"))
                    .col(json_binary("answer"))
                    .col(timestamp("submitted_at"))
                    .col(timestamp("updated_at"))
                    .col(
                        text("status")
                            .default("draft")
                            .check(Expr::col("status").in_tuples(["draft", "submitted"])),
                    )
                    .col(text_null("user_agent"))
                    .col(text_null("ip_address"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("form_submission", "respondent_id")
                            .to("user", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("form_submission", "form_id")
                            .to("form", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let form_updated_by_fk = TableForeignKey::new()
            .name("updated_by-fk")
            .from_tbl("form")
            .from_col("updated_by")
            .to_tbl("user")
            .to_col("id")
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table("form")
                    .add_column_if_not_exists(boolean("is_active").default(true))
                    .add_column_if_not_exists(text("updated_by"))
                    .add_foreign_key(&form_updated_by_fk)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table("form")
                    .drop_column("updated_by")
                    .drop_column("is_active")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("form_submission").to_owned())
            .await
    }
}
