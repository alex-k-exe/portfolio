import { GITHUB_PAT } from '$env/static/private';
import wretch from 'wretch';
import type { PageServerLoad } from './$types';
import { PROJECTS } from '$lib/utils';

export const load: PageServerLoad = async ({ params }) => {
	const markdownContent = await wretch(
		`https://raw.githubusercontent.com/alex-k-exe/${params.projectName}/main/README.md`
	)
		.headers({ 'User-Agent': 'alex-k-exe' })
		.auth(`Bearer ${GITHUB_PAT}`)
		.get()
		.text();

	return { markdownContent, projectName: PROJECTS.getKey(params.projectName) };
};
