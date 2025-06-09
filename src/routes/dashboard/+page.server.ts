import { NASA_API_KEY, NEWS_API_KEY } from '$env/static/private';
import { error } from '@sveltejs/kit';
import wretch from 'wretch';

export type Props = {
	apodPromise: Promise<{ title: string; hdurl: string }>;
	newsPromise: Promise<{ articles: { title: string; url: string }[] }>;
};

export async function load({ platform }) {
	if (!platform?.env) {
		return error(500);
	}
	const apodPromise: Props['apodPromise'] = wretch(
		`https://api.nasa.gov/planetary/apod?api_key=${platform.env.NASA_API_KEY ?? NASA_API_KEY}`
	)
		.get()
		.json();

	const newsPromise: Props['newsPromise'] = wretch(
		`https://newsapi.org/v2/top-headlines?apiKey=${platform.env.NEWS_API_KEY ?? NEWS_API_KEY}&pageSize=3&country=us`
	)
		.get()
		.json();

	return { apodPromise, newsPromise };
}
