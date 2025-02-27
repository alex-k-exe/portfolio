<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';

	let projects = ['Given example', 'Three squares'];
</script>

{#each projects as project, i}
	<div class="pb-10">
		<a href={`/${i}`}>{project}</a>
		<AspectRatio ratio={16 / 9}
			><img
				src={`$lib/assets/${project}.png`}
				alt={`Thumbnail for project named "${project}"`}
			/></AspectRatio
		>
	</div>
{/each}
