<script setup lang="ts">
  import MatchList from "./components/matchsearch/MatchSearch.vue";
</script>

<template>
  <main class="container-main">
    <section class="container-content">
      <div class="app-header">
        <h1>Football Emporium</h1>
        <p>by <b>Barış Ürüm</b> a.k.a. <b>burumdev</b></p>
      </div>
      <div class="app-info">
        <p>Powered by <b>Rust</b> and <b>Axum</b> on backend and <b>Vue.js</b> on the frontend.</p>
        <p>Special thanks to <b>OpenFootball</b> contributors for the public domain data. You folks are awesome!</p>
      </div>
      <MatchList />
    </section>
  </main>
</template>

<style scoped>
  .app-info {
    margin-top: 1rem;
  }
  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .container-main {
    width: 100%;
    background: url(/img/grass.webp);
    display: flex;
    justify-content: center;
    color: var(--fg-light);
  }
  .container-content {
    background: rgba(0, 75, 40, 0.85);
    margin: 4rem 0;
    padding: 1rem;
    width: var(--desktop-width);
  }
</style>
