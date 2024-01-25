// submits a form's data asynchronously as a JSON blob,
// deriving all values from the form itself
// you can wrap this in another function to deal with return data and perform
// context-specific actions
const handleSubmitJson = async (e: Event): Promise<Response> => {
	if (!(e.target instanceof HTMLFormElement)) throw new Error("Not called on HTMLFormElement");
	const res = await fetch(e.target.action, {
		headers: new Headers({ 'Content-Type': 'application/json' }),
		method: e.target.method,
		body: JSON.stringify(Object.fromEntries(new FormData(e.target)))
	});
	// TODO: handle redirects
	return res;
};

// parses out the value of a single entry in a string of cookies
// returns undefined if the named value does not exist
const cookieValue = (s: string, keepSig: boolean = false): string => {
	const val = document.cookie
		.split('; ')
		.find((row) => row.startsWith(`${s}=`))
		?.split('=')[1];
	if (!val) throw new Error("Cookie not found.");

	const decoded = decodeURIComponent(val);
	if (keepSig) return decoded;

	const cut = decoded.split('=', 2);
	return cut[1];
};

export { cookieValue, handleSubmitJson };
