<script lang="ts">
	import { Button, Dropdown, Heading, Input, Label, Textarea } from 'flowbite-svelte';

	import {
		handleSubmitJson,
		highlightZodErrors,
		objectFromForm,
		resetZodErrors,
	} from '../../helpers';

	import { z, ZodError } from 'zod';
	import Fa from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';

	import type { CreateInvestigationDetails } from '../../bindings/CreateInvestigationDetails';
	import type { CreateQuestionDetails } from '../../bindings/CreateQuestionDetails';

	import QuestionItem from '../../investigations/Question.svelte';

	let newInvestigation: CreateInvestigationDetails = {
		first_name: '',
		middle_name: '',
		last_name: '',
		date_of_birth: '',
		missing_since: '',
		internal_id: '',
		namus_id: '',
		synopsis: '',
		questions: [],
	};
	// not sure if this is necessary
	$: newInvestigation.questions = [];

	const lol = () => {
		console.log(newInvestigation);
	};

	const CreateInvestigationSchema = z.object({
		first_name: z.string(),
		middle_name: z.string().optional(),
		last_name: z.string(),
		date_of_birth: z
			.string()
			.regex(
				/^[0-9]{4}-[0-9]{1,2}-[0-9]{1,2}/,
				'Date of birth must be in the format of "YYYY-MM-DD"',
			),
		missing_since: z
			.string()
			.regex(
				/^[0-9]{4}-[0-9]{1,2}-[0-9]{1,2}/,
				'Missing since must be in the format of "YYYY-MM-DD"',
			),
		internal_id: z.string().optional(),
		namus_id: z
			.string()
			.regex(/^mp[0-9]+$/i, 'NamUs ID must be in the format of "MP12345"')
			.toLowerCase()
			.optional(),
		synopsis: z.string(),
	});

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

	// const checkNamus = (e: Event) => {
	// 	if (!(e.target instanceof HTMLFormElement)) throw new Error('Not called on HTMLFormElement');
	// 	const namusId = e.target.form.namusImportId.value;
	// 	if (namusId.match(/^mp[0-9]+$/i)) {
	// 		e.target.form.namusImportId.setCustomValidity('');
	// 	} else {
	// 		e.target.form.namusImportId.setCustomValidity('NamUs ID must be in the format of "MP12345"');
	// 	}
	// };

	// //https://www.namus.gov/api/CaseSets/NamUs/MissingPersons/Cases/98673
	// //TODO: CORS seems to prevent this from working - may need to make this call server-side
	// const namusImport = async (e: Event) => {
	// 	if (!(e.target instanceof HTMLFormElement)) throw new Error('Not called on HTMLFormElement');
	// 	console.log(e.target);
	// 	const namusId = e.target.namusImportId.value;
	// 	const res = await fetch(
	// 		`https://www.namus.gov/api/CaseSets/NamUs/MissingPersons/Cases/${namusId}`,
	// 	);
	// 	if (!res.ok) {
	// 		console.log(res);
	// 	}
	// 	console.log(res.json());
	// };

	const addQuestion = (e: Event) => {
		if (!(e.target instanceof HTMLFormElement)) throw new Error('Not called on HTMLFormElement');
		const newQuestion: CreateQuestionDetails = {
			pretty_id: (newInvestigation.questions.length + 1).toString(),
			summary: e.target.addQuestion.value,
			details: '',
			investigation: '',
			status: '',
			outcome: '',
			action_items: [],
		};

		// we can't just push to the array, because Svelte won't "see" the change
		// the compiler only looks at variable assignment, not mutation
		// so to trigger a re-render, we have to create a new array and assign it to the variable
		newInvestigation.questions = [...newInvestigation.questions, newQuestion];
	};
</script>

<!--
<Button class="mt-2 float-right" color="green">
	NamUs Import <Fa class="inline-block ml-2" icon={faPlus} />
</Button>
<Dropdown class="m-4">
	<Label class="" for="namusImportId">NamUs ID</Label>
	<form on:input={checkNamus} on:submit|preventDefault={namusImport}>
		<Input class="mb-4" name="namusImportId" placeholder="MPxxxxx" />
		<Button color="green" type="submit">Import</Button>
	</form>
</Dropdown>
-->

<Heading tag="h1" class="mt-4 mb-8">Create Investigation</Heading>

<form
	action="/api/admin/investigations/create"
	method="POST"
	on:input={checkInput}
	on:submit|preventDefault={submitCreateInvestigationForm}
>
	<Heading tag="h3" class="mt-4 mb-4">Personal Information</Heading>
	<div class="flex mb-4">
		<div class="mr-4">
			<Label for="first_name">First Name</Label>
			<Input id="first_name" name="first_name" bind:value={newInvestigation.first_name} />
		</div>

		<div class="mr-4">
			<Label for="middle_name">Middle Name (optional)</Label>
			<Input id="middle_name" name="middle_name" bind:value={newInvestigation.middle_name} />
		</div>

		<div class="">
			<Label for="last_name">Last Name</Label>
			<Input id="last_name" name="last_name" bind:value={newInvestigation.last_name} />
		</div>
	</div>

	<div class="flex mb-8">
		<div class="">
			<Label for="date_of_birth">Date of Birth (YYYY-MM-DD)</Label>
			<Input id="date_of_birth" name="date_of_birth" placeholder="YYYY-MM-DD" bind:value={newInvestigation.date_of_birth} />
		</div>
	</div>

	<Heading tag="h3" class="mb-4">Case Information</Heading>
	<div class="flex mb-4">
		<div class="mr-4">
			<Label for="missing_since">Missing Since (YYYY-MM-DD)</Label>
			<Input id="missing_since" name="missing_since" placeholder="YYYY-MM-DD" bind:value={newInvestigation.missing_since} />
		</div>
		<div class="mr-4">
			<Label for="internal_id">Internal ID (optional)</Label>
			<Input id="internal_id" name="internal_id" bind:value={newInvestigation.internal_id} />
		</div>
		<div class="">
			<Label for="namus_id">NamUs ID (optional - MPxxxxx)</Label>
			<Input id="namus_id" name="namus_id" placeholder="MPxxxxx" bind:value={newInvestigation.namus_id} />
		</div>
	</div>

	<div class="flex mb-8">
		<div class="w-full">
			<Label for="synopsis">Synopsis</Label>
			<Textarea id="synopsis" name="synopsis" rows="8" class="w-full" bind:value={newInvestigation.synopsis} />
		</div>
	</div>

	<Button class="float-right" color="green" on:click={lol}>
		Add Question <Fa class="inline-block ml-2" icon={faPlus} />
	</Button>
	<Dropdown class="m-4">
		<Label class="" for="addQuestion">Question</Label>
		<form on:submit|preventDefault={addQuestion}>
			<Input id="addQuestion" class="mb-4" name="addQuestion" placeholder="Question" />
			<Button type="submit" color="green">Add</Button>
		</form>
	</Dropdown>

	<Heading tag="h3" class="mb-4">Initial Questions</Heading>
	<div class="mb-8">
		{#each newInvestigation.questions as _question, i}
			<QuestionItem bind:question={newInvestigation.questions[i]} />
		{/each}
	</div>

	<Button type="submit" class="mb-4" color="blue">Submit</Button>
</form>
