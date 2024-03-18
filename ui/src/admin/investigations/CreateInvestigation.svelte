<script lang="ts">
	import { Button, Heading, Input, Label, Textarea } from 'flowbite-svelte';
	import {
		handleSubmitJson,
		highlightZodErrors,
		objectFromForm,
		resetZodErrors,
	} from '../../helpers';
	import { z, ZodError } from 'zod';

	const CreateInvestigationSchema = z.object({
		first_name: z.string(),
		middle_name: z.string().optional(),
		last_name: z.string(),
		date_of_birth: z
			.string()
			.regex(
				/^[0-9]{1,2}\/[0-9]{1,2}\/[0-9]{4}/,
				'Date of birth must be in the format of "MM/DD/YYYY"',
			),
		missing_since: z
			.string()
			.regex(
				/^[0-9]{1,2}\/[0-9]{1,2}\/[0-9]{4}/,
				'Missing since must be in the format of "MM/DD/YYYY"',
			),
		internal_id: z.string().optional(),
		namus_id: z
			.string()
			.regex(/^mp[0-9]+$/i, 'NamUs ID must be in the format of "MP12345"')
			.toLowerCase()
			.optional(),
		synopsis: z.string(),
	});

	//https://www.namus.gov/api/CaseSets/NamUs/MissingPersons/Cases/98673

	// TODO: break as much of this as makes sense out into helpers.ts
	const checkInput = async (e: Event) => {
		if (!e.target || !('form' in e.target))
			throw new Error('No event target, or not triggered by field of form');
		const form = e.target.form as HTMLFormElement;
		// triggered by a change, this event target *will* be a form element
		// and should have a form property pointing to the parent form
		try {
			resetZodErrors(form);
			CreateInvestigationSchema.parse(objectFromForm(form));
		} catch (err) {
			if (err instanceof ZodError) {
				// TODO: figure out how/where we want to spit out our error messages
				highlightZodErrors(form, err);
			}
		}
	};
	const submitCreateInvestigationForm = async (e: Event) => {
		const res = await handleSubmitJson(e);
		if (res.ok) {
			let data = await res.json();
			if (data.id) document.location = `/#/investigations/${data.id}`;
		}
	};
</script>

<Heading tag="h1" class="mt-4 mb-8">Create Investigation</Heading>
<form
	action="/api/admin/investigations/create"
	method="POST"
	on:input={checkInput}
	on:submit|preventDefault={submitCreateInvestigationForm}
>
	<Heading tag="h3" class="mb-4">Personal Information</Heading>
	<div class="flex mb-4">
		<div class="mr-4">
			<Label for="first_name">First Name</Label>
			<Input name="first_name" />
		</div>

		<div class="mr-4">
			<Label for="middle_name">Middle Name (optional)</Label>
			<Input name="middle_name" />
		</div>

		<div class="">
			<Label for="last_name">Last Name</Label>
			<Input name="last_name" />
		</div>
	</div>

	<div class="flex mb-8">
		<div class="">
			<Label for="date_of_birth">Date of Birth (MM/DD/YYYY)</Label>
			<Input name="date_of_birth" placeholder="MM/DD/YYYY" />
		</div>
	</div>

	<Heading tag="h3" class="mb-4">Case Information</Heading>
	<div class="flex mb-4">
		<div class="mr-4">
			<Label for="missing_since">Missing Since (MM/DD/YYYY)</Label>
			<Input name="missing_since" placeholder="MM/DD/YYYY" />
		</div>
		<div class="mr-4">
			<Label for="internal_id">Internal ID (optional)</Label>
			<Input name="internal_id" />
		</div>
		<div class="">
			<Label for="namus_id">NamUs ID (optional - MPxxxxx)</Label>
			<Input name="namus_id" placeholder="MP12345" />
		</div>
	</div>

	<div class="flex mb-8">
		<div class="w-full">
			<Label for="synopsis">Synopsis</Label>
			<Textarea name="synopsis" rows="8" class="w-full" />
		</div>
	</div>

	<Button type="submit" class="mb-4" color="blue">Submit</Button>
</form>
