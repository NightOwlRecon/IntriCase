import { type Writable, writable } from 'svelte/store';

import { type User } from './typedefs';

let userDetails: Writable<undefined | User> = writable(undefined);

let users: Writable<[]> = writable([]);

export { userDetails, users };
