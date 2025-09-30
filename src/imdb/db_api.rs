use std::collections::BTreeMap;

use axum::http::StatusCode;
use either::Either;

use crate::{
    imdb::{
        IMDB, ReadyState,
        data_types::{
            Match, MatchId, Season, SeasonId, Team, TeamId, TeamTournamentSeasonMatchMap,
            TeamTournamentYearlyMatchMap, Tournament, TournamentId, Year,
        },
    },
    rest_api::query_types::*,
};

const _MOD: &str = "IMDB_API";

impl IMDB<ReadyState> {
    pub fn seasons(&self) -> Vec<&Season> {
        Vec::from_iter(self.season_map.values())
    }

    pub fn tournaments(&self) -> Vec<Tournament<'_>> {
        let mut sort_it: Vec<_> = self
            .tournament_id_name_map
            .iter()
            .map(|tour| Tournament {
                id: *tour.0,
                name: tour.1,
            })
            .collect();

        sort_it.sort_unstable_by(|first, second| first.id.cmp(&second.id));

        sort_it
    }

    pub fn teams(&self) -> Vec<Team<'_>> {
        let sort_it: Vec<_> = self
            .team_id_name_map
            .iter()
            .map(|team| Team {
                id: *team.0,
                name: team.1,
            })
            .collect();

        sort_it
    }

    pub fn all_matches(&self) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        Ok((
            self.season_match_map
                .range(..)
                .fold(0, |acc, list| acc + list.1.len()),
            self.season_match_map
                .range(..)
                .flat_map(|(_, sea_matches)| self.matches_by_slice(sea_matches)),
        ))
    }

    pub fn season_matches_by_id(
        &self,
        season_id: &SeasonId,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        self.season_match_map
            .get(season_id)
            .map(|match_list| self.matches_by_slice(match_list))
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub fn match_by_id(&self, match_id: &MatchId) -> Result<&Match, StatusCode> {
        self.match_data_map
            .get(match_id)
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub fn tournament_by_id(&self, tour_id: &TournamentId) -> Result<&str, StatusCode> {
        self.tournament_id_name_map
            .get(tour_id)
            .map(|tour| tour.as_str())
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub fn tournament_matches_by_id(
        &self,
        tour_id: &TournamentId,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        self.tournament_match_map
            .get(tour_id)
            .map(|match_list| self.matches_by_slice(match_list))
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub fn yearly_matches_by_year(
        &self,
        year: &Year,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        self.check_year_exists_in_range(year)?;
        self.yearly_match_map
            .get(year)
            .map(|match_list| self.matches_by_slice(match_list))
            .ok_or(StatusCode::NOT_FOUND)
    }

    pub fn yearly_matches_year_range(
        &self,
        year_start: &Year,
        year_end: &Year,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        self.check_btreemap_range(year_start, year_end, &self.yearly_match_map)?;

        Ok((
            self.yearly_match_map
                .range(year_start..=year_end)
                .fold(0, |acc, list| acc + list.1.len()),
            self.yearly_match_map
                .range(year_start..=year_end)
                .flat_map(|(_, year_matches)| self.matches_by_slice(year_matches)),
        ))
    }

    pub fn tournament_matches_by_season_id(
        &self,
        tour_id: &TournamentId,
        season_id: &SeasonId,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        let tour_map = self.get_inner_map(&self.tournament_season_match_map, tour_id)?;
        let match_list = tour_map.get(season_id);

        match match_list {
            Some(list) => Ok(Either::Left(self.matches_by_slice(list))),
            None => Ok(Either::Right([].iter())),
        }
    }

    pub fn tournament_matches_by_year(
        &self,
        tour_id: &TournamentId,
        year: &Year,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        self.check_year_exists_in_range(year)?;
        let tour_year_map = self.get_inner_map(&self.tournament_yearly_match_map, tour_id)?;

        let match_list = tour_year_map.get(year);

        match match_list {
            Some(list) => Ok(Either::Left(self.matches_by_slice(list))),
            None => Ok(Either::Right([].iter())),
        }
    }

    pub fn tournament_yearly_matches_year_range(
        &self,
        tour_id: &TournamentId,
        year_start: &Year,
        year_end: &Year,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        self.check_btreemap_range(year_start, year_end, &self.yearly_match_map)?;
        let tour_year_map = self.get_inner_map(&self.tournament_yearly_match_map, tour_id)?;

        Ok((
            tour_year_map
                .range(year_start..=year_end)
                .fold(0, |acc, list| acc + list.1.len()),
            tour_year_map
                .range(year_start..=year_end)
                .flat_map(|(_, year_matches)| self.matches_by_slice(year_matches)),
        ))
    }

    pub fn team_matches_by_id(
        &self,
        team_id: &TeamId,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        let team_map = self.get_team_home_away_map_season(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;

        Ok((
            tour_map.range(..).fold(0, |acc, sea_map| {
                acc + sea_map.1.range(..).fold(0, |acc, item| acc + item.1.len())
            }),
            tour_map.range(..).flat_map(|(_, sea_map)| {
                sea_map
                    .range(..)
                    .flat_map(|item| self.matches_by_slice(item.1))
            }),
        ))
    }

    pub fn team_matches_by_season_id(
        &self,
        team_id: &TeamId,
        season_id: &SeasonId,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        let team_map = self.get_team_home_away_map_season(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;

        Ok((
            tour_map.range(..).fold(0, |acc, sea_map| {
                acc + sea_map
                    .1
                    .get(season_id)
                    .map(|match_list| acc + match_list.len())
                    .unwrap_or(0)
            }),
            tour_map.range(..).flat_map(|(_, sea_map)| {
                sea_map
                    .get(season_id)
                    .map(|match_list| Either::Left(self.matches_by_slice(match_list)))
                    .unwrap_or(Either::Right([].iter()))
            }),
        ))
    }

    pub fn team_matches_by_year(
        &self,
        team_id: &TeamId,
        year: &Year,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        self.check_year_exists_in_range(year)?;

        let team_map = self.get_team_home_away_map_yearly(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;

        Ok((
            tour_map.range(..).fold(0, |acc, year_map| {
                acc + year_map
                    .1
                    .get(year)
                    .map(|match_list| acc + match_list.len())
                    .unwrap_or(0)
            }),
            tour_map.range(..).flat_map(|(_, sea_map)| {
                sea_map
                    .get(year)
                    .map(|match_list| Either::Left(self.matches_by_slice(match_list)))
                    .unwrap_or(Either::Right([].iter()))
            }),
        ))
    }

    pub fn team_matches_year_range(
        &self,
        team_id: &TeamId,
        year_start: &Year,
        year_end: &Year,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        self.check_year_exists_in_range(year_start)?;
        self.check_year_exists_in_range(year_end)?;

        let team_map = self.get_team_home_away_map_yearly(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;

        Ok((
            tour_map.range(..).fold(0, |acc, (_, year_map)| {
                acc + year_map
                    .range(year_start..=year_end)
                    .fold(0, |acc, (_, match_list)| acc + match_list.len())
            }),
            tour_map.range(..).flat_map(move |(_, year_map)| {
                year_map
                    .range(year_start..=year_end)
                    .flat_map(|(_, match_list)| self.matches_by_slice(match_list))
            }),
        ))
    }

    pub fn team_matches_by_tournament_id(
        &self,
        team_id: &TeamId,
        tour_id: &TournamentId,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        let team_map = self.get_team_home_away_map_season(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;
        let season_map = tour_map.get(tour_id);

        match season_map {
            Some(sea_map) => Ok((
                sea_map.iter().fold(0, |acc, item| acc + item.1.len()),
                Either::Left(
                    sea_map
                        .iter()
                        .flat_map(|(_, match_list)| self.matches_by_slice(match_list)),
                ),
            )),
            None => Ok((0, Either::Right([].iter()))),
        }
    }

    pub fn team_tournament_matches_by_season_id(
        &self,
        team_id: &TeamId,
        tour_id: &TournamentId,
        season_id: &SeasonId,
        home_away: &HomeAwayOption,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        let team_map = self.get_team_home_away_map_season(home_away);
        let tour_map = self.get_inner_map(team_map, team_id)?;
        let season_map = self.get_inner_map(tour_map, tour_id)?;
        let match_list = season_map.get(season_id);

        match match_list {
            Some(list) => Ok(Either::Left(self.matches_by_slice(list))),
            None => Ok(Either::Right([].iter())),
        }
    }

    pub fn team_tournament_matches_by_year(
        &self,
        team_id: &TeamId,
        tour_id: &TournamentId,
        year: &Year,
        home_away: &HomeAwayOption,
    ) -> Result<impl Iterator<Item = &Match>, StatusCode> {
        self.check_year_exists_in_range(year)?;
        let team_map = self.get_team_home_away_map_yearly(home_away);

        team_map
            .get(team_id)
            .map(|tour_map| {
                tour_map
                    .get(tour_id)
                    .map(|year_map| {
                        year_map
                            .get(year)
                            .map(|match_list| Ok(Either::Left(self.matches_by_slice(match_list))))
                            .unwrap_or(Ok(Either::Right([].iter())))
                    })
                    .unwrap_or(Ok(Either::Right([].iter())))
            })
            .unwrap_or(Ok(Either::Right([].iter())))
    }

    pub fn team_tournament_matches_by_year_range(
        &self,
        team_id: &TeamId,
        tour_id: &TournamentId,
        year_start: &Year,
        year_end: &Year,
        home_away: &HomeAwayOption,
    ) -> Result<(usize, impl Iterator<Item = &Match>), StatusCode> {
        self.check_year_exists_in_range(year_start)?;
        self.check_year_exists_in_range(year_end)?;

        let team_map = self.get_team_home_away_map_yearly(home_away);

        team_map
            .get(team_id)
            .map(|tour_map| {
                tour_map
                    .get(tour_id)
                    .map(|year_map| {
                        Ok((
                            year_map
                                .range(year_start..=year_end)
                                .fold(0, |acc, (_, match_list)| acc + match_list.len()),
                            Either::Left(
                                year_map
                                    .range(year_start..=year_end)
                                    .flat_map(|(_, match_list)| self.matches_by_slice(match_list)),
                            ),
                        ))
                    })
                    .unwrap_or(Ok((0, Either::Right([].iter()))))
            })
            .unwrap_or(Ok((0, Either::Right([].iter()))))
    }
}

// Utilities
impl IMDB<ReadyState> {
    fn matches_by_slice(&self, match_list: &[MatchId]) -> impl Iterator<Item = &Match> {
        match_list
            .iter()
            .map(|match_id| self.match_data_map.get(match_id).unwrap())
    }

    fn check_btreemap_range<K, V>(
        &self,
        start: &K,
        end: &K,
        map: &BTreeMap<K, V>,
    ) -> Result<(), StatusCode>
    where
        K: Ord,
    {
        let err = Err(StatusCode::NOT_FOUND);
        if start == end || end < start {
            return err;
        }

        if !map.contains_key(start) && !map.contains_key(end) {
            println!("Check btree range does not contain");
            return err;
        }

        Ok(())
    }

    fn check_year_exists_in_range(&self, year: &Year) -> Result<(), StatusCode> {
        if self.yearly_match_map.contains_key(year) {
            Ok(())
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

    fn get_inner_map<'a, K, V>(&self, map: &'a BTreeMap<K, V>, id: &K) -> Result<&'a V, StatusCode>
    where
        K: Ord,
    {
        map.get(id).ok_or(StatusCode::NOT_FOUND)
    }

    fn get_team_home_away_map_season(
        &self,
        home_away: &HomeAwayOption,
    ) -> &TeamTournamentSeasonMatchMap {
        use HomeAwayOption::*;
        match home_away {
            Both => &self.team_tournament_season_match_map,
            Home => &self.team_home_tournament_season_match_map,
            Away => &self.team_away_tournament_season_match_map,
        }
    }

    fn get_team_home_away_map_yearly(
        &self,
        home_away: &HomeAwayOption,
    ) -> &TeamTournamentYearlyMatchMap {
        use HomeAwayOption::*;
        match home_away {
            Both => &self.team_tournament_yearly_match_map,
            Home => &self.team_home_tournament_yearly_match_map,
            Away => &self.team_away_tournament_yearly_match_map,
        }
    }
}
