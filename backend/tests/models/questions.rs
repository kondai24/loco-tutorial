use loco_rs::prelude::*;
use loco_tutorial::{
    app::App,
    dtos::questions::{CreateQuestion, UpdateQuestion},
    models::questions,
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn direct_orm_writes_are_validated() {
    let boot = boot_test::<App>().await.unwrap();
    let result = questions::ActiveModel {
        title: Set("　".into()),
        name: Set("名前".into()),
        content: Set("あいうえお".into()),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await;
    assert!(result.is_err());
    assert!(questions::Model::all(&boot.app_context.db)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
#[serial]
async fn model_transition_preserves_creation_time_and_updates_timestamp() {
    let boot = boot_test::<App>().await.unwrap();
    let db = &boot.app_context.db;
    let original = questions::Model::create(
        db,
        CreateQuestion {
            title: "質問".into(),
            name: "名前".into(),
            content: "あいうえお".into(),
        },
    )
    .await
    .unwrap();
    let changed = original
        .clone()
        .into_active_model()
        .edit(
            db,
            UpdateQuestion {
                title: "更新".into(),
                name: "名前".into(),
                content: "かきくけこ".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(changed.created_at, original.created_at);
    assert!(changed.updated_at >= original.updated_at);
    assert_eq!(changed.title, "更新");
    let mut invalid = changed.into_active_model();
    invalid.content = Set("あいうえ".into());
    assert!(invalid.update(db).await.is_err());
    assert_eq!(
        questions::Model::find_by_id(db, original.id)
            .await
            .unwrap()
            .content,
        "かきくけこ"
    );
}
