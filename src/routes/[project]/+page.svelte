<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';

	let { data } = $props();
</script>

<a href="/">Home</a>

<AspectRatio ratio={800 / 600}>
	<iframe src={`/${data.id}/nannou-embed`} class="h-full w-full" title="Embedded Nannou app"
	></iframe>
</AspectRatio>
