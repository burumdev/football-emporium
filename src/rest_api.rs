use axum::{Router, routing::get};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::appstate::AppState;

mod get_api;
use get_api::*;

pub mod query_types;
pub mod response_types;

pub fn get_router(state: AppState) -> Router {
    let allowed_origins = [
        state.env_vars.host_origin.parse().unwrap(),
        state.env_vars.fe_dev_origin.parse().unwrap(),
    ];

    Router::new()
        .route("/seasons", get(get_seasons))
        .route("/all_matches", get(get_all_matches))
        .route("/tournaments", get(get_tournaments))
        .route("/teams", get(get_teams))
        .route("/seasons/{id}", get(get_season_matches_by_id))
        .route("/tournaments/{id}", get(get_tournament_matches_by_id))
        .route(
            "/tournaments/{id}/seasons/{season_id}",
            get(get_tournament_matches_by_season_id),
        )
        .route(
            "/tournaments/{id}/years/{year}",
            get(get_tournament_matches_by_year),
        )
        .route(
            "/tournaments/{id}/years/{start}/{end}",
            get(get_tournament_matches_by_year_range),
        )
        .route("/years/{year}", get(get_yearly_matches_by_year))
        .route(
            "/years/{start}/{end}",
            get(get_yearly_matches_by_year_range),
        )
        .route("/teams/{id}", get(get_team_matches_by_id))
        .route(
            "/teams/{id}/seasons/{season_id}",
            get(get_team_matches_by_season_id),
        )
        .route("/teams/{id}/years/{year}", get(get_team_matches_by_year))
        .route(
            "/teams/{id}/years/{year_start}/{year_end}",
            get(get_team_matches_by_year_range),
        )
        .route(
            "/teams/{id}/tournaments/{tour_id}",
            get(get_team_matches_by_tournament_id),
        )
        .route(
            "/teams/{id}/tournaments/{tour_id}/seasons/{season_id}",
            get(get_team_tournament_matches_by_season_id),
        )
        .route(
            "/teams/{id}/tournaments/{tour_id}/years/{year}",
            get(get_team_tournament_matches_by_year),
        )
        .route(
            "/teams/{id}/tournaments/{tour_id}/years/{year_start}/{year_end}",
            get(get_team_tournament_matches_by_year_range),
        )
        .layer(ServiceBuilder::new().layer(CorsLayer::new().allow_origin(allowed_origins)))
        .with_state(state)
}
