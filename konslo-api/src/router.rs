use axum::Router;
use axum::routing::get;
use konslo_core::service::habit::HabitService;
use crate::handlers::{create_habits_handler, delete_habits_handler, get_all_habits_handler, get_habit_handler};

pub fn get_router(service: HabitService) -> Router {
    Router::new()
        .route("/habits",get(get_all_habits_handler).post(create_habits_handler),)
        .route("/habits/{id}",get(get_habit_handler).delete(delete_habits_handler),)
        .with_state(service)
}