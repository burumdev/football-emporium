use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::time::Instant;

use anyhow::Context;
use chrono::Datelike;
use rayon::prelude::*;

pub mod data_types;
mod json_fetcher;
mod db_api;

use data_types::{
    Match, MatchId, MatchDataMap, MatchList, Season, SeasonId, SeasonMap, SeasonMatchMap, TournamentId,
    TournamentIdNameMap, TournamentNameIdMap, TournamentMatchMap, Year, YearlyMatchMap, TournamentSeasonMatchMap, TournamentYearlyMatchMap, TeamId,
    TeamIdNameMap, TeamNameIdMap, TeamTournamentYearlyMatchMap, TeamTournamentSeasonMatchMap,
};
use json_fetcher::fetch_json_raw_data;
use json_fetcher::{JsonFileContentsRaw, JsonFilesContentsAllRaw};

use crate::constants::ERR_PFX;
const MOD: &str = "IMDB";

#[derive(Debug)]
pub enum IMDBError {
    NoDataAvailable,
    DataIntegrity,
    FolderNameMalformed,
}
impl Error for IMDBError {}
impl Display for IMDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IMDBError::*;
        match self {
            NoDataAvailable => write!(f, "Could not parse any useful data from json files."),
            DataIntegrity => write!(
                f,
                "One or more data structures are malformed. Lengths didn't match.",
            ),
            FolderNameMalformed => {
                write!(f, "Folder name should be in the format: 2015-16 or 2015.")
            }
        }
    }
}

pub trait IMDBState {}
pub struct InitState;
pub struct ReadyState;

impl IMDBState for InitState {}
impl IMDBState for ReadyState {}

pub type IMDBReady = Arc<IMDB<ReadyState>>;

#[allow(clippy::upper_case_acronyms)]
pub struct IMDB<S: IMDBState = InitState> {
    season_id_head: SeasonId,
    tournament_id_head: TournamentId,
    team_id_head: TeamId,
    match_id_head: AtomicUsize,
    season_map: SeasonMap,
    match_data_map: MatchDataMap,
    season_match_map: SeasonMatchMap,
    yearly_match_map: YearlyMatchMap,
    tournament_id_name_map: TournamentIdNameMap,
    tournament_name_id_map: TournamentNameIdMap,
    tournament_match_map: TournamentMatchMap,
    tournament_season_match_map: TournamentSeasonMatchMap,
    tournament_yearly_match_map: TournamentYearlyMatchMap,
    team_id_name_map: TeamIdNameMap,
    team_name_id_map: TeamNameIdMap,
    team_tournament_season_match_map: TeamTournamentSeasonMatchMap,
    team_tournament_yearly_match_map: TeamTournamentYearlyMatchMap,
    team_home_tournament_season_match_map: TeamTournamentSeasonMatchMap,
    team_away_tournament_season_match_map: TeamTournamentSeasonMatchMap,
    team_home_tournament_yearly_match_map: TeamTournamentYearlyMatchMap,
    team_away_tournament_yearly_match_map: TeamTournamentYearlyMatchMap,
    _phantom: PhantomData<S>,
}

impl IMDB<InitState> {
    pub async fn init() -> anyhow::Result<IMDB<ReadyState>> {
        let raw_data = fetch_json_raw_data().await?;

        let me = Self {
            season_id_head: 1,
            tournament_id_head: 1,
            match_id_head: AtomicUsize::new(1),
            team_id_head: 1,
            season_map: SeasonMap::new(),
            match_data_map: MatchDataMap::new(),
            season_match_map: SeasonMatchMap::new(),
            yearly_match_map: YearlyMatchMap::new(),
            tournament_id_name_map: TournamentIdNameMap::new(),
            tournament_name_id_map: TournamentNameIdMap::new(),
            tournament_match_map: TournamentMatchMap::new(),
            tournament_season_match_map: TournamentSeasonMatchMap::new(),
            tournament_yearly_match_map: TournamentYearlyMatchMap::new(),
            team_id_name_map: TeamIdNameMap::new(),
            team_name_id_map: TeamNameIdMap::new(),
            team_tournament_season_match_map: TeamTournamentSeasonMatchMap::new(),
            team_tournament_yearly_match_map: TeamTournamentYearlyMatchMap::new(),
            team_home_tournament_season_match_map: TeamTournamentSeasonMatchMap::new(),
            team_away_tournament_season_match_map: TeamTournamentSeasonMatchMap::new(),
            team_home_tournament_yearly_match_map: TeamTournamentYearlyMatchMap::new(),
            team_away_tournament_yearly_match_map: TeamTournamentYearlyMatchMap::new(),
            _phantom: PhantomData,
        };

        let me = Self::build(me, raw_data)?;

        Ok(me)
    }

