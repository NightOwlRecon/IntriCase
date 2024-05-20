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
	import { faPlus } from '@fortawesome/free-solid-svg-icons';
	import ActionItem from './ActionItem.svelte';

	import type { CreateQuestionDetails } from '../bindings/CreateQuestionDetails';
	import type { CreateActionItemDetails } from '../bindings/CreateActionItemDetails';

	export let question: CreateQuestionDetails;

	export let editing: boolean = true;

	let actionItems: CreateActionItemDetails[] = [];

	const addActionItem = (e: Event) => {
		actionItems = [
			...actionItems,
			{
				pretty_id: `${question.pretty_id}.${actionItems.length + 1}`,
				summary: '',
				status: 'not_started',
				details: '',
				outcome: '',
				assignee: '',
				resolved: '',
			},
		];
	};

	$: actionItems = Object.values(question.action_items);
	$: progress =
		(actionItems.filter((item) => item.status === 'completed').length / actionItems.length) * 100;
</script>

{#if !editing}
	<AccordionItem>
		<div slot="header" class="flex w-full">
			<div class="flex-grow">{question.pretty_id}. {question.summary}</div>

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

		{#if actionItems.length > 0}
			<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
			<Listgroup>
				{#each actionItems as actionItem}
					<ActionItem bind:actionItem />
				{/each}
			</Listgroup>
		{/if}
	</AccordionItem>
{:else if editing}
	<AccordionItem open>
		<div slot="header" class="flex w-full">
			<span>{question.pretty_id}.</span>
			<div>
				<Label for="summary">Summary</Label>
				<Input name="summary" bind:value={question.summary} />
			</div>
		</div>
		<Heading tag="h6" class="mt-4 mb-4">Details</Heading>
		<Label for="details">Details</Label>
		<Textarea name="details" rows="8" class="w-full" bind:value={question.details} />
		<Button class="float-right" color="green" on:click={addActionItem}>
			Add <Fa class="inline-block ml-2" icon={faPlus} />
		</Button>
		<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
		<Listgroup>
			{#each actionItems as actionItem}
				<ActionItem editing={true} bind:actionItem />
			{/each}
		</Listgroup>
	</AccordionItem>
{/if}
