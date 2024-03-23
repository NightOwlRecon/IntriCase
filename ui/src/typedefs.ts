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

type Question = {
	id: string;
	pretty_id: string;
	summary: string;
	details?: string;
	investigation: string;
	outcome?: string;
	creator: string;
	status: string;
	created: string;
};

type ActionItem = {
	id: string;
	pretty_id: string;
	summary: string;
	details?: string;
	outcome?: string;
	assignee?: string;
	creator: string;
	question: string;
	assigned?: string
	resolved?: string;
	created: string;
};

export type { ActionItem, Investigation, Question, User };
