use axum::Router;
use axum::routing::{get, post, put};
use konslo_core::services::habit::HabitService;
use std::sync::Arc;
use konslo_core::services::check::CheckService;
use crate::handlers::check_handlers::{create_check_handler, get_checks_by_habit_handler, update_check_handler};
use crate::handlers::habit_handler::{create_habits_handler, delete_habits_handler, get_all_habits_handler, get_all_habits_with_today_checks_handler, get_habit_handler};

#[derive(Clone)]
pub struct AppState {
    pub habit_service: Arc<dyn HabitService>,
    pub check_service: Arc<dyn CheckService>,
}

pub fn get_router(state: AppState) -> Router {
    Router::new()

        .route("/habits",
               get(get_all_habits_handler).post(create_habits_handler),
        ).route("/habits/today",
                get(get_all_habits_with_today_checks_handler)
        )
        .route("/habits/{id}",
               get(get_habit_handler).delete(delete_habits_handler),
        )
        .route("/habits/{id}/checks",
            post(create_check_handler).get(get_checks_by_habit_handler)
        )
        .route("/habits/{habit_id}/checks/{check_id}",
               put(update_check_handler)
        )
        .with_state(state)
}
