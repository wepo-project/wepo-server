<script>
import Home from './components/Home.vue'
import NotFound from './components/NotFound.vue'
import Login from './components/Login.vue'
import Register from './components/Register.vue'
import SendPost from './components/SendPost.vue'
import { setTokenFromLocalStorage } from './axios/client'
const routes = {
  '/': Home,
  '/login': Login,
  '/reg': Register,
  '/send': SendPost,
}
export default {
  data() {
    return {
      currentPath: window.location.hash,
    }
  },
  computed: {
    currentView() {
      return routes[this.currentPath.slice(1) || '/'] || NotFound
    }
  },
  mounted() {
    window.addEventListener('hashchange', () => {
      this.currentPath = window.location.hash
    })
    if (!setTokenFromLocalStorage()) {
      window.location.href = "#/login";
    }
  }
}
</script>

<template>
  <div>
    <a href="#/">Home</a> |
    <a href="#/login">Login</a> |
    <a href="#/reg">Register</a> |
    <a href="#/send">Send Po</a> |
    <component :is="currentView" />
  </div>
</template>
