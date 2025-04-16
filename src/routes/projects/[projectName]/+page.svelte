<script>
	let { data } = $props();
</script>

<svelte:head>
	<meta name="description" content={`Explanation of project named ${data.projectName}`} />
	<title>Project - {data.projectName}</title>
</svelte:head>

<a href="/" class="text-3xl">Home</a>

<!-- <SvelteMarkdown source={data.markdownContent} /> -->
Hello
