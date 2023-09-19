<script lang="ts">
	import type { PageData } from "./$types";

	export let data: PageData;
</script>

<div class="bg p-10">
	<p>hi</p>
	{#if data.me}
		<p>{JSON.stringify(data.me, null, 2)}</p>
	{/if}
</div>

<style lang="less">
</style>
