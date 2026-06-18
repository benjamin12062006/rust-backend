mod support;

use backend_config::configuration::loader::load_configuration;
use backend_infrastructure::database::{check_db_connection, create_connection};
use rstest::rstest;
use support::runtime::block_on;

#[rstest]
fn database_connection_ping_is_opt_in() {
    if std::env::var("RUN_DB_TESTS").as_deref() != Ok("true") {
        eprintln!("skipping database test; set RUN_DB_TESTS=true to enable it");
        return;
    }

    block_on(async {
        let config = load_configuration().expect("database tests require valid configuration");
        let pool = create_connection(&config.databases.postgres.uri)
            .await
            .expect("database pool should be created");

        check_db_connection(&pool)
            .await
            .expect("database ping should succeed");
    });
}
