<script lang="ts" setup>
import { ref } from 'vue'
import client from '../axios/client';

const nick = ref('')
const pwd = ref('')

const emit = defineEmits(["auth"]);

async function onLogin() {
  let succ = await client.loginWithAccount(nick.value, pwd.value);
  if (succ) {
    emit("auth");
    window.location.href = "#/";
  }
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
