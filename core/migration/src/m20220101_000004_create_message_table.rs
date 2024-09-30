use sea_orm_migration::prelude::*;

use crate::{m20220101_000002_create_thread_table::Thread, m20220101_000003_create_run_table::Run};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::Content).json().not_null())
                    .col(ColumnDef::new(Message::Role).string().not_null())
                    .col(ColumnDef::new(Message::Attachments).json())
                    .col(ColumnDef::new(Message::Metadata).json().not_null())
                    .col(ColumnDef::new(Message::ThreadId).integer())
                    .col(ColumnDef::new(Message::RunId).integer())
                    .col(ColumnDef::new(Message::Status).string().not_null())
                    .col(ColumnDef::new(Message::AssistantId).string())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::ThreadId)
                            .to(Thread::Table, Thread::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::RunId)
                            .to(Run::Table, Run::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    CreatedAt,
    Attachments,
    Content,
    Metadata,
    Role,
    ThreadId,
    RunId,
    Status,
    AssistantId,
}
