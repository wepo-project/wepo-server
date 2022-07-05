<script lang="ts" setup>
import { ref } from 'vue'
import client from '../axios/client';

const nick = ref('')
const pwd = ref('')

async function onRegister() {
  const resp = await client.post("/user/add_user", {
    'nick': nick.value,
    'pwd': pwd.value,
  });
}
</script>

<template>
  <div>
    <input v-model.trim="nick" type="text" placeholder="nick"/>
    <br/>
    <input v-model.trim="pwd" type="password" placeholder="password"/>
    <div>
      <input type="button" value="Register" @click="onRegister"/>
    </div>
  </div>
</template>

<style scoped>
</style>
