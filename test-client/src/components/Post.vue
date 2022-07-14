<script setup lang="ts">
import { reactive } from "@vue/reactivity";
import client from "../axios/client";
import router from "../pageRouter";
const props = defineProps<{
  item?: PostModel
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

const onOrigin = async (id: string) => {
  router.push(`/po/${id}`);
}

</script>

<template>
  <template v-if="item!=null">
    <div @click="check_details" class="p-2 border-b">
      <div class="flex pb-2">
        <img class="avatar rounded" :src="item!.sender.avatar_url" alt="avatar"/>
        <div class="flex flex-col ml-2">
          <div>{{item!.sender.nick}}</div>
          <div class="text-sm text-gray-500">{{ new Date(item!.create_time).toLocaleString() }}</div>
        </div>
      </div>
      <div class="mb-2 text-xl">{{ item!.content }}</div>
      <template v-if="item!.origin_id">
        <div class="cursor-pointer border border-gray-400 rounded-md p-2 mb-2" @click="onOrigin(item!.origin_id!)">
          <div class="text-sm text-gray-400 mb-1">转自</div>
          <div class="flex pb-2">
            <img class="avatar rounded" :src="item!.origin_sender!.avatar_url" alt="avatar"/>
            <div class="flex flex-col ml-2">
              <div>{{item!.origin_sender!.nick}}</div>
              <div class="text-sm text-gray-500">{{ new Date(item!.origin_create_time!).toLocaleString() }}</div>
            </div>
          </div>
          <div>{{item!.origin_content!}}</div>
        </div>
      </template>
      <!-- 点赞 -->
      <input type="button" :value="`${state.liked ? 'liked' : 'like'}:${state.like_count}`" class="action-button"
        :class="state.liked ? 'active' : ''" @click.stop="like" />
      <!-- 评论 -->
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

.avatar {
  width: 40px;
  height: 40px;
}
</style>