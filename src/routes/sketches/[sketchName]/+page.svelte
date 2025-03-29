<script lang="ts">
	let { data } = $props();
</script>

<a href="/">Home</a>

<iframe
	src={`/sketches/${data.sketchName}/nannou-embed`}
	width="1024px"
	height="768px"
	title="Embedded Nannou app"
>
</iframe>
