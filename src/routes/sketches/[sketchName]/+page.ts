import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const description = await (await fetch(`/sketches/descriptions/${params.sketchName}.md`)).text();
	return {
		sketchName: params.sketchName,
		description
	};
};
