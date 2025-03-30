<script lang="ts">
	import AspectRatio from '$lib/components/ui/aspect-ratio/aspect-ratio.svelte';
	import { PROJECTS, SKETCHES } from '$lib/utils';
</script>

<svelte:head>
	<meta name="description" content={`Alex Kammin's portfolio - home page`} />
	<title>Alex Kammin's portfolio</title>
</svelte:head>

<h1>Alex Kammin's portfolio</h1>

<h2>Coding projects</h2>
<div class="flex flex-wrap">
	{#each PROJECTS.entries() as projectName}
		<div class="w-full p-2 sm:w-1/2 lg:w-1/3">
			<a href={`/projects/${projectName[1]}`}
				>{projectName[0]}
				<AspectRatio ratio={16 / 9}>
					<img
						src={`projects/${projectName[1]}.png`}
						alt={`Thumbnail for project named "${projectName[0]}"`}
					/>
				</AspectRatio>
			</a>
		</div>
	{/each}
</div>

<h2>Sketches</h2>
Interactive creative coding visualisations
<div class="flex flex-wrap">
	{#each SKETCHES.entries() as sketchName}
		<div class="w-full p-2 sm:w-1/2 lg:w-1/3">
			<a href={`/sketches/${sketchName[1]}`}
				>{sketchName[0]}
				<AspectRatio ratio={16 / 9}>
					<img
						src={`sketches/thumbnails/${sketchName[1]}.png`}
						alt={`Thumbnail for sketch named "${sketchName[0]}"`}
					/>
				</AspectRatio>
			</a>
		</div>
	{/each}
</div>

<h2>Extended essay</h2>
3500-word essay I wrote for high school evaluating the outsourcing of a SOC rather than social engineering
training for a small business
<a href="/ee.pdf" class="block">Read it here</a>
