import { type Writable, writable } from 'svelte/store';

import { type User } from './bindings/User';

import { type ListUser } from './typedefs';

let userDetails: Writable<undefined | User> = writable(undefined);

// TODO: may make more sense to have the standard User here and do any formatting in the component
let users: Writable<ListUser[]> = writable([]);

export { userDetails, users };
