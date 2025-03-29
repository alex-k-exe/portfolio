export function convertName(name: string): string {
	return name.trim().toLowerCase().replaceAll(' ', '_');
}
