<script lang="ts">
	import GameGrid from "./GameGrid.svelte";
	import type { PageData } from "./$types";
	import { invalidate } from "$app/navigation";
	import { API_URL } from "@/lib/def";
	import { onDestroy, onMount } from "svelte";

	export let data: PageData;

	let moti: number;
	let score: number;
	let motiCost: number;
	let controller: {
		reset: () => void;
		tryKoeviikko: () => boolean;
	};
	let interval: number;

	function onLoss() {
		alert("Loss");
		controller.reset();
	}
	function onWin() {
		alert("Win");
		controller.reset();
	}
	async function onAddScore() {
		await fetch(API_URL + "/update_score", {
			body: JSON.stringify({
				newscore: score
			}),
			headers: {
				"Content-Type": "application/json"
			},
			method: "POST"
		});
	}

	onMount(() => {
		interval = setInterval(() => invalidate(API_URL + "/list_users"), 1000);
	});
	onDestroy(() => {
		clearInterval(interval);
	});
</script>

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