    fn build(mut me: Self, raw_data: JsonFilesContentsAllRaw) -> anyhow::Result<IMDB<ReadyState>> {
        const ERR_FN: &str = "::build";

        println!("{MOD}: building in memory database.");
        let now = Instant::now();

        for (folder, files) in raw_data.into_iter() {
            let season_result = Self::per_folder_season(&folder, &mut me.season_map, &mut me.season_id_head);

            let Ok(season_id) = season_result else {
                eprintln!(
                    "{ERR_PFX} {MOD}{ERR_FN}: Could not parse folder name '{folder}' as a season: {} is the reason. Continuing without this folder...",
                    season_result.unwrap_err()
                );
                continue;
            };

            let mut matchlists = Self::per_folder_parse_contents_as_matchlists(
                &me.match_id_head,
                &folder,
                &files,
                season_id,
            );

            if matchlists.is_empty() {
                eprintln!(
                    "{ERR_PFX} {ERR_FN}: Could not parse any matches from '{folder}'. Continuing without this folder..."
                );
                continue;
            }

            matchlists
                .par_sort_unstable_by(
                    |first, second| first.matches.first().unwrap().id.cmp(&second.matches.first().unwrap().id)
                );
            let match_ids: Vec<MatchId> = matchlists
                .iter()
                .flat_map(|match_list|
                    match_list.matches
                        .iter()
                        .map(|mch| mch.id)
                ).collect();

            me.season_match_map.extend(Self::per_folder_season_match_map(match_ids, season_id));

            matchlists
                .into_iter()
                .for_each(|match_list| {
                    Self::per_matchlist_build(
                        match_list,
                        season_id,
                        &mut me,
                    );
                });
        }

        println!(
            "{MOD}: build ended with elapsed milliseconds: {}",
            now.elapsed().as_millis()
        );

        if me.match_data_map.is_empty() {
            return Err(IMDBError::NoDataAvailable.into());
        }

        Self::check_data_integrity(&me)?;

        Ok(Self::ready(me))
    }

