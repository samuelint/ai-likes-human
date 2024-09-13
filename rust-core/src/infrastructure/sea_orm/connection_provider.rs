use sea_orm::DatabaseConnection;
use shaku::Component;
use shaku::Interface;
use std::sync::Arc;

pub trait ConnectionProvider: Interface {
    fn get(&self) -> Arc<DatabaseConnection>;
}

#[derive(Component)]
#[shaku(interface = ConnectionProvider)]
pub struct ConnectionProviderImpl {
    #[allow(dead_code)]
    connection: Arc<DatabaseConnection>,
}

impl ConnectionProvider for ConnectionProviderImpl {
    fn get(&self) -> Arc<DatabaseConnection> {
        Arc::clone(&self.connection)
    }
}

impl ConnectionProviderImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}
