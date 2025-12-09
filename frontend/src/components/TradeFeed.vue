<template>
	<div class="bg-background-light h-[600px] overflow-y-auto rounded-lg text-sm">
		<h2 class="text-graphite-light bg-background-light sticky top-0 p-4 text-xl font-semibold">Trades</h2>
		<div v-for="(trade, index) in trades" :key="index" class="grid grid-cols-3 gap-4 p-4 py-1">
			<span :class="trade.side === 'Buy' ? 'text-bullish-green' : 'text-bearish-red'">
				{{ trade.price.toFixed(2) }}
			</span>
			<span class="text-right">{{ trade.amount.toFixed(4) }}</span>
			<span class="text-graphite text-right">{{ new Date(trade.timestamp).toLocaleTimeString() }}</span>
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Trade } from '@/types'

defineProps<{
	trades: Trade[]
}>()
</script>
