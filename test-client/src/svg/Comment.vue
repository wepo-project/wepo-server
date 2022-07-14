<script lang="ts" setup>

const emit = defineEmits(["click"])

const onClick = () => {
    emit('click')
}

</script>

<template>
    <svg class="w-6 h-6 dark:stroke-[#e2e8f0]" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" @click.stop="onClick">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
    </svg>
</template>