<script setup lang="ts">
  import { useMatchlistStore } from "@/store/matchlist";
  const ml_store = useMatchlistStore();

  import { onBeforeMount } from "vue";
  import Filter from "./Filter.vue";
  import List from "./List.vue";

  onBeforeMount(() => {
    ml_store.fetch_metadata();
  });
</script>

<template>
  <Filter />
  <List />
</template>

<style scoped>
</style>
