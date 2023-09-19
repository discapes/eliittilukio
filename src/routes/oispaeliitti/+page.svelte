<script lang="ts">
	import GameGrid from "./GameGrid.svelte";
	import type { PageData } from "./$types";
	import { invalidate } from "$app/navigation";
	import { API_URL } from "@/lib/def";
	import { onDestroy, onMount } from "svelte";
	import { confetti } from "@neoconfetti/svelte";

	export let data: PageData;

	let moti: number;
	let score: number;
	let motiCost: number;
	let controller: {
		reset: () => void;
		tryKoeviikko: () => boolean;
	};

	let highscore = data.me.score;
	let won = false;
	let interval: number;
	let highscoreUpdateTimeout: number | null = null;

	function onLoss() {
		setTimeout(() => {
			alert("Hävisit!");
			controller.reset();
		}, 1000);
	}
	function onWin() {
		won = true;
	}

	async function uploadHighscore() {
		highscoreUpdateTimeout = null;
		await fetch(API_URL + "/update_score", {
			body: JSON.stringify({
				newscore: highscore
			}),
			headers: {
				"Content-Type": "application/json"
			},
			method: "POST"
		});
	}

	async function onAddScore() {
		if (score > highscore) {
			highscore = score;
			if (highscoreUpdateTimeout === null) {
				highscoreUpdateTimeout = setTimeout(uploadHighscore, 3000);
			}
		}
	}

	onMount(() => {
		interval = setInterval(() => invalidate(API_URL + "/list_users"), 1000);
	});
	onDestroy(() => {
		clearInterval(interval);
	});
</script>

{#if won}
	<div
		style="position: absolute; left: 50%; top: 30%"
		use:confetti={{
			force: 0.7,
			stageWidth: window.innerWidth,
			stageHeight: window.innerHeight,
			colors: ["#ff3e00", "#40b3ff", "#676778"]
		}}
	/>
{/if}

<section class="flex justify-center">
	<div class="flex flex-col items-end justify-start w-full p-3">
		<div class="bg p-3">
			{#each data.top as p}
				<p class="">{p.username.slice(0, 20)}: {p.score}</p>
			{/each}
		</div>
	</div>
	<GameGrid
		on:loss
		bind:moti
		bind:motiCost
		bind:score
		bind:controller
		{onLoss}
		{onWin}
		{onAddScore}
	/>
	<div class="flex flex-col items-start justify-start p-3 w-full gap-3">
		<div class="bg p-3">
			<p>moti: {moti}</p>
			<p>score: {score}</p>
			<p>highscore: {highscore}</p>
		</div>
		<button
			class="text-center button"
			disabled={moti < motiCost}
			class:bg-lime-800={moti >= motiCost}
			on:click={controller.tryKoeviikko}
			><p>Koeviikko</p>
			({motiCost})</button
		>
	</div>
</section>
