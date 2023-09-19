<script lang="ts">
	import navItems from "@/data/pages";
	import { page } from "$app/stores";
	import { API_URL } from "@/lib/def";
	import { invalidateAll } from "$app/navigation";

	async function logout() {
		await fetch(API_URL + "/logout", {
			method: "POST"
		});
		invalidateAll();
	}
</script>

<header>
	<div class="w-full" />
	<nav class="w-full">
		<ul>
			{#each navItems as navItem}
				<li aria-current={$page.url.pathname === navItem.path ? "page" : undefined}>
					<a href={navItem.path}>{navItem.name}</a>
				</li>
			{/each}
		</ul>
	</nav>
	<div class="flex gap-3 w-full justify-end items-center p-1">
		{#if $page.data.me}
			<p class="bg p-1">Logged in as {$page.data.me.username}</p>
			<button class="button" on:click={logout}> Log out </button>
		{:else}
			<a class="button" href="/login">Login</a>
			<a class="button" href="/register">Register</a>
		{/if}
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
		background-color: var(--bg-color);
	}

	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background-size: contain;
	}

	li {
		position: relative;
		height: 100%;
	}

	li[aria-current="page"]::before {
		--size: 6px;
		content: "";
		width: 0;
		height: 0;
		position: absolute;
		top: 0;
		left: calc(50% - var(--size));
		border: var(--size) solid transparent;
		border-top: var(--size) solid var(--color-theme-1);
	}

	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0.5rem;
		color: var(--color-text);
		font-weight: 700;
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}

	a:hover {
		text-shadow: #fff 1px 0 10px;
	}
</style>
