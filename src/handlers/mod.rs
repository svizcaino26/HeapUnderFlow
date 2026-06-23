use crate::models::*;
use axum::{response::IntoResponse, Json};
use serde::Serialize;

mod handlers_inner;

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(
        QuestionDetail {
            question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
            title: question.title,
            description: question.description,
            created_at: "2022-12-31 18:44:08.287442".to_string(),
        }
    )
}

pub async fn read_questions() -> impl IntoResponse {
    Json(
        vec![
            QuestionDetail {
                question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                title: "Newly Created Question".to_string(),
                description: "Some description".to_string(),
                created_at: "2022-12-31 18:44:08.287442".to_string(),
            },
            QuestionDetail {
                question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                title: "Another Newly Created Question".to_string(),
                description: "Some other  description".to_string(),
                created_at: "2022-12-31 18:44:08.287442".to_string(),
            },
        ]
    )
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    ()
}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    Json(
        AnswerDetail {
            answer_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
            question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
            content: "Some content".to_string(),
            created_at: "2022-12-31 18:44:08.287442".to_string(),
        }
    )
}

pub async fn read_answers(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    Json(
        vec![
            AnswerDetail {
                answer_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                content: "Some content".to_string(),
                created_at: "2022-12-31 18:44:08.287442".to_string(),
            },
            AnswerDetail {
                answer_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
                content: "Some content".to_string(),
                created_at: "2022-12-31 18:44:08.287442".to_string(),
            },
        ]
    )
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) -> impl IntoResponse {
    ()
}
