use crate::schema::configuration;
use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable, Debug)]
#[diesel(table_name = configuration)]
pub struct ConfigurationItemDto {
    pub id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = configuration)]
pub struct NewConfigurationItemDto<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
