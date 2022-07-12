<script lang="ts" setup>
import { ref } from 'vue'
import client from '../axios/client'

const content = ref('')

async function onSend() {
  if (content.value != '') {
    const resp = await client.post('post', 'add_post', {
      data: {
        content: content.value,
      }
    });
    if (resp.data.id) {
      content.value = ""
    }
  }
}
</script>

<template>
  <div>
    <textarea v-model="content" style="width:200px;height:100px;"></textarea>
    <input type="button" @click="onSend" value="Send" />
  </div>
</template>