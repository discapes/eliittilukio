<script lang="ts">
	import { API_URL } from "@/lib/def";

	let feedbackModal = true;
	async function sendFeedback(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const text = form.text.value as string;

		await fetch(API_URL + "/send_feedback", {
			method: "POST",
			body: JSON.stringify({
				text
			}),
			headers: {
				"Content-Type": "application/json"
			}
		});
		form.reset();
		alert("Kiitti palautteesta!");
	}
</script>

<div>
	<div class="modal flex items-center justify-center bg">
		<div class="bg-dark relative w-[400px] p-5 flex flex-col gap-5 items-start rounded">
			<h1 class="my-0">Palaute</h1>
			<form class="contents" on:submit|preventDefault={sendFeedback}>
				<textarea
					required
					class="bg p-1 w-full resize-y border outline-none rounded"
					name="text"
					rows="4"
				/>
				<input class="button" value="Lähetä" type="submit" />
			</form>
		</div>
	</div>
</div>
