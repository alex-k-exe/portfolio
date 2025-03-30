import wretch from 'wretch';
import { GITHUB_PAT } from '$env/static/private';
import type { PageServerLoad } from './projects/[projectName]/$types';
import { error } from '@sveltejs/kit';

// Fetch the repository description
async function fetchRepoDescription(projectName: string) {
	const apiUrl = `https://api.github.com/repos/alex-k-exe/${projectName.toLowerCase().replaceAll(' ', '-')}`;

	return wretch(apiUrl)
		.headers({ Accept: 'application/vnd.github.v3+json' })
		.auth(`Bearer ${GITHUB_PAT}`)
		.get()
		.json((data) => data.description || 'No description provided');
}

export const load: PageServerLoad = async () => {
	const projectNames = ['Portfolio', 'PT App'];
	const projects = new Map<string, string>();

	try {
		for (const projectName of projectNames) {
			projects.set(projectName, await fetchRepoDescription(projectName));
		}
	} catch (e) {
		console.log(1, e.message);
		error(400, 'Cannot fetch repo description' + e.message);
	}

	return { sketches: ['Epicycloids', 'Three squares', 'Game of Life'], projects };
};
