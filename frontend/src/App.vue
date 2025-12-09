<template>
	<div class="bg-background-dark text-graphite-light flex min-h-screen flex-col font-sans">
		<header class="bg-background-light/80 sticky top-0 z-10 border-b border-white/10 backdrop-blur-sm">
			<nav class="mx-auto flex max-w-7xl items-center justify-between p-4 lg:px-8" aria-label="Global">
				<div class="flex lg:flex-1">
					<RouterLink to="/" class="text-accent-blue -m-1.5 p-1.5 text-xl font-bold"> HFT Terminal </RouterLink>
				</div>
				<div class="flex gap-8">
					<RouterLink to="/" class="text-graphite text-sm leading-6 font-semibold hover:text-white">Order Book</RouterLink>
					<RouterLink to="/chart" class="text-graphite text-sm leading-6 font-semibold hover:text-white">Live Chart</RouterLink>
				</div>
			</nav>
		</header>

		<main class="flex-grow">
			<RouterView />
		</main>

		<footer class="text-graphite p-4 text-center text-xs">
			<a
				href="https://github.com/Selithrarion/rust-hft-matching-core"
				target="_blank"
				class="inline-flex items-center gap-2 transition-colors hover:text-white"
			>
				<GitHubIcon />
				<span>HFT Matching Engine by Selithrarion</span>
			</a>
		</footer>
	</div>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import GitHubIcon from './components/icons/GitHubIcon.vue'
</script>
