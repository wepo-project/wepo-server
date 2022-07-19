<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router';
import { reactive, watch } from 'vue';
const route = useRoute();
const tabs = [
    { name: "home", title: "HOME" },
    { name: "send", title: "SEND" },
    { name: "my_post", title: "ME" },
];
const getIndex = () => tabs.findIndex((e) => route.name == e.name)
const state = reactive({
    currTab: getIndex(),
})
watch(route, () => {
    state.currTab = getIndex()
})
</script>
<template>
    <ul class="flex flex-row flex-wrap list-none border-b-0 pl-0">
        <li v-for="(item, index) in tabs" class="flex-auto flex items-center justify-center" role="presentation">
            <router-link :to="{ name: item.name }" class="
            w-full
            block
            font-medium
            text-center
            text-xs
            leading-tight
            uppercase
            border-x-0 border-t-0 border-b-2 border-transparent
            px-6
            py-3
            dark-white
            hover:border-transparent hover:bg-gray-100
            focus:border-transparent
            dark:hover:bg-[#48484f]
            active
            " :class="state.currTab==index?'bg-gray-100 dark:bg-[#48484f]':''">{{item.title}}</router-link>
        </li>
    </ul>
</template>