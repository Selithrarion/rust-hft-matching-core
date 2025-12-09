<template>
	<div class="relative h-[600px] overflow-y-auto text-sm">
		<div class="bg-background-dark text-graphite sticky top-0 z-20 grid grid-cols-3 gap-4 px-4 py-2">
			<span>Price (USDT)</span>
			<span class="text-right">Amount (ETH)</span>
			<span class="text-right">Total</span>
		</div>

		<div v-for="level in levelsWithDepth" :key="level.price" class="relative grid grid-cols-3 gap-4 px-4 py-1 hover:bg-white/5">
			<div
				class="absolute top-0 right-0 h-full"
				:class="side === 'Buy' ? 'bg-bullish-green/10' : 'bg-bearish-red/10'"
				:style="{ width: `${(level.depth / maxDepth) * 100}%` }"
			/>

			<span class="z-10" :class="side === 'Buy' ? 'text-bullish-green' : 'text-bearish-red'">
				{{ level.price.toFixed(2) }}
			</span>
			<span class="z-10 text-right">{{ level.amount.toFixed(4) }}</span>
			<span class="text-graphite-light z-10 text-right">{{ (level.price * level.amount).toFixed(2) }}</span>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { OrderBookLevel, Side } from '@/types'

const props = defineProps<{
	levels: OrderBookLevel[]
	side: Side
}>()

const levelsWithDepth = computed(() => {
	let totalAmount = 0
	return props.levels.map((level) => {
		totalAmount += level.amount
		return { ...level, depth: totalAmount }
	})
})

const maxDepth = computed(() => {
	return levelsWithDepth.value[levelsWithDepth.value.length - 1]?.depth ?? 0
})
</script>
