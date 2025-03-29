<script>
	import SvelteMarkdown from 'svelte-markdown';

	let { data } = $props();
</script>

<a href="/" class="text-3xl">Home</a>

<SvelteMarkdown source={data.markdownContent} />
