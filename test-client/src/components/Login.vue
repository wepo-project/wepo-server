<script lang="ts" setup>
import { ref } from 'vue'
import client, { setToken } from '../axios/client';

const nick = ref('')
const pwd = ref('')

const emit = defineEmits(["auth"]);

async function onLogin() {
  const resp = await client.post("/user/login", {
    'nick': nick.value,
    'pwd': pwd.value,
  });
  setToken(resp.data["token"]);
  emit("auth");
  window.location.href = "#/";
}
</script>

<template>
  <div>
    <input v-model.trim="nick" type="text" placeholder="nick"/>
    <br/>
    <input v-model.trim="pwd" type="password" placeholder="password"/>
    <div>
      <input type="button" value="Login" @click="onLogin"/>
    </div>
  </div>
</template>

<style scoped>
</style>
