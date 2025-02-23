<script lang="ts">
	import { onMount } from 'svelte';

	let { data } = $props();

	console.log(data.id);

	async function loadNannou() {
		await import(`$lib/crates/nannou-${data.id}/pkg/nannou_web_test.js`).then(
			async ({ default: nannou }) => {
				await nannou().then((s: { main_web: () => void }) => {
					console.log(`aaaeee ${data.id}`);
					s.main_web();
				});
			}
		);
	}

	// Call the function when the component mounts
	onMount(loadNannou);
</script>
