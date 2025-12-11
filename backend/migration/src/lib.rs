pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251109_032731_app_schema;
mod m20251207_021458_decimal_and_object_grid_schema;
mod m20251207_041152_drop_event_object_grid;
mod m20251207_042647_change_time_to_utc;
mod m20251211_142132_jwks;

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
        ]
    }
}
