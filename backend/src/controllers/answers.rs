use axum::http::StatusCode;
use loco_rs::prelude::*;

use crate::{
    dtos::answers::{AnswerResponse, CreateAnswer},
    models::answers,
};

#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    Path(question_id): Path<i64>,
    Json(params): Json<CreateAnswer>,
) -> Result<Response> {
    let answer = answers::Model::create(&ctx.db, question_id, params).await?;
    format::render()
        .status(StatusCode::CREATED)
        .json(AnswerResponse {
            answer: answer.into(),
        })
}

#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    Path((question_id, id)): Path<(i64, i64)>,
) -> Result<Response> {
    answers::Model::remove(&ctx.db, question_id, id).await?;
    format::render().status(StatusCode::NO_CONTENT).empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/questions")
        .add("/{question_id}/answers", post(create))
        .add("/{question_id}/answers/{id}", delete(remove))
}
