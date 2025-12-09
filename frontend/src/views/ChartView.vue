<template>
	<div ref="chartContainer" class="h-[500px] w-full" />
</template>

<script setup lang="ts">
import { ref, onMounted, watch, onBeforeUnmount } from 'vue'
import {
	createChart,
	type IChartApi,
	type ISeriesApi,
	type CandlestickData,
	type UTCTimestamp,
	CandlestickSeries,
} from 'lightweight-charts'
import { useAppStore } from '@/stores/app'
import { useResizeObserver } from '@vueuse/core'

const store = useAppStore()
const chartContainer = ref<HTMLElement | null>(null)
let chart: IChartApi | null = null
let candlestickSeries: ISeriesApi<'Candlestick'> | null = null

const applyChartTheme = (chart: IChartApi) => {
	chart.applyOptions({
		layout: {
			background: { color: '#0d1117' },
			textColor: '#c9d1d9',
		},
		grid: {
			vertLines: { color: '#ffffff10' },
			horzLines: { color: '#ffffff10' },
		},
		timeScale: {
			timeVisible: true,
			secondsVisible: true,
		},
	})
}

const applySeriesTheme = (series: ISeriesApi<'Candlestick'>) => {
	series.applyOptions({
		wickUpColor: '#2da44e',
		upColor: '#2da44e',
		wickDownColor: '#f85149',
		downColor: '#f85149',
		borderVisible: false,
	})
}

onMounted(() => {
	if (!chartContainer.value) return

	chart = createChart(chartContainer.value)

	applyChartTheme(chart)

	candlestickSeries = chart.addSeries(CandlestickSeries)
	applySeriesTheme(candlestickSeries)

	candlestickSeries.setData(store.candles as CandlestickData<UTCTimestamp>[])

	chart.timeScale().fitContent()
})

useResizeObserver(chartContainer, (entries) => {
	const entry = entries[0]
	const { width, height } = entry!.contentRect
	if (chart) {
		chart.resize(width, height)
	}
})

onBeforeUnmount(() => {
	if (chart) {
		chart.remove()
		chart = null
	}
})

watch(
	() => store.candles,
	(newCandles) => {
		if (!candlestickSeries || newCandles.length === 0) return
		const lastCandle = newCandles[newCandles.length - 1]
		candlestickSeries.update(lastCandle as CandlestickData<UTCTimestamp>)
	},
	{ deep: true },
)
</script>
