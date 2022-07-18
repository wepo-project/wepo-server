<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core";
import { reactive } from "vue";
import client from "../axios/client";
import Post from "./Post.vue";

const state = reactive({
  list: <any>[],
  page: 0,
  next: true,
});

const getNextPage = async () => {
  state.page = state.page + 1;
  const resp = await client.get("post", "browse", {
    params: {
      page: state.page,
    },
  });
  state.next = resp.data.next;
  state.page = resp.data.next;
  state.list = resp.data.list;
};

onMounted(async () => {
  await getNextPage();
});

</script>

<template>
  <div>
    <div v-for="(item) in state.list" :key="item.id">
      <Post :item="item"></Post>
    </div>
    <div v-if="!state.list || !state.list.length">
      It's empty here
    </div>
  </div>
</template>