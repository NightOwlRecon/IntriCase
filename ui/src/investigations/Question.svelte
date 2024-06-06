<script lang="ts">
	import {
		AccordionItem,
		Button,
		Heading,
		Input,
		Label,
		Listgroup,
		Progressbar,
		Textarea,
	} from 'flowbite-svelte';

	import Fa from 'svelte-fa';
	import { faPencil, faPlus } from '@fortawesome/free-solid-svg-icons';
	import ActionItem from './ActionItem.svelte';

	import type { CreateQuestionDetails } from '../bindings/CreateQuestionDetails';

	export let question: CreateQuestionDetails;

	export let editing: boolean = true;

	const addActionItem = (_e: Event) => {
		question.action_items = [
			...question.action_items,
			{
				pretty_id: `${question.pretty_id}.${question.action_items.length + 1}`,
				summary: '',
				status: 'not_started',
				details: '',
				outcome: '',
				assignee: '',
				resolved: '',
			},
		];
	};

	$: progress =
		(question.action_items.filter((item) => item.status === 'completed').length /
			question.action_items.length) *
		100;

	const saveQuestion = (_e: Event) => {
		editing = false;
	};
</script>

{#if !editing}
	<div class="flex w-full">
		<div class="flex-grow">{question.pretty_id}. {question.summary}</div>
		<Button color="light" on:click={() => (editing = true)}>
			Edit <Fa class="inline-block ml-2" icon={faPencil} />
		</Button>

		{#if progress}
			<Progressbar
				{progress}
				class="mr-4 mt-1 w-48 text-white"
				color="blue"
				size="h-4"
				labelInside
			/>
		{/if}
	</div>

	<p>{question.details}</p>

	{#if question.action_items.length > 0}
		<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
		<Listgroup>
			{#each question.action_items as actionItem}
				<ActionItem bind:actionItem />
			{/each}
		</Listgroup>
	{/if}
{:else if editing}
	<div class="flex w-full">
		<span>{question.pretty_id}.</span>
		<div>
			<Label for="summary">Summary</Label>
			<Input name="summary" bind:value={question.summary} />
		</div>
	</div>
	<Heading tag="h6" class="mt-4 mb-4">Details</Heading>
	<Label for="details">Details</Label>
	<Textarea name="details" rows="8" class="w-full mb-4" bind:value={question.details} />
	<Button class="float-right" color="green" on:click={addActionItem}>
		Add Action Item <Fa class="inline-block ml-2" icon={faPlus} />
	</Button>
	<Heading tag="h4" class="mb-4">Action Items</Heading>
	<Listgroup>
		{#each question.action_items as actionItem}
			<ActionItem {editing} bind:actionItem />
		{/each}
	</Listgroup>
	<Button color="blue" on:click={saveQuestion}>Save Question</Button>
{/if}
