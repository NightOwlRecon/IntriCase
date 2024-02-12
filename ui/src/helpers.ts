// submits a form's data asynchronously as a JSON blob,
// deriving all values from the form itself
// you can wrap this in another function to deal with return data and perform
// context-specific actions
const handleSubmitJson = async (e: Event): Promise<Response> => {
	if (!(e.target instanceof HTMLFormElement)) throw new Error('Not called on HTMLFormElement');
	const res = await fetch(e.target.action, {
		headers: new Headers({ 'Content-Type': 'application/json' }),
		method: e.target.method,
		body: JSON.stringify(Object.fromEntries(new FormData(e.target))),
	});
	// TODO: handle redirects
	return res;
};

// parses out the value of a single entry in a string of cookies
// returns undefined if the named value does not exist
const cookieValue = (s: string): string | undefined => {
	const val = document.cookie
		.split('; ')
		.find((row) => row.startsWith(`${s}=`))
		?.split('=')[1];
	if (!val) return;

	return decodeURIComponent(val);
};

const handleUriPath = (): string[] => {
	// let's see if we have a hash
	if (!!document.location.hash) {
		let path = document.location.hash.split('/');
		// and whether the hash looks good and there are more elements
		if (path[0] === '#!' && path.length > 1) {
			// remove the first (#!) element and return the rest (we can do this recursively to shift off an entry at each layer in nested components)
			path.shift();
			return path;
		}
	}
	return [];
};

export { cookieValue, handleSubmitJson, handleUriPath };
