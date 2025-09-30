use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub type MatchId = usize;
pub type TournamentId = usize;
pub type SeasonId = usize;
pub type TeamId = usize;
pub type SeasonMap = BTreeMap<SeasonId, Season>;
pub type SeasonMatchMap = BTreeMap<SeasonId, Vec<MatchId>>;
pub type Year = u32;
pub type MatchDataMap = HashMap<MatchId, Match>;
pub type YearlyMatchMap = BTreeMap<Year, Vec<MatchId>>;
pub type TournamentIdNameMap = HashMap<TournamentId, String>;
pub type TournamentNameIdMap = HashMap<String, TournamentId>;
pub type TournamentMatchMap = BTreeMap<TournamentId, Vec<MatchId>>;
pub type TournamentSeasonMatchMap = BTreeMap<TournamentId, SeasonMatchMap>;
pub type TournamentYearlyMatchMap = BTreeMap<TournamentId, YearlyMatchMap>;

pub type TeamIdNameMap = BTreeMap<TeamId, String>;
pub type TeamNameIdMap = BTreeMap<String, TeamId>;
pub type TeamTournamentSeasonMatchMap = BTreeMap<TeamId, TournamentSeasonMatchMap>;
pub type TeamTournamentYearlyMatchMap = BTreeMap<TeamId, TournamentYearlyMatchMap>;

#[derive(Debug, Serialize)]
pub struct Season {
    pub id: SeasonId,
    pub start_year: Year,
    pub end_year: Option<Year>,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tournament<'a> {
    pub id: TournamentId,
    pub name: &'a str,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Team<'a> {
    pub id: TeamId,
    pub name: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Match {
    #[serde(skip_deserializing)]
    pub id: MatchId,
    #[serde(skip_deserializing)]
    pub season_id: SeasonId,
    #[serde(skip_deserializing)]
    pub tournament_id: TournamentId,
    pub round: Option<String>,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub team1: String,
    pub team2: String,
    pub score: Score,
    pub stage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchList {
    pub name: String,
    pub matches: Vec<Match>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreRaw {
    ht: Option<[u8; 2]>,
    ft: Option<[u8; 2]>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScoreGoals(u8, u8);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "ScoreRaw")]
pub struct Score {
    pub half_time: Option<ScoreGoals>,
    pub full_time: Option<ScoreGoals>,
}

impl From<ScoreRaw> for Score {
    fn from(value: ScoreRaw) -> Self {
        let half_time = value.ht.map(|raw| ScoreGoals(raw[0], raw[1]));
        let full_time = value.ft.map(|raw| ScoreGoals(raw[0], raw[1]));

        Score {
            half_time,
            full_time,
        }
    }
}
