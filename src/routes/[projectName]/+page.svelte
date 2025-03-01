<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';

	let { data } = $props();
</script>

<a href="/">Home</a>

<AspectRatio style="max-height: 60%" ratio={16 / 9}>
	<iframe
		src={`/${data.projectName}/nannou-embed`}
		class="responsive-iframe"
		title="Embedded Nannou app"
	>
	</iframe>
</AspectRatio>

<style>
	.responsive-iframe {
		position: absolute;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
		width: 100%;
		height: 100%;
		border: none;
	}
</style>
