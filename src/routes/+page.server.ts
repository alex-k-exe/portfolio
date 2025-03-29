import { convertName } from '$lib/utils';
import wretch from 'wretch';

// Fetch the repository description
async function fetchRepoDescription(projectName: string) {
	const apiUrl = `https://api.github.com/repos/alex-k-exe/${projectName.toLowerCase().replaceAll(' ', '-')}`;

	return wretch(apiUrl)
		.headers({ Accept: 'application/vnd.github.v3+json' })
		.get()
		.json((data) => data.description || 'No description provided');
}

export async function load() {
	const projectNames = ['Portfolio', 'PT App'];
	const projects = new Map<string, string>();

	for (const projectName in projectNames) {
		console.log(`1 ${projectName} ${convertName(projectName)}`);
		projects.set(projectName, await fetchRepoDescription(convertName(projectName)));
	}

	return { sketches: ['Epicycloids', 'Three squares', 'Game of Life'], projects };
}
