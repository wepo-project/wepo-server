<script lang="ts" setup>
import { onMounted } from "@vue/runtime-core";
import { reactive } from "vue";
import client from "../axios/client";

const state = reactive({
    list: <any>[],
    page: -1,
    next: true,
})

const getNextPage = async () => {
    state.page = state.page + 1;
    const resp = await client.post('/post/my_post', {
        'page': state.page,
    });
    state.next = resp.data.next;
    state.page = resp.data.next;
    state.list = resp.data.list;
}

onMounted(async () => {
    await getNextPage();
});

const like = async (id: any, index: number) => {
    const resp = await client.get('/post/like', {
        params: {
            'id': id
        }
    });
    if (resp.data.succ) {
        state.list[index].likes += 1;
    }
}
</script>

<template>
    <div>
        <div>MY POST</div>
        <div v-for="(item, index) in state.list" :key="item.id">
            <!-- <div>id: {{item.id}}</div> -->
            <div>{{new Date(item.create_time).toLocaleString()}}</div>
            <div>{{item.content}}</div>
            <input type="button" :value="`likes: ${item.likes}`" @click="like(item.id, index)"/>
            <br/>
            <br/>
        </div>
    </div>
</template>