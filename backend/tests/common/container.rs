use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use testcontainers::{ImageExt, core::Mount};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use tokio::sync::OnceCell;
use uuid::Uuid;
struct PgContext {
    master_db: DatabaseConnection,
    base_url: String,
}
static GLOBAL_PG: OnceCell<PgContext> = OnceCell::const_new();

pub struct PgContainer;
impl PgContainer {
    async fn start() -> PgContext {
        let host_path = "/var/tmp/postgres-data";
        let mount = Mount::tmpfs_mount(host_path);
        let container = Postgres::default()
            .with_reuse(testcontainers::ReuseDirective::Always)
            .with_mount(mount)
            .start()
            .await
            .expect("Failed to start postgres container");

        // default postgres: postgres://postgres:postgres@127.0.0.1:5432/postgres
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port");
        let base_url = format!("postgres://postgres:postgres@127.0.0.1:{}", port);

        let connection_string = format!("{}/postgres", base_url);
        let mut opt = ConnectOptions::new(connection_string);
        opt.max_connections(1);
        opt.connect_timeout(Duration::from_mins(5));

        let db = Database::connect(opt)
            .await
            .expect("Can not connect to postgres test container");

        let template_exists: bool = db
            .query_one_raw(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                "SELECT 1 FROM pg_database WHERE datname = 'test_template'",
            ))
            .await
            .unwrap()
            .is_some();

        if !template_exists {
            db.execute_raw(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                "CREATE DATABASE test_template",
            ))
            .await
            .expect("Failed to create template db");

            let template_url = format!("{}/test_template", base_url);

            let mut opt = ConnectOptions::new(template_url);
            opt.max_connections(1);
            opt.connect_timeout(Duration::from_secs(5));

            let template_db = Database::connect(opt)
                .await
                .expect("Failed to connect to template");

            Migrator::up(&template_db, None)
                .await
                .expect("Failed to run migrations");

            template_db
                .close()
                .await
                .expect("Failed to close template db");
        }

        PgContext {
            master_db: db,
            base_url,
        }
    }

    pub async fn create_test_db() -> eyre::Result<DatabaseConnection> {
        let context = GLOBAL_PG.get_or_init(|| Self::start()).await;

        let random_db_name = format!("test_{}", Uuid::new_v4());

        let _clone_template = context
            .master_db
            .execute_raw(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!(
                    "CREATE DATABASE \"{}\" TEMPLATE test_template",
                    random_db_name
                ),
            ))
            .await?;

        let url = format!("{}/{}", context.base_url, random_db_name);

        let mut opt = ConnectOptions::new(url);
        opt.max_connections(1);
        opt.connect_timeout(Duration::from_secs(5));
        opt.idle_timeout(Duration::from_secs(1));
        opt.sqlx_logging(false);

        let db = Database::connect(opt).await?;
        Ok(db)
    }
}
