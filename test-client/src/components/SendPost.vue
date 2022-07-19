<script lang="ts" setup>
import { ref } from 'vue'
import client from '../axios/client'

const content = ref('')

async function onSend() {
  if (content.value != '') {
    const resp = await client.post('post', 'send', {
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
  <div class="py-2 px-3">
    <textarea v-model="content" class="w-full h-20 p-2 border rounded-sm"></textarea>
    <div class="btn btn-blue w-fit mt-2" @click="onSend">Send</div>
  </div>
</template>