<script lang="ts">
	import { page } from "$app/stores";
	import { invalidateAll, goto } from "$app/navigation";
	import { API_URL } from "@/lib/def";

	let message: string | undefined;

	async function onSubmit(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const username = form.username.value as string;
		const password = form.password.value as string;

		if (username.length && password.length) {
			const res = await fetch(API_URL + "/login", {
				body: JSON.stringify({
					username,
					password
				}),
				headers: {
					"Content-Type": "application/json"
				},
				method: "POST"
			});
			if (res.ok) {
				await invalidateAll();
				await goto("/");
			} else {
				message = await res.text();
			}
		}
	}
</script>

<div class="bg flex flex-col items-center m-10 p-10 gap-5">
	{#if message}
		<p>{message}</p>
	{/if}
	<form class="flex flex-col gap-4 w-32" on:submit|preventDefault={onSubmit}>
		<div>
			<label for="username">Username</label>
			<input type="text" id="username" class="bg w-full" name="username" autocomplete="username" />
		</div>
		<div>
			<label for="username">Password</label>
			<input
				type="password"
				id="password"
				class="bg w-full"
				name="password"
				autocomplete="current-password"
			/>
		</div>
		<input class="bg cursor-pointer" type="submit" value="Submit" />
	</form>
</div>

<style lang="less">
	input {
		outline: none;
	}
</style>
