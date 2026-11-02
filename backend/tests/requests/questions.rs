use axum::http::StatusCode;
use loco_rs::testing::prelude::*;
use loco_tutorial::{app::App, models::answers};
use sea_orm::{ConnectionTrait, EntityTrait};
use serde_json::{json, Value};
use serial_test::serial;

fn valid() -> Value {
    json!({"title":"日本語の質問", "name":"名前", "content":"あいうえお"})
}

#[tokio::test]
#[serial]
async fn question_crud_answers_and_parent_scope() {
    request::<App, _, _>(|request, ctx| async move {
        assert_eq!(
            request.get("/api/questions").await.json::<Value>(),
            json!({"questions":[]})
        );
        let created = request.post("/api/questions").json(&valid()).await;
        created.assert_status(StatusCode::CREATED);
        let id = created.json::<Value>()["question"]["id"].as_i64().unwrap();
        let path = format!("/api/questions/{id}");
        let mut changed = valid();
        changed["title"] = json!("変更");
        let updated = request.put(&path).json(&changed).await;
        updated.assert_status_ok();
        assert_eq!(updated.json::<Value>()["question"]["title"], "変更");
        let empty = request
            .post(&format!("{path}/answers"))
            .json(&json!({}))
            .await;
        empty.assert_status(StatusCode::CREATED);
        assert!(empty.json::<Value>()["answer"]["content"].is_null());
        let answer_id = empty.json::<Value>()["answer"]["id"].as_i64().unwrap();
        let other = request
            .post("/api/questions")
            .json(&valid())
            .await
            .json::<Value>();
        let other_id = other["question"]["id"].as_i64().unwrap();
        request
            .delete(&format!("/api/questions/{other_id}/answers/{answer_id}"))
            .await
            .assert_status(StatusCode::NOT_FOUND);
        assert_eq!(
            request.get(&path).await.json::<Value>()["answers"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        request
            .delete(&format!("{path}/answers/{answer_id}"))
            .await
            .assert_status(StatusCode::NO_CONTENT);
        request
            .delete(&format!("{path}/answers/{answer_id}"))
            .await
            .assert_status(StatusCode::NOT_FOUND);
        request
            .post(&format!("{path}/answers"))
            .json(&json!({"name":"", "content":""}))
            .await
            .assert_status(StatusCode::CREATED);
        request
            .delete(&path)
            .await
            .assert_status(StatusCode::NO_CONTENT);
        assert!(answers::Entity::find()
            .all(&ctx.db)
            .await
            .unwrap()
            .is_empty());
        request
            .get(&path)
            .await
            .assert_status(StatusCode::NOT_FOUND);
        request
            .put(&path)
            .json(&valid())
            .await
            .assert_status(StatusCode::NOT_FOUND);
        request
            .delete(&path)
            .await
            .assert_status(StatusCode::NOT_FOUND);
        request
            .post(&format!("{path}/answers"))
            .json(&json!({}))
            .await
            .assert_status(StatusCode::NOT_FOUND);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn standard_validation_errors_preserve_saved_data() {
    request::<App, _, _>(|request, _ctx| async move {
        for content in ["", "    ", "　　　　　", "あいうえ", "😀😀😀😀"] {
            let mut input = valid();
            input["content"] = json!(content);
            let response = request.post("/api/questions").json(&input).await;
            response.assert_status(StatusCode::BAD_REQUEST);
            assert!(response.json::<Value>()["errors"]["content"].is_array());
        }
        for field in ["title", "name"] {
            let mut input = valid();
            input[field] = json!("　 \t");
            request
                .post("/api/questions")
                .json(&input)
                .await
                .assert_status(StatusCode::BAD_REQUEST);
        }
        request
            .post("/api/questions")
            .json(&json!({"title":23}))
            .await
            .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        let id = request
            .post("/api/questions")
            .json(&valid())
            .await
            .json::<Value>()["question"]["id"]
            .as_i64()
            .unwrap();
        let path = format!("/api/questions/{id}");
        request
            .put(&path)
            .json(&json!({}))
            .await
            .assert_status(StatusCode::BAD_REQUEST);
        assert_eq!(
            request.get(&path).await.json::<Value>()["question"]["content"],
            "あいうえお"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn database_failure_returns_generic_server_error() {
    request::<App, _, _>(|request, ctx| async move {
        ctx.db
            .execute_unprepared("DROP TABLE answers; DROP TABLE questions;")
            .await
            .unwrap();
        let response = request.get("/api/questions").await;
        response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!response.text().contains("no such table"));
    })
    .await;
}
