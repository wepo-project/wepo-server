<script setup lang="ts">
import { reactive } from "@vue/reactivity";
import client from "../axios/client";
import Heart from "../svg/heart.vue";
import Comment from "../svg/comment.vue";
import Hate from "../svg/Hate.vue";
import { useRouter } from "vue-router";

const router = useRouter();

const props = withDefaults(defineProps<{
  item?: PostModel
  showDelete?: boolean
}>(), {
  showDelete: false,
});

let id = props.item?.id.toString();

const state = reactive({
  like_count: props.item?.like_count ?? 0,
  liked: props.item?.liked ?? false,
  hate_count: props.item?.hate_count ?? 0,
  hated: props.item?.hated ?? false,
  comment_count: props.item?.comment_count ?? 0,
})

const check_details = async () => {
  console.log(`check_details ${id}`)
  router.push({
    name: 'po',
    params: { id },
  });
}

const like = async () => {
  let is_cancel = state.liked;
  const resp = await client.get("post", is_cancel ? "cancel_like" : "like", {
    params: { id },
  });
  if (resp.data.succ) {
    state.liked = !is_cancel;
    state.like_count += is_cancel ? -1 : 1;
  }
  // 之前点过赞了
  else if (resp.data.code == 201) {
    state.liked = !is_cancel
  }
};

const onOrigin = async (_id: string) => {
  console.log(`onOrigin ${_id}`)
  router.push({
    name: 'po',
    params: { id: _id },
  });
}

const hate = async () => {
  let is_cancel = state.hated;
  const resp = await client.get("post", is_cancel ? "cancel_hate" : "hate", {
    params: { id },
  });
  if (resp.data.succ) {
    state.hated = !is_cancel;
    state.hate_count += is_cancel ? -1 : 1;
  }
  // 之前点过赞了
  else if (resp.data.code == 201) {
    state.hated = !is_cancel
  }
};

const deletePost = async () => {
  await client.delete('post', 'delete', {
    data: { id }
  })
}

</script>

<template>
  <template v-if="item!=null">
    <div @click="check_details" class="p-2 border-b">
      <div class="flex pb-2">
        <img class="avatar rounded" :src="item!.sender.avatar_url" alt="avatar"/>
        <div class="flex flex-col ml-2">
          <div class="dark-white">{{item!.sender.nick}}</div>
          <div class="text-sm text-gray-500">{{ new Date(item!.create_time).toLocaleString() }}</div>
        </div>
      </div>
      <div class="mb-2 text-xl dark-white">{{ item!.content }}</div>
      <template v-if="item!.origin_id">
        <div class="cursor-pointer border border-gray-400 rounded-md p-2 mb-2" @click.stop="onOrigin(item!.origin_id!)">
          <div class="text-sm text-gray-400 mb-1">Origin</div>
          <div class="flex pb-2">
            <img class="avatar rounded" :src="item!.origin_sender!.avatar_url" alt="avatar"/>
            <div class="flex flex-col ml-2">
              <div class="dark-white">{{item!.origin_sender!.nick}}</div>
              <div class="text-sm text-gray-500">{{ new Date(item!.origin_create_time!).toLocaleString() }}</div>
            </div>
          </div>
          <div class="dark-white text-sm">{{item!.origin_content!}}</div>
        </div>
      </template>
      <div class="flex" @click.stop="void">
        <Heart v-bind:liked="state.liked" @click="like"/>
        <div class="ml-1 mr-2 dark-white select-none">{{state.like_count}}</div>
        <Comment/>
        <div class="ml-1 mr-2 dark-white select-none">{{state.comment_count}}</div>
        <Hate v-bind:hated="state.hated" @click="hate"/>
        <div class="ml-1 mr-2 dark-white select-none">{{state.hate_count}}</div>
        <div v-if="showDelete" class="ml-auto hover:underline" @click.stop="deletePost">Delete</div>
      </div>
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