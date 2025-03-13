<script lang="ts">
	import { onMount } from 'svelte';

	let { data } = $props();

	async function loadNannou() {
		await import(`$lib/crates/${data.projectName}/pkg/nannou_web_test.js`).then(
			async ({ default: nannou }) => {
				console.log(2);
				await nannou().then((s: { main_web: () => void }) => {
					s.main_web();
				});
				console.log(3);
			}
		);
	}

	console.log(1);
	// Call the function when the component mounts
	onMount(loadNannou);
</script>
