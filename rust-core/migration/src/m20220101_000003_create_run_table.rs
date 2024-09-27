use sea_orm_migration::prelude::*;

use crate::m20220101_000002_create_thread_table::Thread;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Run::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Run::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Run::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Run::AssistantId).string().not_null())
                    .col(ColumnDef::new(Run::Instructions).string())
                    .col(ColumnDef::new(Run::Model).string().not_null())
                    .col(ColumnDef::new(Run::Status).string().not_null())
                    .col(ColumnDef::new(Run::ThreadId).integer())
                    .col(ColumnDef::new(Run::Metadata).json().not_null())
                    .col(ColumnDef::new(Run::Temperature).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_thread")
                            .from(Run::Table, Run::ThreadId)
                            .to(Thread::Table, Thread::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Run::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Run {
    Table,
    Id,
    CreatedAt,
    AssistantId,
    Instructions,
    Metadata,
    Model,
    Status,
    ThreadId,
    Temperature,
}
