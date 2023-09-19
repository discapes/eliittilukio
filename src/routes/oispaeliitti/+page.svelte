<script lang="ts">
	import GameGrid from "./GameGrid.svelte";

	let moti: number;
	let score: number;
	let motiCost: number;
	let controller: {
		reset: () => void;
		tryKoeviikko: () => boolean;
	};

	function onLoss() {
		alert("Loss");
		controller.reset();
	}
	function onWin() {
		alert("Win");
		controller.reset();
	}
</script>

<section class="flex justify-center">
	<div class="flex flex-col">
		<div class="flex gap-7 p-3">
			<h1 class="tracking-widest font-thin">Oispa Eliitti</h1>
			<div class="box grow rounded flex flex-col justify-around gap-3">
				<p>Moti: <span class="font-bold">{moti}</span></p>
				<p>Pisteet: <span class="font-bold">{score}</span></p>
			</div>
			<button
				class="text-center"
				disabled={moti < motiCost}
				class:bg-lime-200={moti >= motiCost}
				on:click={controller.tryKoeviikko}
				><p>Koeviikko</p>
				({motiCost})</button
			>
		</div>
		<div class="inline-flex">
			<GameGrid on:loss bind:moti bind:motiCost bind:score bind:controller />
		</div>
		<div class="flex gap-3 p-3 rounded" />
	</div>
</section>

<style>
	button,
	.box {
		background-color: rgba(245, 245, 245, 0.8);
		color: #101010;
		@apply rounded p-3 shadow-lg drop-shadow-lg border-2 border-black/80;
	}
	button {
		background-color: rgba(245, 245, 245, 0.8);
	}
	button:hover:not([disabled]) {
		background-color: rgb(234, 234, 234);
	}
</style>
