<script lang="ts">
    import {
        hitlerQuotes,
        kanyeQuotes,
        NASA_API_KEY,
        NEWS_API_KEY,
    } from "../consts";
    import wretch from "wretch";
    import "../styles/global.css";

    let apodPromise: Promise<{ title: string; url: string }> = wretch(
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

    let reducingKanyeQuotes = kanyeQuotes;
    let reducingHitlerQuotes = hitlerQuotes;
    let revealQuoteAuthor = $state(false);

    /** Returns a random quote and its author

    Even chance of it being Kanye or Hitler
    */
    function getRandomQuote() {
        revealQuoteAuthor = false;

        const random = Math.random();
        let randomQuote: { quote: string; author: string };
        if (random < 0.001) return { quote: "Wake up", author: "You" };
        else if (random < 0.5) {
            const randomIndex = Math.floor(
                Math.random() * reducingKanyeQuotes.length,
            );
            randomQuote = {
                quote: reducingKanyeQuotes[randomIndex],
                author: "Kanye",
            };
            reducingKanyeQuotes.splice(randomIndex, 1);
            if (!randomQuote.quote) console.log(randomIndex);
        } else {
            const randomIndex = Math.floor(
                Math.random() * reducingHitlerQuotes.length,
            );
            randomQuote = {
                quote: reducingHitlerQuotes[randomIndex],
                author: "Hitler",
            };
            reducingHitlerQuotes.splice(randomIndex, 1);
            if (!randomQuote.quote) console.log(randomIndex);
        }
        if (!randomQuote.quote) {
            console.log(reducingKanyeQuotes, reducingHitlerQuotes);
            randomQuote = { quote: "Ran out of quotes", author: "" };
            reducingKanyeQuotes = kanyeQuotes;
            reducingHitlerQuotes = hitlerQuotes;
            console.log(kanyeQuotes, reducingKanyeQuotes, reducingHitlerQuotes);
        }
        return randomQuote;
    }

    let randomQuote = $state(getRandomQuote());
</script>

<div class="cards">
    <div class="card">
        <h2>Astronomy Picture of the Day</h2>
        {#await apodPromise}
            <p>Loading...</p>
        {:then apodResult}
            <img
                src={apodResult.url}
                alt={"APOD - " + apodResult.title}
                class="w-full"
            />
            {apodResult.title}. Find out more at
            <a href="https://apod.nasa.gov/apod/astropix.html">NASA's website</a
            >
        {/await}
    </div>
    <div class="card">
        <h2>Looking up my name</h2>
        <img
            class="w-full"
            src="/src/components/assets/lookingUpMyName.svg"
            alt="Screenshot of search engine results for my name"
        />
        I'm actually quite proud of what shows up
    </div>
    <div class="card">
        <h2>News</h2>
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
    <div class="card">
        <h2>Kanye or Hitler?</h2>
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
        <button class="block" onclick={() => (randomQuote = getRandomQuote())}
            >Get new quote</button
        >
    </div>
</div>

<p>
    Check out the <a href="https://app.peasy.so/share/alexkammin.com"
        >analytics for this website</a
    >
</p>

<style>
    .cards {
        display: grid;
        grid-template-columns: 1fr 1fr;
    }

    .card {
        padding: 5px;
        border: 10, solid, grey;
    }

    h2 {
        margin-top: 0px;
        padding-top: 0px;
    }
</style>
