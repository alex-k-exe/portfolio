export function convertProjectName(name: string): string {
	return name.trim().toLowerCase().replaceAll(' ', '_');
}
