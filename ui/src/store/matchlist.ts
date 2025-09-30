import { defineStore } from "pinia";
import type { Option } from "vue3-select-component";

import type { IMatchListState, Match } from "./storetypes.ts";
import { HomeAway, PaginationPerPage } from "./storetypes.ts";

export const useMatchlistStore = defineStore("matchlist", {
  state: (): IMatchListState => {
    return {
      is_loading: false,
      list: [],
      total: 0,
      seasons: [],
      tournaments: [],
      teams: [],
      years: [],
      filter_data: {
        home_away: HomeAway.Both,
      },
      pagination_data: {
        total_pages: 1,
        offset: 0,
        per_page: PaginationPerPage.Ten,
      },
    }
  },
  actions: {
    async fetch_metadata() {
      try {
        Promise.all([fetch(`${__API_URL__}/seasons`), fetch(`${__API_URL__}/tournaments`), fetch(`${__API_URL__}/teams`)])
          .then(async results => {
            this.seasons = await results[0].json();
            this.tournaments = await results[1].json();
            this.teams = await results[2].json();
          }).then(() => {
            this.years = [...new Set(this.seasons.flatMap(sea => sea.end_year ? [sea.start_year, sea.end_year] : sea.start_year))];
            this.fetch_matchlist();
          });
      } catch (err) {
        const { message } = err as Error;
        console.error(message);
      }
    },
    async fetch_matchlist() {
      let url = __API_URL__;

      const { filter_data: fil } = this;
      const { pagination_data: pag } = this;

      if (this.has_no_filters()) {
        url += "/all_matches";
      } else {
        if (fil.team_id) {
          url += `/teams/${fil.team_id}`;
        }
        if (fil.tournament_id) {
          url += `/tournaments/${fil.tournament_id}`;
        }
        if (fil.season_id) {
          url += `/seasons/${fil.season_id}`;
        } else if (fil.from_year) {
          url += `/years/${fil.from_year}`
          if (fil.to_year) {
            url += `/${fil.to_year}`;
          }
        }
      }

      let pag_query = `?offset=${pag.offset}&per_page=${pag.per_page}`;
      if (fil.team_id) {
        pag_query += `&home_away=${fil.home_away}`;
      }

      this.is_loading = true;

      try {
        const response = await fetch(`${url}${pag_query}`);
        const json = await response.json();
        this.list = this.add_tournament_name_to_matches(json.list);
        this.total = json.total;
        this.update_pagination_data();
        this.is_loading = false;
      } catch (err) {
        const { message } = err as Error;
        console.error(message);
        this.is_loading = false;
      }
    },
    add_tournament_name_to_matches(list: Match[]): Match[] {
      return list.map(
        (match: Match): Match => (
          {
            ...match,
            tournament_name: this.tournaments.find(tour => tour.id === match.tournament_id)?.name || "Tournament Name N/A"
          }
        ));
    },
    reset_paginator_and_fetch() {
      this.reset_paginator();
      this.fetch_matchlist();
    },
    on_select_season(season: Option<number>) {
      this.filter_data.season_id = season.value;
      this.filter_data.from_year = undefined;
      this.filter_data.to_year = undefined;
      this.reset_paginator_and_fetch();
    },
    on_select_from_year(from_year: Option<number>) {
      this.filter_data.from_year = from_year.value;
      this.filter_data.season_id = undefined;
      this.reset_paginator_and_fetch();
    },
    on_deselect_from_year() {
      this.filter_data.to_year = undefined;
      this.reset_paginator_and_fetch();
    },
    on_select_to_year() {
      this.filter_data.season_id = undefined;
      this.reset_paginator_and_fetch();
    },
    on_select_per_page() {
      this.update_pagination_data();
      this.fetch_matchlist();
    },
    on_paginate_one(is_forward: boolean) {
      this.pagination_data.offset = this.get_normalized_offset(is_forward, 1);
      this.fetch_matchlist();
    },
    on_paginate_jump(is_forward: boolean, multiplier: number) {
      this.pagination_data.offset = this.get_normalized_offset(is_forward, multiplier);
      this.fetch_matchlist();
    },
    get_normalized_offset(is_forward: boolean, multiplier: number = 1) {
      const { per_page, offset } = this.pagination_data;
      const { total } = this;
      const amount = per_page * multiplier;
      if (is_forward) {
        if (offset + amount > total) {
          let new_offset = offset + (Math.floor((total - offset) / per_page)) * per_page;
          if (new_offset === total) {
            new_offset -= per_page;
          }

          return new_offset;
        } else {
          return offset + amount;
        }
      } else {
        if (offset - amount < 0) {
          return 0;
        } else {
          return offset - amount;
        }
      }
    },
    update_pagination_data() {
      const { per_page, offset } = this.pagination_data;
      const { total } = this;
      let total_pages;
      if (total === 0) {
        total_pages = 1;
      } else {
        total_pages = offset % per_page > 0 ? 1 : 0;
        total_pages += Math.ceil((total - offset % per_page) / per_page);
      }

      this.pagination_data.total_pages = total_pages;
    },
    has_no_filters() {
      return Object
        .entries(this.filter_data)
        .filter(([key, _]) => !["to_year", "home_away"].includes(key))
        .every(([_, value]) => value === undefined);
    },
    reset_paginator() {
      this.pagination_data.offset = 0;
      this.pagination_data.total_pages = 1;
    },
    reset_list() {
      this.list = [];
      this.total = 0;
    }
  },
  getters: {
    filter_seasons: (state): Option<number>[] =>
      state.seasons
        .map(sea => (
          {
            label: `${sea.start_year}${sea.end_year ? '-' + sea.end_year : ''}`,
            value: sea.id,
          }
        )),
    filter_tournaments: (state): Option<number>[] =>
      state.tournaments
        .map(tour => (
          {
            label: tour.name,
            value: tour.id,
          }
        )),
    filter_teams: (state): Option<number>[] =>
      state.teams
        .map(team => (
          {
            label: team.name,
            value: team.id,
          }
        )),
    filter_from_years: (state): Option<number>[] =>
      state.years
        .map(year => (
          {
            label: year.toString(),
            value: year,
          }
        ))
        .filter(year => state.filter_data.to_year ? year.value < state.filter_data.to_year : true),
    filter_to_years: (state): Option<number>[] =>
      state.years
        .map(year => (
          {
            label: year.toString(),
            value: year,
          }
        ))
        .filter(year => state.filter_data.from_year ? year.value > state.filter_data.from_year : year.value !== state.years[0]),
  }
});
