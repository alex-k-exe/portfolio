export function load({ params }) {
	let id = Number(params.project);

	return {
		id: isNaN(id) ? 1 : id
	};
};