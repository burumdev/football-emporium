<script setup lang="ts">
  import { useMatchlistStore } from "@/store/matchlist";
  const ml_store=useMatchlistStore();

  import { HomeAway } from "@/store/storetypes";

  import VueSelect from "vue3-select-component";
</script>

<template>
  <div class="matchlist-filter">
    <div class="filter-element">
      <label>Season</label>
      <VueSelect
        v-model="ml_store.filter_data.season_id"
        placeholder="Select Season"
        :options="ml_store.filter_seasons"
        @option-selected="ml_store.on_select_season"
        @option-deselected="ml_store.reset_paginator_and_fetch"
      />
    </div>
    <div class="filter-element">
      <label>Year</label>
      <VueSelect
        v-model="ml_store.filter_data.from_year"
        placeholder="Select Year"
        :options="ml_store.filter_from_years"
        @option-selected="ml_store.on_select_from_year"
        @option-deselected="ml_store.on_deselect_from_year"
      />
    </div>
    <div class="filter-element">
      <label>To Year</label>
      <VueSelect
        v-model="ml_store.filter_data.to_year"
        :options="ml_store.filter_to_years"
        placeholder="Select To Year"
        @option-selected="ml_store.on_select_to_year"
        @option-deselected="ml_store.reset_paginator_and_fetch"
      />
    </div>
    <div class="filter-element">
      <label>Tournament</label>
      <VueSelect
        v-model="ml_store.filter_data.tournament_id"
        placeholder="Select Tournament"
        :options="ml_store.filter_tournaments"
        @option-selected="ml_store.reset_paginator_and_fetch"
        @option-deselected="ml_store.reset_paginator_and_fetch"
      />
    </div>
    <div class="filter-element">
      <label>Team</label>
      <VueSelect
        v-model="ml_store.filter_data.team_id"
        placeholder="Select Team"
        :options="ml_store.filter_teams"
        @option-selected="ml_store.reset_paginator_and_fetch"
        @option-deselected="ml_store.reset_paginator_and_fetch"
      />
    </div>
    <div class="filter-element">
      <label>Home/Away</label>
      <VueSelect
        v-model="ml_store.filter_data.home_away"
        placeholder="Select Home or Away"
        @option-selected="ml_store.reset_paginator_and_fetch"
        :is-clearable="false"
        :options="
          [
            {
              label: 'Both',
              value: HomeAway.Both,
            },
            {
              label: 'Home',
              value: HomeAway.Home,
            },
            {
              label: 'Away',
              value: HomeAway.Away,
            },
          ]"
      />
    </div>
  </div>
</template>

<style scoped>
  .matchlist-filter {
    width: 100%;
    margin-top: 1.2rem;
    display: grid;
    gap: 1rem;
    justify-content: space-between;
    grid-template-columns: 1fr 1fr 1fr;
    --vs-background-color: var(--fg-light);
  }
  .filter-element {
    label {
      font-size: var(--font-size-smaller);
      font-weight: bold;
    }
    width: 100%;
  }
  .vue-select:has(.has-value) {
    --vs-background-color: var(--fg-lime);
  }
</style>
