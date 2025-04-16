<script lang="ts">
	import { SKETCHES } from '$lib/utils.js';

	let { data } = $props();
</script>

<svelte:head>
	<meta name="description" content={`${data.sketchName} - interactive sketch with explanation`} />
	<title>Sketch - {data.sketchName}</title>
</svelte:head>

<a href="/" class="text-3xl">Home</a>

<iframe
	src={`/sketches/${SKETCHES.getValue(data.sketchName)}/nannou-embed`}
	width="1024px"
	height="768px"
	title="Embedded Nannou app"
>
</iframe>

<!-- <SvelteMarkdown source={data.description}></SvelteMarkdown> -->
Hello
