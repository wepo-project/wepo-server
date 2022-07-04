<script setup>
import { ref } from 'vue'
import client, { setToken } from '../axios/client';

defineProps({
  msg: String
})

const nick = ref('')
const pwd = ref('')

async function onLogin() {
  const resp = await client.post("/user/login", {
    'nick': nick.value,
    'pwd': pwd.value,
  });
  setToken(resp.data["token"]);
}
</script>

<template>
  <div>
    <input v-model.trim="nick" type="text" placeholder="nick"/>
    <br/>
    <input v-model.trim="pwd" type="password" placeholder="password"/>
  </div>
  <div>
    <input type="button" value="Login" @click="onLogin"/>
  </div>
</template>

<style scoped>
</style>
