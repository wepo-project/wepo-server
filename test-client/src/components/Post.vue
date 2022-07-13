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
    like_count: number
    liked: number
    comment_count: number
  } | undefined;
}>();

let id = props.item?.id.toString();

const state = reactive({
  like_count: props.item?.like_count ?? 0,
  liked: props.item?.liked ?? false, // 已经点赞
  comment_count: props.item?.comment_count ?? 0,
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
    state.like_count += 1
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
    state.like_count -= 1
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
      <input type="button" :value="`${state.liked ? 'liked' : 'like'}:${state.like_count}`" class="action-button"
        :class="state.liked ? 'active' : ''" @click.stop="like" />
      <input type="button" :value="`comment:${state.comment_count}`" class="action-button" />
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