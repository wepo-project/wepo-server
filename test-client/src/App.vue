<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router';
import { onMounted, reactive } from 'vue';
import client from './axios/client';
import router from './pageRouter';
import Tabbar from './components/Tabbar.vue';
const route = useRoute()

const state = reactive({
  isLogined: false,
  // loaded: false,
})
onMounted(async () => {
  let succ = false;
  try {
    succ = await client.loginWithToken();
  } catch(e) { console.error(e) }
  if (!succ) {
    router.push('/login')
  } else {
    // router.push('/')
  }
  state.isLogined = client.isLogined();
})
const onAuth = () => {
  state.isLogined = true;
}
</script>

<template>
  <div class="flex flex-col h-screen">
    <router-view @auth="onAuth" class="flex-auto overflow-scroll hidden-scrollbar" :key="route.fullPath"></router-view>
    <tabbar v-if="state.isLogined"></tabbar>
  </div>
</template>