use loco_rs::prelude::*;
use validator::ValidationError;

pub use super::_entities::questions::{self, ActiveModel, Entity, Model};
use crate::dtos::questions::{CreateQuestion, UpdateQuestion};

fn not_blank(value: &str) -> std::result::Result<(), ValidationError> {
    if value.chars().all(char::is_whitespace) {
        Err(ValidationError::new("blank"))
    } else {
        Ok(())
    }
}

#[derive(Validate)]
struct Validator {
    #[validate(custom(function = "not_blank", message = "Title can't be blank"))]
    title: String,
    #[validate(custom(function = "not_blank", message = "Name can't be blank"))]
    name: String,
    #[validate(custom(function = "not_blank", message = "Content can't be blank"))]
    #[validate(length(min = 5, message = "Content is too short (minimum is 5 characters)"))]
    content: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            title: self.title.as_ref().to_owned(),
            name: self.name.as_ref().to_owned(),
            content: self.content.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Validatable::validate(&self)?;
        let now = chrono::Utc::now().into();
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        Ok(self)
    }
}

impl Model {
    pub async fn all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Ok(Entity::find()
            .order_by_asc(questions::Column::Id)
            .all(db)
            .await?)
    }

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i64) -> ModelResult<Self> {
        Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(ModelError::EntityNotFound)
    }

    pub async fn create(db: &DatabaseConnection, params: CreateQuestion) -> ModelResult<Self> {
        let item = ActiveModel {
            title: Set(params.title),
            name: Set(params.name),
            content: Set(params.content),
            ..Default::default()
        };
        // Preserve Loco's typed validation error for HTTP; before_save also guards direct ORM writes.
        Validatable::validate(&item)?;
        Ok(item.insert(db).await?)
    }

    pub async fn remove(db: &DatabaseConnection, id: i64) -> ModelResult<()> {
        // The generated foreign key cascades answers in this same DELETE statement.
        if Entity::delete_by_id(id).exec(db).await?.rows_affected == 0 {
            return Err(ModelError::EntityNotFound);
        }
        Ok(())
    }
}

impl ActiveModel {
    pub async fn edit(
        mut self,
        db: &DatabaseConnection,
        params: UpdateQuestion,
    ) -> ModelResult<Model> {
        self.title = Set(params.title);
        self.name = Set(params.name);
        self.content = Set(params.content);
        Validatable::validate(&self)?;
        Ok(self.update(db).await?)
    }
}
