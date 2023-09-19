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

<div class="modal flex items-center justify-center">
	<div class="bg-dark relative w-[400px] p-5 bg flex flex-col gap-5 items-start rounded">
		<h3>Palauteboksi</h3>
		<form class="contents" on:submit|preventDefault={sendFeedback}>
			<textarea required class="resize-y w-full" name="text" rows="4" />
			<input class="button" value="Lähetä" type="submit" />
		</form>
	</div>
</div>
