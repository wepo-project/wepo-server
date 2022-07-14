<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { reactive } from 'vue';

const state = reactive({
    tabs: [
        { to: "/", name: "HOME" },
        { to: "/send", name: "SEND" },
        { to: "/my_post", name: "ME" },
    ]
})

</script>
<template>
    <ul class="flex flex-row flex-wrap list-none border-b-0 pl-0">
        <li v-for="(item) in state.tabs" class="flex-auto flex items-center justify-center" role="presentation">
            <router-link :to="item.to" class="
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
            my-2
            dark-white
            hover:border-transparent hover:bg-gray-100
            focus:border-transparent
            dark:hover:bg-[#48484f]
            active
            ">{{item.name}}</router-link>
        </li>
    </ul>
</template>