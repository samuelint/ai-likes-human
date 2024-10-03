use crate::entities::profile;
use crate::profile::domain::dto::ProfileDto;
use crate::profile::domain::ProfileRepository;
use anyhow::anyhow;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;
use std::num::ParseIntError;
use std::sync::Arc;

pub struct SeaOrmProfileRepository {
    connection: Arc<DatabaseConnection>,
}

impl SeaOrmProfileRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl ProfileRepository for SeaOrmProfileRepository {
    async fn find(&self, id: &str) -> Result<Option<ProfileDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

        let configuration = profile::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if configuration.is_none() {
            return Ok(None);
        }

        let configuration: ProfileDto = (&configuration.unwrap()).into();

        Ok(Some(configuration))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<ProfileDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let configuration = profile::Entity::find()
            .filter(profile::Column::Name.eq(name))
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if configuration.is_none() {
            return Ok(None);
        }

        let configuration: ProfileDto = (&configuration.unwrap()).into();

        Ok(Some(configuration))
    }

    async fn upsert(&self, dto: &ProfileDto) -> Result<ProfileDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let model: profile::ActiveModel = dto.into();

        let on_conflict = OnConflict::column(profile::Column::Name)
            .update_column(profile::Column::Prompt)
            .to_owned();

        profile::Entity::insert(model)
            .on_conflict(on_conflict)
            .exec(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        let result = profile::Entity::find()
            .filter(profile::Column::Name.eq(dto.name.clone()))
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?
            .ok_or_else(|| "Failed to find inserted item")
            .map_err(|e| anyhow!(e))?;

        Ok((&result).into())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error + Send>> {
        let id: i32 = id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

        profile::Entity::delete_by_id(id)
            .exec(self.connection.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(())
    }
}
