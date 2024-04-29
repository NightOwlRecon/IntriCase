<script lang="ts">
	import {
		AccordionItem,
		Badge,
		Button,
		Heading,
		Input,
		Label,
		Listgroup,
		ListgroupItem,
		Progressbar,
		Textarea,
	} from 'flowbite-svelte';

	import Fa from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';

	import type { Question } from '../bindings/Question';
	import type { ActionItem } from '../bindings/ActionItem';

	export let question: Question;

	export let editControls: boolean = false;
	export let editing: boolean = true;

	let newActionItem: ActionItem | undefined;

	$: action_items = Object.values(question.action_items);
	$: progress =
		(action_items.filter((item) => item.status === 'completed').length / action_items.length) * 100;

	const getStatusColor = (status: string) => {
		switch (status) {
			case 'completed':
				return 'green';
			case 'in_progress':
				return 'blue';
			case 'unassigned':
				return 'red';
			case 'not_started':
				return 'dark';
		}
	};

	const getStatusText = (status: string) => {
		switch (status) {
			case 'completed':
				return 'Completed';
			case 'in_progress':
				return 'In Progress';
			case 'unassigned':
				return 'Unassigned';
			case 'not_started':
				return 'Not Started';
		}
	};
</script>

{#if !editing}
	<AccordionItem>
		{#if editControls}
			<span>Edit Controls</span>
		{/if}
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

		{#if action_items.length > 0}
			<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
			<Listgroup>
				{#each action_items as action_item}
					<ListgroupItem>
						{question.pretty_id}.{action_item.pretty_id}. {action_item.summary}
						<Badge class="float-right" color={getStatusColor(action_item.status)}>
							{getStatusText(action_item.status)}
						</Badge>
					</ListgroupItem>
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
				<Input name="summary" value={question.summary} />
			</div>
		</div>
		<Heading tag="h6" class="mt-4 mb-4">Details</Heading>
		<Label for="details">Details</Label>
		<Textarea name="details" rows="8" class="w-full" value={question.details} />
		<Button class="float-right" color="green">
			Add <Fa class="inline-block ml-2" icon={faPlus} />
		</Button>
		<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
		<Listgroup>
			{#each action_items as action_item}
				<ListgroupItem>
					{question.pretty_id}.{action_item.pretty_id}. {action_item.summary}
					<Badge class="float-right" color={getStatusColor(action_item.status)}>
						{getStatusText(action_item.status)}
					</Badge>
				</ListgroupItem>
			{/each}
			{#if newActionItem}
				<ListgroupItem>
					{question.pretty_id}.{newActionItem.pretty_id}.
					<Label for="summary">Summary</Label>
					<Input name="summary" value={newActionItem.summary} />
					<Label for="details">Details</Label>
					<Textarea name="details" value={newActionItem.details} />
					<Button color="blue">Save</Button>
				</ListgroupItem>
			{/if}
		</Listgroup>
	</AccordionItem>
{/if}
