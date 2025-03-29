<script>
	import SvelteMarkdown from 'svelte-markdown';

	let { data } = $props();
</script>

<SvelteMarkdown source={data.} />
