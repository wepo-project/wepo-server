<script setup lang="ts">
import { reactive } from "@vue/reactivity";
import client from "../axios/client";
import router from "../pageRouter";
const props = defineProps<{
  item: {
    id: string
    create_time: string
    content: string
    sender: number
    likes: number
    liked: number
    comments: number
  } | undefined;
}>();

let id = props.item?.id.toString();

const state = reactive({
  likes: props.item?.likes ?? 0,
  liked: props.item?.liked ?? false, // 已经点赞
  comments: props.item?.comments ?? 0,
})

const like = async () => {
  if (state.liked) {
    return cancel_like()
  }
  const resp = await client.get("post", "like", {
    params: { id },
  });
  if (resp.data.succ) {
    state.liked = true
    state.likes += 1
  }
  // 之前点过赞了
  else if (resp.data.code == 201) {
    state.liked = true
  }
};

const check_details = async () => {
  router.push(`/po/${id}`);
}

const cancel_like = async () => {
  const resp = await client.get("post", "cancel_like", {
    params: { id },
  });
  if (resp.data.succ || resp.data.code == 201) {
    state.liked = false
    state.likes -= 1
  }
  // 没有点过赞
  else if (resp.data.code == 201) {
    state.liked = false
  }
}

</script>

<template>
  <template v-if="props.item!=null">
    <div @click="check_details">
      <div>{{ new Date(item!.create_time).toLocaleString() }}</div>
      <div>{{ item!.content }}</div>
      <input type="button" :value="`${state.liked ? 'liked' : 'like'}:${state.likes}`" class="action-button"
        :class="state.liked ? 'active' : ''" @click.stop="like" />
      <input type="button" :value="`comment:${state.comments}`" class="action-button" />
    </div>
  </template>
</template>

<style scoped>
.action-button {
  border: none;
  padding: 4px 10px;
  margin-right: 10px;
}

.active {
  background: #9c9cff;
  color: white
}
</style>