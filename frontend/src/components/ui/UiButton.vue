<template>
	<button
		:disabled="disabled"
		class="transform cursor-pointer rounded-md px-5 py-2.5 font-bold text-white transition-all duration-150 ease-in-out hover:scale-102 active:scale-98 disabled:cursor-not-allowed disabled:bg-gray-600 disabled:opacity-50 disabled:hover:scale-100"
		:class="{
			'bg-osu-pink hover:opacity-80': variant === 'primary',
			'bg-osu-light text-gray-300 hover:bg-gray-700': variant === 'secondary',
		}"
	>
		<slot />
	</button>
</template>

<script setup lang="ts">
withDefaults(defineProps<{ disabled?: boolean; variant?: 'primary' | 'secondary' }>(), {
	variant: 'primary',
})
</script>
