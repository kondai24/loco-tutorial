use loco_rs::prelude::*;

pub use super::_entities::answers::{self, ActiveModel, Entity, Model};
use crate::dtos::answers::CreateAnswer;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = chrono::Utc::now().into();
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        Ok(self)
    }
}

impl Model {
    pub async fn for_question(db: &DatabaseConnection, question_id: i64) -> ModelResult<Vec<Self>> {
        Ok(Entity::find()
            .filter(answers::Column::QuestionId.eq(question_id))
            .order_by_asc(answers::Column::Id)
            .all(db)
            .await?)
    }

    pub async fn create(
        db: &DatabaseConnection,
        question_id: i64,
        params: CreateAnswer,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        super::questions::Model::find_by_id(&txn, question_id).await?;
        let answer = ActiveModel {
            question_id: Set(question_id),
            name: Set(params.name),
            content: Set(params.content),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(answer)
    }

    pub async fn remove(db: &DatabaseConnection, question_id: i64, id: i64) -> ModelResult<()> {
        let txn = db.begin().await?;
        super::questions::Model::find_by_id(&txn, question_id).await?;
        let deleted = Entity::delete_many()
            .filter(answers::Column::QuestionId.eq(question_id))
            .filter(answers::Column::Id.eq(id))
            .exec(&txn)
            .await?;
        if deleted.rows_affected == 0 {
            return Err(ModelError::EntityNotFound);
        }
        txn.commit().await?;
        Ok(())
    }
}
