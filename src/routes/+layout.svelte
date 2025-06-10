<script lang="ts">
	import '../app.css';
	import profileCircle from '$lib/assets/profileCircle.svg';
	import music from '$lib/assets/icons/music.svg';
	import musicBanned from '$lib/assets/icons/musicBanned.svg';
	import { page as pageState } from '$app/state';

	let { children } = $props();

	let musicElement: HTMLAudioElement;
	let playMusic = $state(false);
	const songs = ['thrifty', 'napPullsKipReturn', 'napRicoVan'] as const;
	let currentSong = 0;

	function toggleAudio() {
		playMusic = !playMusic;
		if (playMusic) musicElement.play();
		else musicElement.pause();
	}

	function handleSongEnd() {
		currentSong = (currentSong + 1) % songs.length;
		musicElement.src = `/songs/${songs[currentSong]}.mp3`;
		musicElement.play();
	}

	const pages: { href: string; title: string }[] = [
		{ href: '/', title: 'About me' },
		{ href: '/dashboard', title: 'Dashboard' },
		{ href: '/malware', title: 'Malware' }
	];
</script>

<div class="grid">
	<a href="/" class="hidden place-content-center border-r-2 border-b-2 border-red-500 md:flex"
		><img
			class="max-w-1"
			style="margin-bottom: 0px;"
			src={profileCircle}
			alt="Logo - a badly drawn portrait of me"
		/></a
	>
	<div class="col-span-2 flex items-center justify-around border-b-2 border-red-500 md:col-auto">
		<a href="/" class="inline-flex items-center justify-center md:hidden"
			><img
				src={profileCircle}
				style="margin-bottom: 0px;"
				class="w-20"
				alt="Logo - a badly drawn portrait of me"
			/></a
		>
		<h1 class="sr-only md:not-sr-only">Alex Kammin's portfolio</h1>
		{#each pages.slice(1) as page, i (i)}
			<a href={page.href} class="text-xl md:hidden">{page.title}</a>
		{/each}
		<button onclick={toggleAudio} class="musicButton w-10">
			{#if playMusic}
				<img src={music} alt="Play music icon" />
			{:else}
				<img src={musicBanned} alt="Pause music icon" />
			{/if}
		</button>
	</div>
	<div class="lined hidden border-r-2 border-red-500 md:block">
		<span style="font-size: 2rem; line-height: 1.5;">Navigation</span>

		{#each pages as page, i (i)}
			<p style="margin-bottom: 0px;">
				<a href={page.href} class={page.href == pageState.url.pathname ? ' bg-(--yellow)' : ''}
					>{page.title}</a
				>
			</p>
		{/each}

		<button
			onclick={toggleAudio}
			style="background-color: transparent; max-width: 125px; align-self: flex-end; justify-content: start;"
		>
			{#if playMusic}
				<img src={music} alt="Play music icon" />
			{:else}
				<img src={musicBanned} alt="Pause music icon" />
			{/if}
		</button>
	</div>
	<div class="lined col-span-2 overflow-y-scroll md:col-auto">
		{@render children()}
	</div>
</div>
<audio bind:this={musicElement} onended={handleSongEnd} src={`songs/${songs[0]}.mp3`}></audio>

<style>
	.grid {
		width: 100vw;
		height: 100vh;
		grid-template-columns: 1fr 4fr;
		grid-template-rows: 1fr 8fr;
		position: relative;
	}

	.grid div {
		padding: 0px 10px;
	}

	.lined {
		background-image: linear-gradient(var(--background-blue) 0.05em, transparent 0.05em);
		background-size: 100% 1.5em;
	}

	.musicButton {
		background-color: transparent;
		@media (width >= 48rem /* 768px */) {
			display: none;
		}
		margin: 0px;
		padding: 0px;
	}

	.musicButton img {
		margin: 0px;
	}
</style>
