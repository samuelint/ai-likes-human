use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_configuration_table::Configuration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Configuration::Table)
            .columns([Configuration::Key, Configuration::Value])
            .values_panic(["SELECTED_LLM_MODEL".into(), "openai:gpt-4o-mini".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Configuration::Table)
            .and_where(Expr::col(Configuration::Key).eq("SELECTED_LLM_MODEL"))
            .to_owned();

        manager.exec_stmt(delete).await
    }
}
