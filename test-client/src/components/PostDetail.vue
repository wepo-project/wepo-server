<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue';
import client from '../axios/client';
import router from '../pageRouter';
import Post from './Post.vue';

let id = router.currentRoute.value.params.id as string;

let state = reactive({
  data: null as any,
})

onMounted(async () => {
  let resp = await client.get('post', 'get_post', {
    params: { id },
  })
  state.data = resp.data;
})
const content = ref('')
async function onComment() {
  if (content.value != '') {
    const resp = await client.post('post', 'comment', {
      data: {
        content: content.value,
        origin_id: id,
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
    <div>DETAIL</div>
    <Post v-if="state.data" :item="state.data"></Post>
    <br/>
    <br/>
    <textarea v-model="content" style="width:200px;height:40px;"></textarea>
    <input type="button" @click="onComment" value="Comment" />
  </div>
</template>