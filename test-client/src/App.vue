<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { onMounted, reactive } from 'vue';
import client from './axios/client';
import router from './pageRouter';

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
  <div>
    <template v-if="!state.isLogined">
      <router-link to="/login">Login</router-link> | 
      <router-link to="/reg">Register</router-link>
    </template>
    <template v-else>
      <router-link to="/">Home</router-link> | 
      <router-link to="/send">Send Post</router-link> | 
      <router-link to="/my_post">My Posts</router-link>
    </template>
    <router-view @auth="onAuth"></router-view>
  </div>
</template>