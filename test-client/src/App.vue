<script>
import Home from './components/Home.vue'
import NotFound from './components/NotFound.vue'
import Login from './components/Login.vue'
import Register from './components/Register.vue'
import SendPost from './components/SendPost.vue'
import MyPost from './components/MyPost.vue'
import client from './axios/client'
const routes = {
  '/': Home,
  '/login': Login,
  '/reg': Register,
  '/send': SendPost,
  '/mine': MyPost,
}
export default {
  data() {
    return {
      currentPath: window.location.hash,
      isLogined: false,
      loaded: false,
    }
  },
  computed: {
    currentView() {
      return routes[this.currentPath.slice(1) || '/'] || NotFound
    }
  },
  async mounted() {
    window.addEventListener('hashchange', () => {
      this.currentPath = window.location.hash
    })
    let succ = false;
    try {
      succ = await client.loginWithToken();
    } catch(e) { console.error(e) }
    if (!succ) {
      window.location.href = "#/login";
    }
    this.isLogined = client.isLogined;
    this.loaded = true;
  },
  methods: {
    auth() {
      this.isLogined = true;
    }
  }
}
</script>

<template>
  <div>
    <template v-if="!isLogined">
      <a href="#/login">Login</a> |
      <a href="#/reg">Register</a> |
    </template>
    <template v-else>
      <a href="#/">Home</a> |
      <a href="#/send">Send Post</a> |
      <a href="#/mine">My Posts</a>
    </template>
    <component v-if="loaded" :is="currentView" @auth="auth"/>
  </div>
</template>
