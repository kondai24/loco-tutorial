use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("index_answers_on_question_id")
                    .table(Alias::new("answers"))
                    .col(Alias::new("question_id"))
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("index_answers_on_question_id")
                    .table(Alias::new("answers"))
                    .to_owned(),
            )
            .await
    }
}
