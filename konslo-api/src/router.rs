use crate::handlers::habit::{
    add_check_handler, create_habit_handler, delete_habit_handler, get_all_habits_with_period_checks_handler,
    get_habit_handler, reset_period_checks_handler,
};
use axum::Router;
use axum::routing::{get, post};
use konslo_core::services::habit::HabitService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub habit_service: Arc<dyn HabitService>,
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/habits", post(create_habit_handler).get(get_all_habits_with_period_checks_handler))
        .route("/habits/{id}", get(get_habit_handler).delete(delete_habit_handler))
        .route("/habits/{id}/checks", post(add_check_handler).delete(reset_period_checks_handler))
        .with_state(state)
}
