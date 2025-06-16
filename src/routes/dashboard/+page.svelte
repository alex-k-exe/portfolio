<script lang="ts">
	import { hitlerQuotes, kanyeQuotes } from '$lib/consts';
	import type { Props } from './+page.server';
	import lookingUpMyName from '$lib/assets/lookingUpMyName.svg';

	let { data }: { data: Props } = $props();

	let reducingKanyeQuotes = [...kanyeQuotes];
	let reducingHitlerQuotes = [...hitlerQuotes];
	let revealQuoteAuthor = $state(false);
	let randomQuote = $state(getRandomQuote());

	/** Returns a random quote and its author

    Even chance of it being Kanye or Hitler

    Is a pure function
    */
	function getRandomQuote() {
		const random = Math.random();
		let randomQuote: { quote: string; author: string; index: number };
		if (random < 0.001) return { quote: 'Wake up', author: 'You', index: 0 };
		else if (random < 0.5) {
			const randomIndex = Math.floor(Math.random() * reducingKanyeQuotes.length);
			randomQuote = {
				quote: reducingKanyeQuotes[randomIndex],
				author: 'Kanye',
				index: randomIndex
			};
			if (!randomQuote.quote)
				randomQuote = {
					quote: 'Ran out of Kanye quotes, resetting',
					author: '',
					index: 0
				};
		} else {
			const randomIndex = Math.floor(Math.random() * reducingHitlerQuotes.length);
			randomQuote = {
				quote: reducingHitlerQuotes[randomIndex],
				author: 'Hitler',
				index: randomIndex
			};
			if (!randomQuote.quote)
				randomQuote = {
					quote: 'Ran out of Kanye quotes, resetting',
					author: '',
					index: 0
				};
		}
		return randomQuote;
	}
	function handleGetNewQuote() {
		revealQuoteAuthor = false;
		randomQuote = getRandomQuote();
		switch (randomQuote.author) {
			case 'Kanye': {
				reducingKanyeQuotes.splice(randomQuote.index, 1);
				break;
			}
			case 'Hitler': {
				reducingHitlerQuotes.splice(randomQuote.index, 1);
				break;
			}
			case '': {
				reducingKanyeQuotes = [...kanyeQuotes];
				reducingHitlerQuotes = [...hitlerQuotes];
			}
		}
	}
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta
		name="description"
		content="Cards that use APIs which I find useful, interesting, or funny"
	/>
</svelte:head>
<h2><span>Dashboard</span></h2>

Legend:
<div class="flex gap-2">
	<p class="bg-(--yellow)">Useful</p>
	<p class="bg-(--green)">Interesting</p>
	<p class="bg-(--pink)">Fun</p>
</div>

<div class="cards grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
	<div class="bg-(--yellow)">
		<h3>News</h3>
		{#await data.newsPromise}
			Loading...
		{:then newsResult}
			<!-- Error occurred -->
			{#if typeof newsResult === 'string'}
				{newsResult}
			{:else}{#if newsResult.articles.length === 0}
					No news is good news. Check back later
				{/if}
				{#each newsResult.articles as article, i (i)}
					<p>
						<a href={article.url}>{article.title}</a>
					</p>
				{/each}
			{/if}
		{/await}
	</div>
	<div class="bg-(--green)">
		<h3>Astronomy Picture of the Day</h3>
		{#await data.apodPromise}
			<p>Loading...</p>
		{:then apodResult}
			<img src={apodResult.url} alt={'APOD - ' + apodResult.title} class="max-h-150" />
			{apodResult.title}. Find out more at
			<a href="https://apod.nasa.gov/">NASA's website</a>
		{/await}
	</div>
	<div class="bg-(--green)">
		<h3>Looking up my name</h3>
		<img
			class="max-h-150"
			src={lookingUpMyName}
			alt="Screenshot of search engine results for my name"
		/>
		I'm actually quite proud of what shows up
	</div>
	<div class="bg-(--pink)">
		<h3>Kanye or Hitler?</h3>
		Try to guess whether this quote was said by Kanye or Hitler:
		<blockquote>{randomQuote.quote}</blockquote>
		{#if revealQuoteAuthor}
			<button onclick={() => (revealQuoteAuthor = false)}>Hide author</button>
			{randomQuote.author}
		{:else}
			<button onclick={() => (revealQuoteAuthor = true)}>Reveal author</button>
		{/if}
		<button onclick={handleGetNewQuote} class="float-end">Get new quote</button>
		<p style="margin-bottom: 0px;">
			Inspired by a <a href="https://www.youtube.com/watch?v=0RMdwA8GWB8&pp=ygUMa2FueWUgaGl0bGVy"
				>Garfunkel and Oates joke</a
			>
		</p>
	</div>
	<div class="bg-(--yellow)">
		<h3>Analytics for this website</h3>
		<a href="https://app.peasy.so/share/alexkammin.com">Public dashboard through Peasy</a>
	</div>
</div>

<style>
	.cards div {
		padding: 1rem;
		margin: 0.25rem;
		border-radius: 0.5rem;
	}

	h3 {
		line-height: 1.5;
	}
</style>
