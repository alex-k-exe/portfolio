<script lang="ts">
	let { data } = $props();
</script>

<a href="/">Home</a>

<iframe
	src={`/${data.projectName}/nannou-embed`}
	class="responsive-iframe"
	title="Embedded Nannou app"
>
</iframe>

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
