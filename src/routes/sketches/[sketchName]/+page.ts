import { SKETCHES } from '$lib/utils';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const sketchName = SKETCHES.getKey(params.sketchName);
	if (!sketchName) error(404, `Name "${params.sketchName}" is invalid`);

	return {
		sketchName: sketchName,
		description: await (await fetch(`/sketches/descriptions/${params.sketchName}.md`)).text()
	};
};
