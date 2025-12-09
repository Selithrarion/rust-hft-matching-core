<template>
	<div class="bg-background-light rounded-lg p-4">
		<h2 class="mb-4 text-xl font-semibold text-white">Place Order</h2>
		<form @submit.prevent>
			<div class="mb-4">
				<label for="price" class="text-graphite mb-1 block text-sm">Price (USDT)</label>
				<input
					id="price"
					v-model.number="price"
					type="number"
					placeholder="0.00"
					class="bg-background-dark w-full rounded-md border-2 border-transparent p-2 text-white focus:border-blue-500 focus:outline-none"
				/>
			</div>

			<div class="mb-6">
				<label for="amount" class="text-graphite mb-1 block text-sm">Amount (ETH)</label>
				<input
					id="amount"
					v-model.number="amount"
					type="number"
					placeholder="0.0000"
					class="bg-background-dark w-full rounded-md border-2 border-transparent p-2 text-white focus:border-blue-500 focus:outline-none"
				/>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<button
					@click="handleSubmit('Buy')"
					class="bg-bullish-green/80 hover:bg-bullish-green rounded-md py-2 font-semibold text-white transition-colors"
				>
					Buy
				</button>
				<button
					@click="handleSubmit('Sell')"
					class="bg-bearish-red/80 hover:bg-bearish-red rounded-md py-2 font-semibold text-white transition-colors"
				>
					Sell
				</button>
			</div>
			<p v-if="error" class="text-bearish-red mt-4 text-center text-sm">{{ error }}</p>
		</form>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Side } from '@/types'

const price = ref<number | null>(null)
const amount = ref<number | null>(null)
const error = ref<string | null>(null)

async function handleSubmit(side: Side) {
	error.value = null
	if (!price.value || !amount.value || price.value <= 0 || amount.value <= 0) {
		error.value = 'Please enter a valid price and amount.'
		return
	}

	try {
		await invoke('place_order', {
			price: price.value,
			amount: amount.value,
			side: side,
		})
		price.value = null
		amount.value = null
	} catch (e) {
		console.error('Failed to place order:', e)
		error.value = `Error: ${e}`
	}
}
</script>
