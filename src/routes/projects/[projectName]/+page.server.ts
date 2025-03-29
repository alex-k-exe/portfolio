import { GITHUB_PAT } from '$env/static/private';
import wretch from 'wretch';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const repoName = params.projectName.replaceAll('_', '-');
	const markdownContent = await wretch(
		`https://raw.githubusercontent.com/alex-k-exe/${repoName}/main/README.md`
	)
		.auth(`Bearer ${GITHUB_PAT}`)
		.get()
		.text();

	return { markdownContent };
};
