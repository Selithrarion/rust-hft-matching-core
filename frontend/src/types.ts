export enum ConnectionStatus {
	DISCONNECTED = 'DISCONNECTED',
	CONNECTING = 'CONNECTING',
	CONNECTED = 'CONNECTED',
	ERROR = 'ERROR',
}

export type Side = 'Buy' | 'Sell'

export interface OrderBookLevel {
	price: number
	amount: number
}

export interface OrderBookSnapshot {
	bids: OrderBookLevel[]
	asks: OrderBookLevel[]
	timestamp: number
}

export interface Trade {
	price: number
	amount: number
	timestamp: number
	side: Side
}

export interface CandlestickData {
	time: number
	open: number
	high: number
	low: number
	close: number
}
