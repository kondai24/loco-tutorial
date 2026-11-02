use sea_orm::prelude::DateTimeWithTimeZone;
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct AnswerDto {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub question_id: i64,
    pub name: Option<String>,
    pub content: Option<String>,
    #[ts(type = "string")]
    pub created_at: DateTimeWithTimeZone,
    #[ts(type = "string")]
    pub updated_at: DateTimeWithTimeZone,
}
impl From<crate::models::_entities::answers::Model> for AnswerDto {
    fn from(m: crate::models::_entities::answers::Model) -> Self {
        Self {
            id: m.id,
            question_id: m.question_id,
            name: m.name,
            content: m.content,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct CreateAnswer {
    pub name: Option<String>,
    pub content: Option<String>,
}
#[derive(serde::Serialize)]
pub struct AnswerResponse {
    pub answer: AnswerDto,
}
