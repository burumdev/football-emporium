export enum HomeAway {
  Both = "both",
  Home = "home",
  Away = "away",
}

export enum PaginationPerPage {
  Ten = 10,
  TwentyFive = 25,
  Fifty = 50,
  Hundred = 100,
  TwoHundredFifty = 250,
}

export type Season = {
  id: number,
  start_year: number,
  end_year: number,
}

export type Tournament = {
  id: number,
  name: string,
}

export type Team = {
  id: number,
  name: string,
}

export type Score = {
  half_time?: [number, number],
  full_time?: [number, number],
}

export type Match = {
  id: number,
  season_id: number,
  tournament_id: number,
  tournament_name: string,
  round?: string,
  date: string,
  time?: string,
  team1: string,
  team2: string,
  score: Score,
  stage?: string,
}

export type FilterSeason = {
  label: string,
  value: number,
  disabled?: boolean,
}

export type FilterYear = {
  label: number,
  value: number,
  disabled?: boolean,
}

export type FilterTournament = {
  label: string,
  value: number,
  disabled?: boolean,
}

export type FilterData = {
  season_id?: number,
  tournament_id?: number,
  team_id?: number,
  from_year?: number,
  to_year?: number,
  home_away: HomeAway,
}

export type PaginationData = {
  total_pages: number,
  offset: number,
  per_page: number,
}

export interface IMatchListState {
  is_loading: boolean,
  list: Match[],
  total: number,
  seasons: Season[],
  tournaments: Tournament[],
  teams: Team[],
  selected_season?: Season,
  years: number[],
  filter_data: FilterData,
  pagination_data: PaginationData,
}
