use serde::Serialize;

use crate::imdb::data_types::Match;

#[derive(Serialize)]
pub struct MatchListResponse<'a> {
    pub total: usize,
    pub list: Vec<&'a Match>,
}
