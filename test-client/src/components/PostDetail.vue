<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue';
import client from '../axios/client';
import router from '../pageRouter';
import Post from './Post.vue';

let id = router.currentRoute.value.params.id as string;
console.log(`id:${id}`)

let state = reactive({
  post: null as PostModel | null,
  comments: [] as PostModel[],
})

onMounted(async () => {
  let resp = await client.get('post', 'get_post', {
    params: { id },
  })
  if (resp.data) {
    state.post = resp.data.post
    state.comments = resp.data.comments
  }
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
    <Post v-if="state.post" :item="state.post"></Post>
    <br/>
    <div class="flex flex-col px-2">
      <textarea v-model="content" class="w-full h-20 p-2 border rounded-sm"></textarea>
      <div class="btn btn-blue w-fit mt-2" @click="onComment">Comment</div>
    </div>
    <br/>
    <br/>
    <template v-if="state.comments && state.comments.length">
      <div v-for="(item) in state.comments">
        <Post :item="item"></Post>
        <br/>
      </div>
    </template>
  </div>
</template>