<script lang="ts">
	const qna = [
		[
			"[palautteet mobiiliversiosta ja isoista näytöistä]",
			"Paransin responsiivisuutta, ilmoittakaa jos on vielä ongelmia ja millä näytöillä.",
		],
		["Linkkeihin voisi lisätä myös Editan", "Lisätty."],
		[
			"uusi palikka kiukaan jälkeen oispa eliittiin kiitos",
			"Palautteeseen voi lähettää ehdotuksia uusille palikoille, lisään ne mielellään.",
		],
		[
			"Keskiviikon ENA01.6 ei näy",
			"Se ei näy myöskään tsykin virallisessa ruokavuorotaulukossa, joten en tiedä mihin kohtaan se kuuluu.",
		],
		[
			"Ruokailuvuorolaskuri ei näytä ollenkaan tiistai ruokavuoroja!!",
			"Jos tolleen käy niin laittakaa palautteeseen ko. tunti niin tiedän missä ongelma on.",
		],
		[
			"ruakavuorot eivät jää selaimen välimuistiin, vaan ne pitää kopioida aina uudestaan.",
			"Korjasin yhden mahdollisen syyn, toivottavasti toimii nyt.",
		],
		["kissi mirri", "joo"],
	];
</script>

<div class="bg p-3 flex flex-col gap-5">
	<h3>Ilmoitukset</h3>
	<div class="p-3 bg max-w-md">
		<h4>terkut kerlun ICT linjalle</h4>
		<p>
			hei miks flexaatte feikkiscoreilla, kaikki kuiteski tietää et niiden lisäämiseen menee 30
			sekkaa ja sen voi tehdä jokanen skidi jolla on selain. cmon, noloa
		</p>
		<span class="text-stone-400">29.10.</span>
	</div>
	<div class="p-3 bg max-w-md">
		<h4>vastauksia palautteisiin</h4>
		{#each qna as [q, a]}
			<p>
				<span class="text-rose-200">&gt; {q}</span><br />
				<span class="text-lime-200">{a}</span>
			</p>
		{/each}
		<span class="text-stone-400">29.10.</span>
	</div>
</div>
