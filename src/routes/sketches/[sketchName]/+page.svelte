<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown';

	let { data } = $props();
</script>

<a href="/" class="text-3xl">Home</a>

<iframe
	src={`/sketches/${data.sketchName}/nannou-embed`}
	width="1024px"
	height="768px"
	title="Embedded Nannou app"
>
</iframe>

<SvelteMarkdown source={data.description}></SvelteMarkdown>
