import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { listen, type Event, once } from '@tauri-apps/api/event'
import type { OrderBookSnapshot, Trade, CandlestickData } from '@/types'
import { ConnectionStatus } from '@/types'
import { invoke } from '@tauri-apps/api/core'

const MAX_TRADES_IN_FEED = 100
const MAX_CANDLES = 500

interface LatencyUpdate {
	avg_processing_time_ns: number
}

export const useAppStore = defineStore('app', () => {
	const connectionStatus = ref<ConnectionStatus>(ConnectionStatus.DISCONNECTED)
	const lastError = ref<string | null>(null)
	const book = ref<OrderBookSnapshot | null>(null)
	const trades = ref<Trade[]>([])
	const candles = ref<CandlestickData[]>([])
	const roundtripLatency = ref(0)
	const processingLatencyNs = ref(0)

	listen<OrderBookSnapshot>('book_update', (event: Event<OrderBookSnapshot>) => {
		event.payload.bids.forEach((level) => (level.price /= 100))
		event.payload.asks.forEach((level) => (level.price /= 100))
		book.value = event.payload
	}).catch(console.error)

	listen<Trade[]>('new_trades_batch', (event: Event<Trade[]>) => {
		const tradesBatch = event.payload
		if (tradesBatch.length === 0) return

		for (const trade of tradesBatch) {
			trade.price /= 100
			trade.amount /= 10000

			const tradeTime = Math.floor(trade.timestamp / 1000)
			const lastCandle = candles.value[candles.value.length - 1]

			if (lastCandle && lastCandle.time === tradeTime) {
				lastCandle.high = Math.max(lastCandle.high, trade.price)
				lastCandle.low = Math.min(lastCandle.low, trade.price)
				lastCandle.close = trade.price
			} else {
				const newCandle: CandlestickData = {
					time: tradeTime,
					open: trade.price,
					high: trade.price,
					low: trade.price,
					close: trade.price,
				}
				candles.value.push(newCandle)
			}
		}
		trades.value.unshift(...tradesBatch)

		if (trades.value.length > MAX_TRADES_IN_FEED) {
			trades.value.length = MAX_TRADES_IN_FEED
		}
		if (candles.value.length > MAX_CANDLES) {
			candles.value.splice(0, candles.value.length - MAX_CANDLES)
		}
	}).catch(console.error)

	listen<string>('error', (event: Event<string>) => {
		lastError.value = event.payload
		connectionStatus.value = ConnectionStatus.ERROR
	}).catch(console.error)

	listen<LatencyUpdate>('latency_update', (event: Event<LatencyUpdate>) => {
		processingLatencyNs.value = event.payload.avg_processing_time_ns
	}).catch(console.error)

	const bestAsk = computed(() => book.value?.asks[0]?.price ?? 0)
	const bestBid = computed(() => book.value?.bids[0]?.price ?? 0)
	const spread = computed(() => {
		if (bestAsk.value > 0 && bestBid.value > 0) {
			return bestAsk.value - bestBid.value
		}
		return 0
	})

	async function measureLatency() {
		const start = performance.now()
		await invoke('ping')
		const end = performance.now()
		roundtripLatency.value = Math.round(end - start)
	}

	async function initializeConnection() {
		try {
			connectionStatus.value = await invoke<ConnectionStatus>('get_connection_status')
		} catch (e) {
			console.error('Failed to initialize connection:', e)
			connectionStatus.value = ConnectionStatus.ERROR
		}
	}

	return {
		connectionStatus,
		lastError,
		book,
		trades,
		candles, // Экспортируем свечи
		bestAsk,
		bestBid,
		spread,
		roundtripLatency,
		processingLatencyNs,
		measureLatency,
		initializeConnection, // Expose the function
	}
})
