export function convertProjectName(name: string): string {
    return name.toLowerCase().replace(' ', '-');
}