# Portfolio

Originally I tried using Astro because I heard that it was good for static websites like portfolios, but when I added the Dashboard page I needed to run code on the server at runtime. I'm also a lot more familiar with Svelte.

## Inspiration and credit
The style was inspired Napolean Dynamite and the music is three songs from its soundtrack

[Sharlene Yap's portfolio](https://www.sharyap.com/)

I used the [font used for Excalidraw](https://plus.excalidraw.com/excalifont)

[Peasy](https://app.peasy.so/share/alexkammin.com) for analytics

## Struggles
Efficiently loading images
 - [Google Lighthouse](https://pagespeed.web.dev/) complains a lot if you don't use explicit sizes for images and efficient file formats
 - Svelte's `enhanced-img` library is good for this but I had (and still am) trouble setting it up

I had to learn about CSS sizing units to properly line elements up with the background lines

Light and dark mode
 - I'm not even sure if I should add a dark mode because it would look less like paper
 - I'd also have to change the fill color of the SVG icons
