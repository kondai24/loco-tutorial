use sea_orm::prelude::DateTimeWithTimeZone;
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct QuestionDto {
    #[ts(type = "number")]
    pub id: i64,
    pub title: String,
    pub name: String,
    pub content: String,
    #[ts(type = "string")]
    pub created_at: DateTimeWithTimeZone,
    #[ts(type = "string")]
    pub updated_at: DateTimeWithTimeZone,
}

impl From<crate::models::_entities::questions::Model> for QuestionDto {
    fn from(m: crate::models::_entities::questions::Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            name: m.name,
            content: m.content,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct CreateQuestion {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct UpdateQuestion {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub content: String,
}

#[derive(serde::Serialize)]
pub struct QuestionList {
    pub questions: Vec<QuestionDto>,
}
#[derive(serde::Serialize)]
pub struct QuestionResponse {
    pub question: QuestionDto,
}
#[derive(serde::Serialize)]
pub struct QuestionDetail {
    pub question: QuestionDto,
    pub answers: Vec<super::answers::AnswerDto>,
}
