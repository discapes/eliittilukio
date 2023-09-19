<script lang="ts">
	import type { PageData } from "./$types";
	import InstagramEmbed from "./InstagramEmbed.svelte";

	export let data: PageData;

	const links = [
		{
			name: "Wilma",
			logo: "wilma.png",
			url: "https://turku.inschool.fi/"
		},
		{
			name: "TSYKin blogi",
			logo: "wordpress.png",
			url: "https://blog.edu.turku.fi/tsyk/"
		},
		{
			name: "Office 365",
			logo: "office.png",
			url: "https://login.microsoftonline.com/?whr=edu.turku.fi"
		},
		{
			name: "Moodle",
			logo: "moodle.png",
			url: "https://lukio.edu.turku.fi/"
		},
		{
			name: "Viikon ruuat",
			logo: "kaarea.png",
			url: "https://menu.kaarea.fi/KaareaAromieMenus/FI/Default/Kaarea/TSYKLU/Restaurant.aspx"
		},
		{
			name: "Otava",
			logo: "otava.svg",
			url: "https://opiskelija.otava.fi/materiaalit/omat"
		},
		{
			name: "Studeo",
			logo: "studeo.svg",
			url: "https://app.studeo.fi/app/home"
		},
		{
			name: "Sanomapro",
			logo: "sanomapro.png",
			url: "https://kampus.sanomapro.fi/"
		}
	];
</script>

<div class="bg p-10 flex flex-col gap-10">
	<div class="flex gap-10 items-center">
		<img class="inline-block h-[100px]" alt="tsyk logo" src="/favicon.png" />
		<div>
			<h2>Eliittilukio.fi</h2>
			<h3>eli siis TSYK</h3>
			<h4 class="mb-0">
				Tällä hetkellä täältä löytyy linkkejä, oispaeliitti, ruokavuorolaskuri ja palauteboksi.
			</h4>
		</div>
	</div>
	<div class="flex flex-wrap gap-5">
		{#each links as link}
			<a class="bg-white text-black flex flex-col items-center p-3" target="_blank" href={link.url}>
				{link.name}
				<div class="linkimage m-2" style="background-image: url('/logos/{link.logo}');" /></a
			>
		{/each}
	</div>
	<div class="flex justify-evenly">
		<InstagramEmbed username="tsykopkh" />
		<InstagramEmbed username="tsykinlukio" />
	</div>
</div>

<style lang="less">
	.linkimage {
		width: 60px;
		height: 60px;
		display: block;
		background-position: center center;
		background-color: white;
		background-size: contain;
		background-repeat: no-repeat;
	}
</style>
