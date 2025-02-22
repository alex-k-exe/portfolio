<script lang="ts">
	let { id: string } = $props();

	await import(`../../crates/nannou-${id}/pkg/nannou_web_test.js`).then(
		async ({ default: nannou }) => {
			await nannou().then((s: { main_web: () => void }) => {
				s.main_web();
			});
		}
	);
</script>
