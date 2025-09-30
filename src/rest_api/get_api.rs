use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::imdb::{
    IMDBReady,
    data_types::{Match, SeasonId, TeamId, TournamentId, Year},
};

use crate::rest_api::{query_types::*, response_types::*};

#[axum::debug_handler]
pub async fn get_seasons(State(db): State<IMDBReady>) -> Json<Value> {
    Json(json!(db.seasons()))
}

#[axum::debug_handler]
pub async fn get_tournaments(State(db): State<IMDBReady>) -> Json<Value> {
    Json(json!(db.tournaments()))
}

#[axum::debug_handler]
pub async fn get_teams(State(db): State<IMDBReady>) -> Json<Value> {
    Json(json!(db.teams()))
}

#[axum::debug_handler]
pub async fn get_all_matches(
    Query(q_params): Query<QueryParams>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.all_matches().map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

#[axum::debug_handler]
pub async fn get_season_matches_by_id(
    Query(q_params): Query<QueryParams>,
    Path(id): Path<SeasonId>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.season_matches_by_id(&id).map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_yearly_matches_by_year(
    Query(q_params): Query<QueryParams>,
    Path(year): Path<Year>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.yearly_matches_by_year(&year).map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_yearly_matches_by_year_range(
    Query(q_params): Query<QueryParams>,
    Path((year_start, year_end)): Path<(Year, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.yearly_matches_year_range(&year_start, &year_end)
        .map(|(total, it)| {
            Json(json!(MatchListResponse {
                total,
                list: paginate_matches(&q_params, it),
            }))
        })
}

#[axum::debug_handler]
pub async fn get_tournament_matches_by_id(
    Query(q_params): Query<QueryParams>,
    Path(tour_id): Path<TournamentId>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.tournament_matches_by_id(&tour_id).map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_tournament_matches_by_season_id(
    Query(q_params): Query<QueryParams>,
    Path((tour_id, season_id)): Path<(TournamentId, SeasonId)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.tournament_matches_by_season_id(&tour_id, &season_id)
        .map(|it| {
            Json(json!(MatchListResponse {
                total: it.size_hint().1.unwrap_or(0),
                list: paginate_matches(&q_params, it)
            }))
        })
}

#[axum::debug_handler]
pub async fn get_tournament_matches_by_year(
    Query(q_params): Query<QueryParams>,
    Path((tour_id, year)): Path<(TournamentId, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.tournament_matches_by_year(&tour_id, &year).map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_tournament_matches_by_year_range(
    Query(q_params): Query<QueryParams>,
    Path((tour_id, year_start, year_end)): Path<(TournamentId, Year, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.tournament_yearly_matches_year_range(&tour_id, &year_start, &year_end)
        .map(|(total, it)| {
            Json(json!(MatchListResponse {
                total,
                list: paginate_matches(&q_params, it),
            }))
        })
}

#[axum::debug_handler]
pub async fn get_team_matches_by_id(
    Query(q_params): Query<QueryParams>,
    Path(id): Path<TeamId>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_matches_by_id(&id, &q_params.home_away.unwrap_or(HomeAwayOption::Both))
        .map(|(total, it)| {
            Json(json!(MatchListResponse {
                total,
                list: paginate_matches(&q_params, it),
            }))
        })
}

#[axum::debug_handler]
pub async fn get_team_matches_by_season_id(
    Query(q_params): Query<QueryParams>,
    Path((team_id, season_id)): Path<(TeamId, SeasonId)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_matches_by_season_id(
        &team_id,
        &season_id,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_matches_by_year(
    Query(q_params): Query<QueryParams>,
    Path((team_id, year)): Path<(TeamId, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_matches_by_year(
        &team_id,
        &year,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_matches_by_year_range(
    Query(q_params): Query<QueryParams>,
    Path((team_id, year_start, year_end)): Path<(TeamId, Year, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_matches_year_range(
        &team_id,
        &year_start,
        &year_end,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_matches_by_tournament_id(
    Query(q_params): Query<QueryParams>,
    Path((team_id, tour_id)): Path<(TeamId, TournamentId)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_matches_by_tournament_id(
        &team_id,
        &tour_id,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_tournament_matches_by_season_id(
    Query(q_params): Query<QueryParams>,
    Path((team_id, tour_id, season_id)): Path<(TeamId, TournamentId, SeasonId)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_tournament_matches_by_season_id(
        &team_id,
        &tour_id,
        &season_id,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_tournament_matches_by_year(
    Query(q_params): Query<QueryParams>,
    Path((team_id, tour_id, year)): Path<(TeamId, TournamentId, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_tournament_matches_by_year(
        &team_id,
        &tour_id,
        &year,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|it| {
        Json(json!(MatchListResponse {
            total: it.size_hint().1.unwrap_or(0),
            list: paginate_matches(&q_params, it)
        }))
    })
}

#[axum::debug_handler]
pub async fn get_team_tournament_matches_by_year_range(
    Query(q_params): Query<QueryParams>,
    Path((team_id, tour_id, year_start, year_end)): Path<(TeamId, TournamentId, Year, Year)>,
    State(db): State<IMDBReady>,
) -> Result<Json<Value>, StatusCode> {
    db.team_tournament_matches_by_year_range(
        &team_id,
        &tour_id,
        &year_start,
        &year_end,
        &q_params.home_away.unwrap_or(HomeAwayOption::Both),
    )
    .map(|(total, it)| {
        Json(json!(MatchListResponse {
            total,
            list: paginate_matches(&q_params, it),
        }))
    })
}

// UTILITIES
fn paginate_matches<'a>(
    q_params: &QueryParams,
    it: impl Iterator<Item = &'a Match>,
) -> Vec<&'a Match> {
    const DEFAULT_PER_PAGE: PagPerPage = PagPerPage::Ten;

    let offset = q_params.offset.unwrap_or(0);
    let per_page = q_params.per_page.unwrap_or(DEFAULT_PER_PAGE) as usize;

    it.skip(offset).take(per_page).collect::<Vec<&Match>>()
}
