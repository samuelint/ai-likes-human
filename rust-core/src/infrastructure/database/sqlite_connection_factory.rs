use super::migrator::migrate_database;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use r2d2::PooledConnection;
use shaku::Component;

pub type SqliteDBConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

use shaku::Interface;

pub trait SQliteConnectionFactory: Interface {
    fn get(&self) -> SqlitePooledConnection;
}

#[derive(Component)]
#[shaku(interface = SQliteConnectionFactory)]
pub struct SQliteConnectionFactoryImpl {
    connection_pool: SqliteDBConnectionPool,
}

impl SQliteConnectionFactory for SQliteConnectionFactoryImpl {
    fn get(&self) -> SqlitePooledConnection {
        self.connection_pool.get().unwrap()
    }
}

impl SQliteConnectionFactoryImpl {
    fn create_connection_pool(database_url: &str) -> SqliteDBConnectionPool {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

    fn create_migrated_connection_pool(database_url: &str) -> SqliteDBConnectionPool {
        let pool: Pool<ConnectionManager<SqliteConnection>> =
            SQliteConnectionFactoryImpl::create_connection_pool(database_url);
        let mut conn = pool.get().unwrap();
        migrate_database(&mut conn);

        pool
    }

    pub fn new(database_url: &str) -> Self {
        let connection_pool =
            SQliteConnectionFactoryImpl::create_migrated_connection_pool(database_url);
        Self { connection_pool }
    }
}
