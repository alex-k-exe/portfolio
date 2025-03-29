import wretch from 'wretch';
import { GITHUB_PAT } from '$env/static/private';

// Fetch the repository description
async function fetchRepoDescription(projectName: string) {
	const apiUrl = `https://api.github.com/repos/alex-k-exe/${projectName.toLowerCase().replaceAll(' ', '-')}`;

	return wretch(apiUrl)
		.headers({ Accept: 'application/vnd.github.v3+json' })
		.auth(`Bearer ${GITHUB_PAT}`)
		.get()
		.json((data) => data.description || 'No description provided');
}

export async function load() {
	const projectNames = ['Portfolio', 'PT App'];
	const projects = new Map<string, string>();

	for (const projectName of projectNames) {
		projects.set(projectName, await fetchRepoDescription(projectName));
	}

	return { sketches: ['Epicycloids', 'Three squares', 'Game of Life'], projects };
}
