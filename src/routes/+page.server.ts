import wretch from 'wretch';
import { GITHUB_PAT } from '$env/static/private';
import type { PageServerLoad } from './projects/[projectName]/$types';

// Fetch the repository description
async function fetchRepoDescription(projectName: string, github_pat: string) {
	const apiUrl = `https://api.github.com/repos/alex-k-exe/${projectName.toLowerCase().replaceAll(' ', '-')}`;

	return wretch(apiUrl)
		.headers({ Accept: 'application/vnd.github.v3+json' })
		.auth(`Bearer ${github_pat}`)
		.get()
		.json((data) => data.description || 'No description provided');
}

export const load: PageServerLoad = async ({ platform }) => {
	const projectNames = ['Portfolio', 'PT App'];
	const projects = new Map<string, string>();

	for (const projectName of projectNames) {
		projects.set(
			projectName,
			await fetchRepoDescription(projectName, platform?.env.GITHUB_PAT ?? GITHUB_PAT)
		);
	}

	return { sketches: ['Epicycloids', 'Three squares', 'Game of Life'], projects };
};
