use axum::http::StatusCode;
use loco_rs::prelude::*;

use crate::{
    dtos::questions::{
        CreateQuestion, QuestionDetail, QuestionDto, QuestionList, QuestionResponse, UpdateQuestion,
    },
    models::{answers, questions},
};

#[debug_handler]
async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let questions = questions::Model::all(&ctx.db).await?;
    format::json(QuestionList {
        questions: questions.into_iter().map(QuestionDto::from).collect(),
    })
}

#[debug_handler]
async fn show(State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Response> {
    let question = questions::Model::find_by_id(&ctx.db, id).await?;
    let answers = answers::Model::for_question(&ctx.db, id).await?;
    format::json(QuestionDetail {
        question: question.into(),
        answers: answers.into_iter().map(Into::into).collect(),
    })
}

#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateQuestion>,
) -> Result<Response> {
    let question = questions::Model::create(&ctx.db, params).await?;
    format::render()
        .status(StatusCode::CREATED)
        .header("location", &format!("/api/questions/{}", question.id))
        .json(QuestionResponse {
            question: question.into(),
        })
}

#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(params): Json<UpdateQuestion>,
) -> Result<Response> {
    let question = questions::Model::find_by_id(&ctx.db, id)
        .await?
        .into_active_model()
        .edit(&ctx.db, params)
        .await?;
    format::json(QuestionResponse {
        question: question.into(),
    })
}

#[debug_handler]
async fn remove(State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Response> {
    questions::Model::remove(&ctx.db, id).await?;
    format::render().status(StatusCode::NO_CONTENT).empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/questions")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", get(show))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
}
