import { z } from 'zod';

type Investigation = {
	id: string;
	internal_id?: string;
	first_name: string;
	middle_name?: string;
	last_name: string;
	date_of_birth: string;
	namus_id?: string;
	missing_since: string;
	synopsis: string;
	created: string;
};

type User = {
	id: string;
	display_name: string;
	email: string;
	enabled: 'enabled' | 'disabled';
	auth_date: string;
};

export type { Investigation, User };
