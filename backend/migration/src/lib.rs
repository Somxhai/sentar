pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251109_032731_app_schema;
mod m20251207_021458_decimal_and_object_grid_schema;
mod m20251207_041152_drop_event_object_grid;
mod m20251207_042647_change_time_to_utc;
mod m20251211_142132_jwks;
mod m20251222_073639_form_submission;
mod m20251223_134134_rbac_and_moddatetime;
mod m20251231_065451_add_check_for_event_object;
mod m20260104_122934_add_check_for_reservation;
mod m20260104_141735_add_z_to_event_object_position;
mod m20260114_152506_frontend_request;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251109_032731_app_schema::Migration),
            Box::new(m20251207_021458_decimal_and_object_grid_schema::Migration),
            Box::new(m20251207_041152_drop_event_object_grid::Migration),
            Box::new(m20251207_042647_change_time_to_utc::Migration),
            Box::new(m20251211_142132_jwks::Migration),
            Box::new(m20251222_073639_form_submission::Migration),
            Box::new(m20251223_134134_rbac_and_moddatetime::Migration),
            Box::new(m20251231_065451_add_check_for_event_object::Migration),
            Box::new(m20260104_122934_add_check_for_reservation::Migration),
            Box::new(m20260104_141735_add_z_to_event_object_position::Migration),
            Box::new(m20260114_152506_frontend_request::Migration),
        ]
    }
}
