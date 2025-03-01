<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';
	import { convertProjectName } from '$lib/utils';

	let projects = ['Given example', 'Three squares'];
</script>

<div class="flex flex-wrap gap-5">
	{#each projects as project, i}
		<div class="sm:w-full md:w-1/2 lg:w-1/3">
			<a href={`/${convertProjectName(project)}`}>{project}</a>
			<AspectRatio ratio={16 / 9}>
				<img
					src={`src/lib/assets/${convertProjectName(project)}.png`}
					alt={`Thumbnail for project named "${project}"`}
				/>
			</AspectRatio>
		</div>
	{/each}
</div>
