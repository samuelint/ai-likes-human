use diesel::prelude::*;
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;

use crate::configuration::domain::dto::NewConfigurationItemDto;
use crate::configuration::domain::{
    dto::ConfigurationItemDto, repository::ConfigurationRepository,
};

use crate::infrastructure::database::sqlite_connection_factory::SQliteConnectionFactory;
use crate::schema::configuration;

#[derive(Provider)]
#[shaku(interface = ConfigurationRepository)]
pub struct DieselConfigurationRepository {
    #[shaku(inject)]
    connection_factory: Arc<dyn SQliteConnectionFactory>,
}

impl DieselConfigurationRepository {
    pub fn new(connection_factory: Arc<dyn SQliteConnectionFactory>) -> Self {
        Self { connection_factory }
    }
}

impl ConfigurationRepository for DieselConfigurationRepository {
    fn get_all(&mut self) -> Result<Vec<ConfigurationItemDto>, Box<dyn Error>> {
        let connection = &mut self.connection_factory.get();

        Ok(configuration::table.load::<ConfigurationItemDto>(connection)?)
    }

    fn find(&mut self, id: &i32) -> Result<Option<ConfigurationItemDto>, Box<dyn Error>> {
        let connection = &mut self.connection_factory.get();

        Ok(configuration::table
            .filter(configuration::id.eq(id))
            .first::<ConfigurationItemDto>(connection)
            .optional()?)
    }

    fn create(
        &mut self,
        item: NewConfigurationItemDto,
    ) -> Result<ConfigurationItemDto, Box<dyn Error>> {
        let connection = &mut self.connection_factory.get();

        Ok(diesel::insert_into(configuration::table)
            .values(&item)
            .returning(ConfigurationItemDto::as_returning())
            .get_result(connection)?)
    }

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        let connection = &mut self.connection_factory.get();

        match diesel::delete(configuration::table.filter(configuration::key.eq(key)))
            .execute(connection)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
