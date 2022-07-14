<script lang="ts" setup>

const props = defineProps<{
    hated: boolean,
}>();

const emit = defineEmits(["click"])

const onClick = () => {
    emit('click')
}

</script>

<template>
    <svg class="w-6 h-6" :class="props.hated ? 'dark:fill-[#e2e8f0]' : 'dark:stroke-[#e2e8f0]'" :fill="props.hated ? '#000' : 'none'" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
        @click.stop="onClick">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M10 14H5.236a2 2 0 01-1.789-2.894l3.5-7A2 2 0 018.736 3h4.018a2 2 0 01.485.06l3.76.94m-7 10v5a2 2 0 002 2h.096c.5 0 .905-.405.905-.904 0-.715.211-1.413.608-2.008L17 13V4m-7 10h2m5-10h2a2 2 0 012 2v6a2 2 0 01-2 2h-2.5" />
    </svg>
</template>