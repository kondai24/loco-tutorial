#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20261101_005222_questions;
mod m20261101_005353_answers;
mod m20261101_005547_add_answers_question_index;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261101_005222_questions::Migration),
            Box::new(m20261101_005353_answers::Migration),
            Box::new(m20261101_005547_add_answers_question_index::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
