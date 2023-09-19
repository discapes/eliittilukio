<script lang="ts">
	import { invalidateAll } from "$app/navigation";
	import { API_URL } from "@/lib/def";
	import type { PageData } from "./$types";

	export let data: PageData;

	async function logout() {
		await fetch(API_URL + "/logout", {
			method: "POST"
		});
		invalidateAll();
	}
</script>

<div class="bg p-10">
	<p>hi</p>
	{#if data.me}
		<p>{JSON.stringify(data.me, null, 2)}</p>
		<button on:click={logout}> Log out </button>
	{:else}
		<a href="/login">Login</a>
		<a href="/register">Register</a>
	{/if}
</div>

<style lang="less">
</style>
