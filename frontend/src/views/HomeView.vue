<template>
	<div class="mx-auto max-w-7xl p-4 lg:p-8">
		<div class="flex items-end justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Trading Terminal (ETH/USDT)</h1>
				<p class="text-graphite mt-1">
					Status: <span :class="statusColor">{{ store.connectionStatus }}</span>
					<span class="ml-4">
						Round-trip Latency: <span class="font-semibold text-white">{{ store.roundtripLatency }}ms</span>
					</span>
					<span class="ml-4">
						Core Latency: <span class="font-semibold text-white">{{ (store.processingLatencyNs / 1000).toFixed(2) }}µs</span>
					</span>
				</p>
			</div>

			<div v-if="store.book" class="text-right">
				<div class="text-lg text-white">{{ store.bestAsk.toFixed(2) }} / {{ store.bestBid.toFixed(2) }}</div>
				<div class="text-graphite text-sm">Spread: {{ store.spread.toFixed(2) }}</div>
			</div>
		</div>

		<div class="mt-6 grid grid-cols-1 gap-4 lg:grid-cols-6 lg:gap-8">
			<div class="flex flex-col gap-4 lg:col-span-2">
				<OrderEntry />
				<div class="flex-grow">
					<TradeFeed v-if="store.trades.length" :trades="store.trades" />
					<div v-else class="flex-center text-graphite bg-background-light h-full rounded-lg p-4">No trades yet.</div>
				</div>
			</div>

			<div class="lg:col-span-4">
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2 md:gap-8">
					<div class="bg-background-light rounded-lg p-4">
						<h2 class="text-bullish-green mb-4 text-xl font-semibold">Bids (Buy)</h2>
						<OrderBook v-if="store.book" :levels="store.book.bids" side="Buy" />
						<div v-else class="flex-center text-graphite h-[600px]">Waiting for data...</div>
					</div>

					<div class="bg-background-light rounded-lg p-4">
						<h2 class="text-bearish-red mb-4 text-xl font-semibold">Asks (Sell)</h2>
						<OrderBook v-if="store.book" :levels="store.book.asks" side="Sell" />
						<div v-else class="flex-center text-graphite h-[600px]">Waiting for data...</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import OrderBook from '@/components/OrderBook.vue'
import TradeFeed from '@/components/TradeFeed.vue'
import OrderEntry from '@/components/OrderEntry.vue'
import { ConnectionStatus } from '@/types'
import { useIntervalFn } from '@vueuse/core'

const store = useAppStore()

useIntervalFn(() => {
	store.measureLatency()
}, 2000)

const statusColor = computed(() => {
	switch (store.connectionStatus) {
		case ConnectionStatus.CONNECTED:
			return 'text-bullish-green'
		case ConnectionStatus.ERROR:
			return 'text-bearish-red'
		default:
			return 'text-graphite'
	}
})
</script>
