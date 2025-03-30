import { SKETCHES } from '$lib/utils';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const description = await (await fetch(`/sketches/descriptions/${params.sketchName}.md`)).text();
	return {
		sketchName: SKETCHES.getKey(params.sketchName),
		description
	};
};
