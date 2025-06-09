<script lang="ts">
	import '../app.css';
	import profileCircle from '$lib/assets/profileCircle.svg';
	import music from '$lib/assets/icons/music.svg';
	import musicBanned from '$lib/assets/icons/musicBanned.svg';

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
</script>

<div class="grid">
	<div class="hidden place-content-center border-r-2 border-b-2 border-red-500 md:flex">
		<a href="/" class="place-content-center"
			><img
				class="mb-0 h-9/10 w-9/10"
				src={profileCircle}
				alt="Logo - a badly drawn portrait of me"
			/></a
		>
	</div>
	<div class="col-span-2 flex items-center justify-around border-b-2 border-red-500 md:col-auto">
		<a href="/" class="mb-0 p-px md:hidden"
			><img src={profileCircle} alt="Logo - a badly drawn portrait of me" /></a
		>
		<h1 class="sr-only md:not-sr-only">Alex Kammin's portfolio</h1>
		<a href="/dashboard" class="text-xl md:hidden">Dashboard</a>
		<a href="/malware" class="text-xl md:hidden">Malware</a>
	</div>
	<div class="navigation lined hidden border-r-2 border-red-500 md:block">
		<span style="font-size: 2rem; line-height: 1.5;">Navigation</span>
		<a href="/">About me</a>
		<a href="/dashboard">Dashboard</a>
		<a href="/malware">Malware</a>

		<button onclick={toggleAudio}>
			{#if playMusic}
				<img src={music} alt="Play music icon" />
			{:else}
				<img src={musicBanned} alt="Pause music icon" />
			{/if}
		</button>
		<audio bind:this={musicElement} onended={handleSongEnd} src={`songs/${songs[0]}.mp3`}></audio>
	</div>
	<div class="lined col-span-2 overflow-y-scroll md:col-auto">
		{@render children()}
	</div>
</div>

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

	.navigation a {
		display: block;
	}
</style>
