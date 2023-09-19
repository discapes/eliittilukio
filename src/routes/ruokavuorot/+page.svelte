<script lang="ts">
	import { browser } from "$app/environment";
	import "./app.css";
	import { getSlot } from "./model";
	import { parse, type Schedule } from "./parser";

	let schedule: Schedule | null;

	function load(clipboard: string) {
		localStorage.setItem("rkv-data", clipboard);
		schedule = parse(clipboard);
		fetch(window.location.href, {
			method: "POST",
			body: JSON.stringify({
				clipboard,
				problem: !schedule || schedule.hasProblem()
			})
		});
	}

	if (browser) {
		document.addEventListener("paste", (e) => {
			e.preventDefault();
			const data = e.clipboardData!.getData("text");
			load(data);
		});
		if (localStorage.getItem("rkv-data")) load(localStorage.getItem("rkv-data")!);
	}

	const weekdays = `MA,TI,KE,TO,PE`.split(",");
</script>

<svelte:head>
	<title>Ruokavuorot</title>
</svelte:head>

<div class="p-10 md:px-32 bg flex-col flex gap-3">
	<h1>TSYK jakso 1 ruokavuorot</h1>
	<a
		class="font-bold underline"
		href="https://eliittilukio-public.s3.eu-north-1.amazonaws.com/Record_2023-04-13-11-03-16.mp4"
		>lyhyt ohjevideo</a
	>
	<p class="max-w-[400px] min-w-[200px]">
		Etsi Wilman lukujärjestyksestä ensimmäinen kokonainen viikko, kopioi kaikki painamalla
		<code>CTRL + A</code> ja <code>CTRL + C</code> (tai puhelimella <code>Valitse kaikki</code>
		ja <code>Kopioi</code>) ja liitä se tänne painamalla
		<code>CTRL + V</code> (tai puhelimella <code>Liitä</code> seuraavaan kentään). Tiedot säilyy eli
		kannattaa kirjanmerkitä!
	</p>
	<div class="mx-10 flex gap-3">
		<input
			on:input={(e) => {
				load(e.target.value);
				e.target.value = "";
			}}
			type="text"
			class="px-1 w-32 bg-stone-800 outline-none"
		/>
		<!-- <button
					on:click={() => parse(textfield)}
					class="border rounded border-white hover:bg-white/80 bg-white/50 p-1"
					>Paina jos ei toimi</button
				> -->
	</div>
	<p class="max-w-[400px] min-w-[200px]">
		Jos joku TSYKin kurssi puuttuu tietokannasta, pyrin lisäämään sen sinne muutaman päivän sisällä.
		-M
	</p>
	{#if schedule}
		<div class="bg p-5 flex justify-center">
			<table>
				<tr>
					<th class="font-bold">Päivä</th>
					<th class="font-bold">Kurssi</th>
					<th class="font-bold">#</th>
					<th class="font-bold">Ruokailu</th>
					<th class="font-bold">Välitunti</th>
					<th class="font-bold">Oppitunti</th>
				</tr>
				{#each weekdays as wd, i}<tr>
						<td>{wd}</td>
						{#if schedule.getThirdCourse(i)}
							<td>{schedule.getThirdCourse(i)}</td>
							<td>
								{getSlot(i, schedule.getThirdCourse(i)).num}
							</td>
							<td class="whitespace-nowrap">
								{getSlot(i, schedule.getThirdCourse(i)).lunchTime}
							</td>
							<td class="whitespace-nowrap">
								{getSlot(i, schedule.getThirdCourse(i)).breakTime}
							</td>
							<td class="whitespace-nowrap">
								{getSlot(i, schedule.getThirdCourse(i)).lessonTime}
							</td>
						{:else}
							<td>hyppy</td>
							<td class="whitespace-nowrap">-</td>
							<td class="whitespace-nowrap">-</td>
							<td class="whitespace-nowrap">-</td>
							<td class="whitespace-nowrap">-</td>
						{/if}
					</tr>
				{/each}
			</table>
			<!-- <pre class="whitespace-pre-wrap">{JSON.stringify(rkv, null, 2)}</pre> -->
		</div>
	{/if}
</div>

<style lang="less">
	table tr td,
	table tr th {
		padding: 5px 20px;
	}
	code {
		white-space: nowrap;
		padding: 0px 5px;
		display: inline-block;
		background-color: var(--bg-color);
		font-family: monospace;
	}
	h1 {
		@apply text-3xl font-bold;
	}
</style>
