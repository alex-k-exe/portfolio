/** Convert a user readable name for a sketch (e.g. Three squares) to file name
    Using "_" for sketches because that is the standard in Rust
*/
export function convertSketchName(name: string): string {
	return name.trim().toLowerCase().replaceAll(' ', '_');
}

/** Convert a user readable name for a sketch (e.g. Three squares) to file name
    Using "-" for projects because that is the standard in Github
*/
export function convertProjectName(name: string): string {
	return name.trim().toLowerCase().replaceAll(' ', '-');
}
