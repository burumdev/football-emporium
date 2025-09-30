<script setup lang="ts">
  import { computed } from 'vue';
  import type { Match } from '../../store/storetypes.ts';
  const { match } = defineProps<{
    match: Match
  }>();

  const ht_score = computed(() => {
    const team1_score = match.score.half_time ? match.score.half_time[0].toString() : null;
    const team2_score = match.score.half_time ? match.score.half_time[1].toString() : null;
    return team1_score && team2_score ? [team1_score, team2_score] : null;
  });

  const score = computed(() => {
    const team1_score = match.score.full_time ? match.score.full_time[0].toString() : "";
    const team2_score = match.score.full_time ? match.score.full_time[1].toString() : "";
    return [team1_score, team2_score];
  });

  const time = computed(() => {
    return match.time ? match.time.split(":").slice(0, -1).join(":") : null;
  });
</script>

<template>
  <li class="match-item">
    <div class="teams-score">
      <div class="team-container">
        <h4 class="team1">{{ match.team1 }}</h4>
      </div>
      <h4>{{ score[0] }} - {{ score[1] }}</h4>
      <div class="team-container">
        <h4>{{ match.team2 }}</h4>
      </div>
    </div>
    <div class="match-info">
      <h4>{{ match.tournament_name }}</h4>
      <h4 v-if="match.round">{{ `Round: ${match.round}` }}</h4>
      <h4 v-if="match.stage">{{ `Stage: ${match.stage}` }}</h4>
      <h4 v-if="ht_score">{{ `Half-time: ${ht_score[0] + ' - ' + ht_score[1]}` }}</h4>
      <h4>{{ `${match.date}${ time ? ' ' + time : ""}` }}</h4>
    </div>
  </li>
</template>

<style scoped>
  .match-item {
    padding: 25px 0;
    background: var(--item-bg-even);
  }
  .match-item:nth-child(even) {
    background: var(--item-bg-even);
  }
  .match-item:nth-child(odd) {
    background: var(--item-bg-odd);
  }
  .match-info {
    padding: 0 2rem;
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-smaller);
  }
  .teams-score {
    margin-bottom: 1rem;
    font-size: var(--font-size-big);
    font-weight: bold;
    display: flex;
    justify-content: space-between;
  }
  .team-container {
    width: 45%;
  }
  .team1 {
    text-align: right;
  }
</style>
