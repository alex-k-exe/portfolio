<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';
	import { convertName } from '$lib/utils';

	let { data } = $props();
</script>

<h2>Interactive creative coding visualisations</h2>
<div class="flex flex-wrap">
	{#each data.sketches as sketch}
		<div class="w-full p-2 sm:w-1/2 lg:w-1/3">
			<a href={`/sketches/${convertName(sketch)}`}
				>{sketch}
				<AspectRatio ratio={16 / 9}>
					<img src={`${convertName(sketch)}.png`} alt={`Thumbnail for sketch named "${sketch}"`} />
				</AspectRatio></a
			>
		</div>
	{/each}
</div>

<h2>Coding projects</h2>
<div class="flex flex-wrap">
	{#each data.projects as [name, description]}
		<div class="w-full p-2 sm:w-1/2 lg:w-1/3">
			<a href={`/projects/${convertName(name)}`}
				>`${name}: ${description}`
				<AspectRatio ratio={16 / 9}>
					<img src={`${convertName(name)}.png`} alt={`Thumbnail for project named "${name}"`} />
				</AspectRatio></a
			>
		</div>
	{/each}
</div>

<h2>Extended essay</h2>
3500-word essay evaluating the outsourcing of a SOC rather than social engineering training for a small
business
<a href="/ee.pdf">Read it here</a>
