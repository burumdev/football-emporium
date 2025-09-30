<script setup lang="ts">
  import { computed } from "vue";
  import { useMatchlistStore } from "@/store/matchlist";
  const ml_store = useMatchlistStore();

  import { PaginationPerPage } from "@/store/storetypes.ts";

  import VueSelect from "vue3-select-component";

  const total_pages = computed(() => {
    return ml_store.pagination_data.total_pages;
  });

  const current_page = computed(() => {
    const { per_page, offset } = ml_store.pagination_data;

    let cp = 1;
    if (offset > 0) {
      if (offset < per_page) {
        cp += 1;
      } else {
        cp = Math.ceil(offset / per_page) + 1;
      }
    }

    return cp;
  });

  const pag_buttons = computed(() => {
    const { per_page, offset, total_pages: tp } = ml_store.pagination_data;
    const cp = current_page.value;

    let pages = 1;
    if (tp < 4) {
      pages = tp;
    } else {
      if (cp < 3) {
        pages = 5 - (3 - cp);
      } else if (cp > tp - 3) {
        pages = tp - (cp - 3);
      } else {
        pages = 5;
      }
    }

    const pages_arr = [];

    for (let i = 0; i < pages; i++) {
      let page_num = 0;
      if (cp < 4) {
        page_num = 1 + i;
      } else {
        page_num = cp - 2 + i;
      }
      pages_arr[i] = page_num;
    }

    return pages_arr;
  });

  const empty_left = computed(() => {
    const cp = current_page.value;
    return cp < 3 ? 3 - cp : 0;
  });

  const empty_right = computed(() => {
    const cp = current_page.value;
    const tp = total_pages.value;

    return cp > tp - 3 ? 2 - (tp - cp) : 0;
  });

  const shown_range = computed(() => {
    const { per_page, offset } = ml_store.pagination_data;
    const upper = offset + per_page > ml_store.total ? ml_store.total : offset + per_page;
    return [offset, upper];
  });

  const on_click_pag_number_button = (but: number) => {
    const cp = current_page.value;

    if (but === cp) {
      return;
    } else {
      const is_forward = cp < but;
      const diff = is_forward ? but - cp : cp - but;
      ml_store.on_paginate_jump(is_forward, diff);
    }
  }
</script>

<template>
  <div class="matchlist-paginator">
    <div class="pag-section font-size-smaller">
      Showing {{ shown_range[0] }}..{{ shown_range[1] }} of {{ ml_store.total }} matches
    </div>
    <div class="pag-section pagination-controls font-size-smaller">
      <div class="pag-buttons-container">
        <button :disabled="current_page === 1" @click="ml_store.on_paginate_jump(false, 5)"><<</button>
        <button :disabled="current_page === 1" @click="ml_store.on_paginate_one(false)"><</button>
        <button v-if="empty_left > 0" v-for="item in empty_left" class="invisible">#</button>
        <button
          v-for="(but, index) in pag_buttons"
          :class="{ active: current_page === but }"
          @click="() => on_click_pag_number_button(but)"
          >{{ but }}</button>
        <button v-if="empty_right > 0" v-for="item in empty_right" class="invisible">#</button>
        <button :disabled="current_page === total_pages" @click="ml_store.on_paginate_one(true)">></button>
        <button :disabled="current_page === total_pages" @click="ml_store.on_paginate_jump(true, 5)">>></button>
      </div>
    </div>
    <div class="pag-section">
      <VueSelect
        class="pagination-ordering"
        v-model="ml_store.pagination_data.per_page"
        placeholder="Per Page"
        :is-clearable="false"
        :options="
          [
            {
              label: '10',
              value: PaginationPerPage.Ten,
            },
            {
              label: '25',
              value: PaginationPerPage.TwentyFive,
            },
            {
              label: '50',
              value: PaginationPerPage.Fifty,
            },
            {
              label: '100',
              value: PaginationPerPage.Hundred,
            },
            {
              label: '250',
              value: PaginationPerPage.TwoHundredFifty,
            }
          ]"
        @option-selected="ml_store.on_select_per_page"
      />
    </div>
  </div>
</template>

<style scoped>
  .matchlist-paginator {
    background: var(--fg-light);
    color: var(--fg-dark);
    padding: 0.75rem;
    border-top-left-radius: 4px;
    border-top-right-radius: 4px;
    font-weight: bold;
    display: flex;
    justify-content: space-between;
    align-items: center;

    div:nth-child(1) {
      justify-content: left;
    }
    div:nth-child(2) {
      justify-content: center;
    }
    div:nth-child(3) {
      justify-content: right;
    }
  }
  .pag-section {
    display: flex;
    flex: 1 1 0;
  }
  .pagination-controls {
    display: flex;
    justify-content: space-between;
    button {
      border: 1px solid green;
      border-radius: 4px;
      cursor: pointer;
      min-width: 30px;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 2px;
    }
    button:disabled {
      opacity: .5;
    }
    button.active {
      background-color: var(--fg-lime);
    }
    div {
      display: flex;
      justify-content: space-between !important;
    }
  }
  .pag-buttons-container {
    min-width: 310px;
  }
  .pagination-ordering {
    width: fit-content !important;
    --vs-width: 250px !important;
    --vs-font-size: var(--font-size-small);
    --vs-min-height: 20px;
    font-size: var(--font-size-small);
    --vs-option-font-size: var(--font-size-small);
  }
</style>
