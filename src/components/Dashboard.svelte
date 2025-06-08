<script lang="ts">
    import {
        hitlerQuotes,
        kanyeQuotes,
        NASA_API_KEY,
        NEWS_API_KEY,
    } from "../consts";
    import wretch from "wretch";
    import "../styles/global.css";

    let apodPromise: Promise<{ title: string; hdurl: string }> = wretch(
        `https://api.nasa.gov/planetary/apod?api_key=${NASA_API_KEY}`,
    )
        .get()
        .json();

    let newsPromise: Promise<{ articles: { title: string; url: string }[] }> =
        wretch(
            `https://newsapi.org/v2/top-headlines?apiKey=${NEWS_API_KEY}&pageSize=4&country=us`,
        )
            .get()
            .json();

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
        if (random < 0.001)
            return { quote: "Wake up", author: "You", index: 0 };
        else if (random < 0.5) {
            const randomIndex = Math.floor(
                Math.random() * reducingKanyeQuotes.length,
            );
            randomQuote = {
                quote: reducingKanyeQuotes[randomIndex],
                author: "Kanye",
                index: randomIndex,
            };
            if (!randomQuote.quote)
                randomQuote = {
                    quote: "Ran out of Kanye quotes, resetting",
                    author: "",
                    index: 0,
                };
        } else {
            const randomIndex = Math.floor(
                Math.random() * reducingHitlerQuotes.length,
            );
            randomQuote = {
                quote: reducingHitlerQuotes[randomIndex],
                author: "Hitler",
                index: randomIndex,
            };
            if (!randomQuote.quote)
                randomQuote = {
                    quote: "Ran out of Kanye quotes, resetting",
                    author: "",
                    index: 0,
                };
        }
        return randomQuote;
    }
    function handleGetNewQuote() {
        revealQuoteAuthor = false;
        randomQuote = getRandomQuote();
        switch (randomQuote.author) {
            case "Kanye": {
                reducingKanyeQuotes.splice(randomQuote.index, 1);
                break;
            }
            case "Hitler": {
                reducingHitlerQuotes.splice(randomQuote.index, 1);
                break;
            }
            case "": {
                reducingKanyeQuotes = [...kanyeQuotes];
                reducingHitlerQuotes = [...hitlerQuotes];
            }
        }
    }
</script>

<svelte:head>
    <title>Dashboard</title>
    <meta name="description" content="APIs " />
</svelte:head>
<h2>Dashboard</h2>

<p style="margin-bottom: 5px;">Legend:</p>
<div class="flex gap-2">
    <p class="bg-(--yellow)">Things I genuinely use</p>
    <p class="bg-(--green)">About me</p>
    <p class="bg-(--pink)">Funny</p>
</div>

<div class="cards grid grid-cols-2">
    <div class="bg-(--yellow)">
        <h3>Astronomy Picture of the Day</h3>
        {#await apodPromise}
            <p>Loading...</p>
        {:then apodResult}
            <img
                src={apodResult.hdurl}
                alt={"APOD - " + apodResult.title}
                class="max-h-150"
            />
            {apodResult.title}. Find out more at
            <a href="https://apod.nasa.gov/apod/astropix.html">NASA's website</a
            >
        {/await}
    </div>
    <div class="bg-(--green)">
        <h3>Looking up my name</h3>
        <img
            class="max-h-150"
            src="/src/components/assets/lookingUpMyName.svg"
            alt="Screenshot of search engine results for my name"
        />
        I'm actually quite proud of what shows up
    </div>
    <div class="bg-(--yellow)">
        <h3>News</h3>
        {#await newsPromise}
            Loading...
        {:then newsResult}
            {#each newsResult.articles as article}
                <p>
                    <a href={article.url}>{article.title}</a>
                </p>
            {/each}
        {/await}
    </div>
    <div class="bg-(--pink)">
        <h3>Kanye or Hitler?</h3>
        Try to guess whether this quote was said by Kanye or Hitler:
        <blockquote>{randomQuote.quote}</blockquote>
        {#if revealQuoteAuthor}
            <button onclick={() => (revealQuoteAuthor = false)}
                >Hide author</button
            >
            {randomQuote.author}
        {:else}
            <button onclick={() => (revealQuoteAuthor = true)}
                >Reveal author</button
            >
        {/if}
        <br />
        <button onclick={handleGetNewQuote}>Get new quote</button>
        <br />
        Inspired by a
        <a
            href="https://www.youtube.com/watch?v=0RMdwA8GWB8&pp=ygUMa2FueWUgaGl0bGVy"
            >Garfunkel and Oates joke</a
        >
    </div>
    <div class="bg-(--green)">
        <h3>Website analytics</h3>
        <a href="https://app.peasy.so/share/alexkammin.com"
            >https://app.peasy.so/share/alexkammin.com</a
        >
    </div>
</div>

<style>
    * {
        --yellow: #fff2ab;
        --green: #cbf1c4;
        --pink: #ffcce5;
    }

    .cards div {
        padding: 1rem;
        margin: 0.25rem;
        border-radius: 8px;
    }

    h2 {
        margin-top: 0px;
        padding-top: 0px;
    }
</style>