    fn check_data_integrity(me: &Self) -> anyhow::Result<()> {
        let hashmap_len = me.match_data_map.len();

        let season_len = me
            .season_match_map
            .iter()
            .fold(0, |acc, item| acc + item.1.len());

        let yearly_len = me
            .yearly_match_map
            .iter()
            .fold(0, |acc, item| acc + item.1.len());

        let tournament_len = me
            .tournament_match_map
            .iter()
            .fold(0, |acc, item| acc + item.1.len());

        let tournament_season_len = me
            .tournament_season_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1.len()
                    )
            );

        let tournament_yearly_len = me
            .tournament_yearly_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1.len()
                    )
            );

        let team_tournament_season_len = me
            .team_tournament_season_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );

        let team_tournament_yearly_len = me
            .team_tournament_yearly_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );

        let team_home_tournament_season_len = me
            .team_home_tournament_season_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );

        let team_home_tournament_yearly_len = me
            .team_home_tournament_yearly_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );

        let team_away_tournament_season_len = me
            .team_away_tournament_season_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );

        let team_away_tournament_yearly_len = me
            .team_away_tournament_yearly_match_map
            .iter()
            .fold(0,
                |acc, item| acc + item.1
                    .iter()
                    .fold(0,
                        |acc, item| acc + item.1
                            .iter()
                            .fold(0,
                                |acc, item| acc + item.1.len()
                            )
                    )
            );
        if hashmap_len != yearly_len ||
            hashmap_len != season_len ||
            hashmap_len != tournament_len ||
            hashmap_len != tournament_season_len ||
            hashmap_len != tournament_yearly_len ||
            hashmap_len != team_tournament_season_len / 2 ||
            hashmap_len != team_tournament_yearly_len / 2 ||
            hashmap_len != team_home_tournament_season_len ||
            hashmap_len != team_away_tournament_season_len ||
            hashmap_len != team_home_tournament_yearly_len ||
            hashmap_len != team_away_tournament_yearly_len
        {
            return Err(IMDBError::DataIntegrity).with_context(|| {
                format!(
                    "Hashmap length: {}\n\
                    Yearly map length: {}\n\
                    Season map length: {}\n\
                    Tournament map length: {}\n\
                    Tournament season map length: {}\n\
                    Tournament yearly map length: {}\n\
                    Team tournament season map length: {}\n\
                    Team tournament yearly map length: {}\n\
                    Team home tournament season map length: {}\n\
                    Team away tournament season map length: {}\n\
                    Team home tournament yearly map length: {}\n\
                    Team away tournament yearly map length: {}\n\
                    one of these lengths is wrong.",
                    hashmap_len,
                    yearly_len,
                    season_len,
                    tournament_len,
                    tournament_season_len,
                    tournament_yearly_len,
                    team_tournament_season_len,
                    team_tournament_yearly_len,
                    team_home_tournament_season_len,
                    team_away_tournament_season_len,
                    team_home_tournament_yearly_len,
                    team_away_tournament_yearly_len
                )
            });
        }

        println!("{MOD}: {} matches from {} tournaments with {} teams added to the database.", hashmap_len, me.tournament_id_name_map.len(), me.team_id_name_map.len());

        Ok(())
    }

    fn ready(me: IMDB<InitState>) -> IMDB<ReadyState> {
        let IMDB::<InitState> {
            season_id_head,
            tournament_id_head,
            match_id_head,
            team_id_head,
            season_map,
            match_data_map,
            season_match_map,
            yearly_match_map,
            tournament_id_name_map,
            tournament_name_id_map,
            tournament_match_map,
            tournament_season_match_map,
            tournament_yearly_match_map,
            team_id_name_map,
            team_name_id_map,
            team_tournament_season_match_map,
            team_tournament_yearly_match_map,
            team_home_tournament_season_match_map,
            team_away_tournament_season_match_map,
            team_home_tournament_yearly_match_map,
            team_away_tournament_yearly_match_map,
            _phantom,
        } = me;

        IMDB::<ReadyState> {
            season_id_head,
            tournament_id_head,
            match_id_head,
            team_id_head,
            season_map,
            match_data_map,
            season_match_map,
            yearly_match_map,
            tournament_id_name_map,
            tournament_name_id_map,
            tournament_match_map,
            tournament_season_match_map,
            tournament_yearly_match_map,
            team_id_name_map,
            team_name_id_map,
            team_tournament_season_match_map,
            team_tournament_yearly_match_map,
            team_home_tournament_season_match_map,
            team_away_tournament_season_match_map,
            team_home_tournament_yearly_match_map,
            team_away_tournament_yearly_match_map,
            _phantom: PhantomData,
        }
    }

    fn get_tournament_name(matchlist_name: &str) -> String {
        let mut name_split = matchlist_name.split(" ").collect::<Vec<_>>();
        let _ = name_split.pop();

        name_split.join(" ")
    }

    fn per_folder_season(
        folder: &str,
        season_map: &mut SeasonMap,
        season_id_head: &mut SeasonId,
    ) -> anyhow::Result<SeasonId> {
        const ERR_FN: &str = "::per_folder_season";
        let folder_split = folder.split("-").collect::<Vec<_>>();

        let start_year = folder_split.first();

        match start_year {
            None => {
                Err(IMDBError::FolderNameMalformed)
                .with_context(|| format!("{ERR_PFX} {MOD}{ERR_FN}: Could not parse folder name '{folder}' as a season. Check delimiter, it should be a '-' like in '2015-16'."))
            }
            Some(start_year) => {
                let Ok(start_year) = start_year.parse::<Year>() else {
                    return Err(IMDBError::FolderNameMalformed).with_context(|| 
                        format!(
                            "{ERR_PFX} {MOD}{ERR_FN}: Season start year could not be parsed from folder name token '{start_year}' of folder name '{folder}'. Token should be an integer."
                        )
                    );
                };

                let mut end_year = None;
                if let Some(ey) = folder_split.get(1) {
                    end_year = if let Ok(ey) = ey.parse::<Year>() {
                        // We don't check for the edge case where start_year is 2999 and end_year
                        // should be 3000, because end_year data in json files have two digits.
                        let year_millenia = (start_year / 1000) * 1000;

                        let full_ey = ey + year_millenia;

                        Some(full_ey)
                    } else {
                        return Err(IMDBError::FolderNameMalformed).with_context(|| 
                            format!(
                                "{ERR_PFX} {MOD}{ERR_FN}: Season end year could not be parsed from folder name token '{ey}'. Token should be an integer."
                            )
                        );
                    }
                }

                let id = *season_id_head;

                season_map.insert(
                    *season_id_head,
                    Season {
                        id,
                        start_year,
                        end_year,
                    },
                );

                *season_id_head += 1;

                Ok(id)
            }
        }
    }

    fn per_folder_parse_contents_as_matchlists(
        id_head: &AtomicUsize,
        folder: &str,
        files: &JsonFileContentsRaw,
        season_id: SeasonId,
    ) -> Vec<MatchList> {
        const ERR_FN: &str = "::per_folder_parse_contents_as_matchlists";

        files.par_iter().filter_map(|(fname, contents)| {
            let result: Result<MatchList, _> = serde_json::from_str(contents);

            match result {
                Ok(mut list) => {
                    list.matches.iter_mut().for_each(|mch| {
                        mch.id = id_head.fetch_add(1, Ordering::AcqRel);
                        mch.season_id = season_id;
                    });

                    Some(list)
                },
                Err(err) => {
                    eprintln!("{ERR_PFX} {MOD}{ERR_FN}: Error while parsing file '{folder}/{fname}': {err} Continuing without it...");

                    None
                }
            }
        }).collect()
    }

    fn per_folder_season_match_map(match_ids: Vec<MatchId>, season_id: SeasonId) -> SeasonMatchMap {
        SeasonMatchMap::from([(season_id, match_ids)])
    }

    fn per_matchlist_build(
        match_list: MatchList,
        season_id: SeasonId,
        me: &mut Self,
    ) {
        // It should be safe to unwrap the season here
        // Because we got the ID from the upper loop that creates the season.
        let season = me.season_map.get(&season_id).unwrap();
        let start_year = season.start_year;
        let maybe_end_year = season.end_year;
        let match_ids_start = match_list.matches.iter().filter(|mch| mch.date.year() as u32 == start_year).map(|mch| mch.id).collect::<Vec<_>>();

        Self::per_matchlist_yearly_map(&match_ids_start, &mut me.yearly_match_map, start_year);

        let match_ids_end = if let Some(end_year) = maybe_end_year {
            let mids_end = match_list.matches.iter().filter(|mch| mch.date.year() as u32 == end_year).map(|mch| mch.id).collect::<Vec<_>>();
            Self::per_matchlist_yearly_map(&mids_end, &mut me.yearly_match_map, end_year);

            mids_end
        } else {
            vec![]
        };

        let match_ids_all = match_list.matches.iter().map(|mch| mch.id).collect::<Vec<_>>();

        Self::per_matchlist_tournament_maps(
            me,
            match_list,
            season_id,
            (
                &match_ids_start,
                &match_ids_end,
                &match_ids_all,
            ),
            (
                start_year,
                maybe_end_year
            )
        );
    }

    fn per_matchlist_tournament_maps(
        me: &mut Self,
        match_list: MatchList,
        season_id: SeasonId,
        match_ids: (&[MatchId], &[MatchId], &[MatchId]),
        years: (Year, Option<Year>)
    ) {
        let (match_ids_start, match_ids_end, match_ids_all) = match_ids;
        let (start_year, maybe_end_year) = years;

        let tour_name = Self::get_tournament_name(&match_list.name);
        let tournament_id = if let Some(tournament_id) = me.tournament_name_id_map.get(&tour_name) {
            *tournament_id
        } else {
            let tournament_id = me.tournament_id_head;

            me.tournament_name_id_map.insert(tour_name.clone(), tournament_id);
            me.tournament_id_name_map.insert(tournament_id, tour_name);

            me.tournament_id_head += 1;

            tournament_id
        };

        me.tournament_match_map
            .entry(tournament_id)
            .and_modify(|list| list.extend_from_slice(match_ids_all))
            .or_insert(Vec::from(match_ids_all));

        match_list.matches.into_iter().for_each(|mut mch| {
            mch.tournament_id = tournament_id;
            Self::per_match_team_maps(
                me,
                &mch,
                tournament_id,
                season_id,
                start_year,
                maybe_end_year,
            );
            me.match_data_map.insert(mch.id, mch);
        });

        me.tournament_season_match_map
            .entry(tournament_id)
            .and_modify(|map| {
                map
                    .entry(season_id)
                    .and_modify(|list| list.extend_from_slice(match_ids_all))
                    .or_insert(Vec::from(match_ids_all));
            })
            .or_insert(SeasonMatchMap::from([(season_id, Vec::from(match_ids_all))]));

        me.tournament_yearly_match_map
            .entry(tournament_id)
            .and_modify(|map| {
                map
                .entry(start_year)
                    .and_modify(|list| list.extend_from_slice(match_ids_start))
                    .or_insert(Vec::from(match_ids_start));
            })
            .or_insert(YearlyMatchMap::from([(start_year, Vec::from(match_ids_start))]));

        if let Some(end_year) = maybe_end_year {
            me.tournament_yearly_match_map
                .entry(tournament_id)
                .and_modify(|map| {
                    map
                    .entry(end_year)
                        .and_modify(|list| list.extend_from_slice(match_ids_end))
                        .or_insert(Vec::from(match_ids_end));
                })
                .or_insert(YearlyMatchMap::from([(end_year, Vec::from(match_ids_end))]));
        }
    }

    fn per_matchlist_yearly_map(
        match_ids: &[MatchId],
        yearly_match_map: &mut YearlyMatchMap,
        year: Year,
    ) {
        yearly_match_map
            .entry(year)
            .and_modify(|value| value.extend_from_slice(match_ids))
            .or_insert(match_ids.to_vec());
    }

    fn per_match_team_maps(
        me: &mut Self,
        mch: &Match,
        tournament_id: TournamentId,
        season_id: SeasonId,
        start_year: Year,
        maybe_end_year: Option<Year>,
    ) {
        let team_names = [mch.team1.clone(), mch.team2.clone()];
        for (index, team_name) in team_names.into_iter().enumerate() {
            let team_id = if let Some(team_id) = me.team_name_id_map.get(&team_name) {
                *team_id
            } else {
                let team_id = me.team_id_head;

                me.team_name_id_map.insert(team_name.clone(), team_id);
                me.team_id_name_map.insert(team_id, team_name);

                me.team_id_head += 1;

                team_id
            };

            let update_maps = |season_map: &mut TeamTournamentSeasonMatchMap, yearly_map: &mut TeamTournamentYearlyMatchMap| {
                season_map
                    .entry(team_id)
                    .and_modify(|map| {
                        map
                        .entry(tournament_id)
                            .and_modify(|tmap| {
                                tmap
                                .entry(season_id)
                                    .and_modify(|list| list.push(mch.id))
                                    .or_insert(vec![mch.id]);
                            })
                            .or_insert(SeasonMatchMap::from([(season_id, vec![mch.id])]));
                    })
                    .or_insert(TournamentSeasonMatchMap::from([(tournament_id, SeasonMatchMap::from([(season_id, vec![mch.id])]))]));

                if mch.date.year() as u32 == start_year {
                    yearly_map
                        .entry(team_id)
                            .and_modify(|map| {
                                map
                                .entry(tournament_id)
                                    .and_modify(|tmap| {
                                        tmap
                                        .entry(start_year)
                                            .and_modify(|list| list.push(mch.id))
                                            .or_insert(vec![mch.id]);
                                    })
                                    .or_insert(YearlyMatchMap::from([(start_year, vec![mch.id])]));
                            })
                            .or_insert(TournamentYearlyMatchMap::from([(tournament_id, YearlyMatchMap::from([(start_year, vec![mch.id])]))]));
                }

                if let Some(end_year) = maybe_end_year && mch.date.year() as u32 == end_year {
                    yearly_map
                    .entry(team_id)
                        .and_modify(|map| {
                            map
                            .entry(tournament_id)
                                .and_modify(|tmap| {
                                    tmap
                                    .entry(end_year)
                                        .and_modify(|list| list.push(mch.id))
                                        .or_insert(vec![mch.id]);
                                })
                                .or_insert(YearlyMatchMap::from([(end_year, vec![mch.id])]));
                        })
                        .or_insert(TournamentYearlyMatchMap::from([(tournament_id, YearlyMatchMap::from([(end_year, vec![mch.id])]))]));
                }
            };

            update_maps(&mut me.team_tournament_season_match_map, &mut me.team_tournament_yearly_match_map);

            if index == 0 {
                update_maps(&mut me.team_home_tournament_season_match_map, &mut me.team_home_tournament_yearly_match_map);
            } else {
                update_maps(&mut me.team_away_tournament_season_match_map, &mut me.team_away_tournament_yearly_match_map);
            };
        }
    }
}
