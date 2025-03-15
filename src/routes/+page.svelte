<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';
	import { convertProjectName } from '$lib/utils';

	let projects = ['Epicycloids', 'Three squares', 'Game of Life'];
</script>

<div class="flex flex-wrap gap-5">
	{#each projects as project}
		<div class="sm:w-full md:w-1/2 lg:w-1/3">
			<a href={`/${convertProjectName(project)}`}>{project}</a>
			<AspectRatio ratio={16 / 9}>
				<img
					src={`${convertProjectName(project)}.png`}
					alt={`Thumbnail for project named "${project}"`}
				/>
			</AspectRatio>
		</div>
	{/each}
</div>
