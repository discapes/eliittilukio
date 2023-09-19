<script lang="ts">
	import { browser } from "$app/environment";
	import Engine from "./engine.js";
	import { createEventDispatcher } from "svelte";
	import "./tiles.css";

	const dispatch = createEventDispatcher();
	const P_MAX_TILE = 4096;
	const P_INITIAL_MOTICOSTS = "1000,1100,1300,1600,2000";
	let tileContainer: HTMLDivElement;

	export let moti = 0;
	export let score = 0;
	export let motiCost = calcMotiCost(0);
	export let controller = {
		reset,
		tryKoeviikko
	};

	let katkoja = 0;

	const handlers = {
		onWin: () => dispatch("win"),
		onLoss: () => dispatch("loss"),
		onAddScore: (add: number) => {
			score += add;
			moti += add;
		}
	};

	let engine = new Engine(4, 4, handlers);
	if (browser) update();

	export function reset() {
		engine = new Engine(4, 4, handlers);
		moti = 0;
		score = 0;
		motiCost = calcMotiCost(0);
		update();
	}

	export function tryKoeviikko() {
		if (moti >= motiCost) {
			moti -= motiCost;
			motiCost = calcMotiCost(++katkoja);
			engine.katkoReissu();
			update();
			return true;
		} else {
			return false;
		}
	}

	function calcMotiCost(n: number) {
		if (n < 5) return +P_INITIAL_MOTICOSTS.split(",")[n]; //50 * x^2 - (50 * x) + 1000;
		else return 500 * (n + 1) ** 2 - 4500 * (n + 1) + 12000; // (2000), 3000, 5000, 8000, 12000
	}

	function handleKd(e: KeyboardEvent) {
		switch (e.key.toLowerCase()) {
			case "arrowup":
			case "w":
				engine.move(0);
				break;
			case "arrowleft":
			case "a":
				engine.move(3);
				break;
			case "arrowdown":
			case "s":
				engine.move(2);
				break;
			case "arrowright":
			case "d":
				engine.move(1);
				break;
			default:
				return;
		}
		update();
	}

	function update() {
		if (browser)
			window.requestAnimationFrame(() => {
				while (tileContainer.firstChild) tileContainer.removeChild(tileContainer.firstChild);
				for (let col of engine.cells) for (let cell of col) if (cell) drawTile(cell);
			});
	}

	function drawTile(tile: {
		x: number;
		y: number;
		value: number;
		previousPosition?: { x: number; y: number };
		mergedFrom?: any[];
	}) {
		const position = tile.previousPosition || { x: tile.x, y: tile.y };

		const wrapper = document.createElement("div");
		wrapper.classList.add("tile", "tile-" + Math.min(P_MAX_TILE, tile.value));
		wrapper.style.transform = `translate(${125 * position.x}px, ${125 * position.y}px)	`;

		const inner = document.createElement("div");
		inner.classList.add("tile-inner");
		inner.style.backgroundImage = `url("/img/${tile.value}.png")`;

		if (tile.previousPosition) {
			// Make sure that the tile gets rendered in the previous position first
			window.requestAnimationFrame(
				() => (wrapper.style.transform = `translate(${125 * tile.x}px, ${125 * tile.y}px)`)
			);
		} else if (tile.mergedFrom) {
			wrapper.classList.add("tile-merged");
			// Render the tiles that merged
			tile.mergedFrom.forEach(drawTile);
		} else {
			wrapper.classList.add("tile-new");
		}

		wrapper.appendChild(inner);
		tileContainer.appendChild(wrapper);
	}
</script>

<svelte:head>
	<link rel="prefetch" href="/img/2.png" />
	<link rel="prefetch" href="/img/4.png" />
	<link rel="prefetch" href="/img/8.png" />
	<link rel="prefetch" href="/img/16.png" />
	<link rel="prefetch" href="/img/32.png" />
	<link rel="prefetch" href="/img/64.png" />
	<link rel="prefetch" href="/img/128.png" />
	<link rel="prefetch" href="/img/256.png" />
	<link rel="prefetch" href="/img/512.png" />
	<link rel="prefetch" href="/img/1024.png" />
	<link rel="prefetch" href="/img/2048.png" />
	<link rel="prefetch" href="/img/4096.png" />
</svelte:head>
<svelte:window on:keydown={handleKd} />

<div class="absolute z-10 translate-y-[16px] translate-x-[16px]" bind:this={tileContainer} />
<div
	class="grid grid-flow-col gap-[15px] p-4 bg-black/10 rounded"
	style={`grid-template-rows: repeat(${engine.sizey}, minmax(0, 1fr)); grid-template-columns: repeat(${engine.sizex}`}
>
	{#each { length: engine.sizex * engine.sizey } as _}
		<div class="min-h-[110px] min-w-[110px] bg-black/20 rounded" />
	{/each}
</div>

<style global>
</style>
