<script setup lang="ts">
  import { computed } from "vue";

  import { useMatchlistStore } from "@/store/matchlist";
  const ml_store = useMatchlistStore();

  import Loading from "@/components/loading/Loading.vue";
  import Paginator from "./Paginator.vue";
  import MatchItem from "./MatchItem.vue";
</script>

<template>
  <div class="matchlist-container">
    <Paginator />
    <div class="matchlist-wrapper">
      <Loading v-if="ml_store.is_loading" />
      <div v-if="ml_store.list.length === 0" class="no-data">
        <h2>No results matched the combination of filter selections you provided. Try again with different selections.</h2>
      </div>
      <ul class="list-ul">
        <MatchItem
          v-for="match in ml_store.list"
          :match="match"
        />
      </ul>
    </div>
  </div>
</template>

<style scoped>
  .matchlist-container {
    margin-top: 3rem;
  }
  .matchlist-wrapper {
    position: relative;
  }
  .no-data {
    margin: 3rem;
    display: flex;
    justify-content: center;
    text-align: center;
  }
</style>
