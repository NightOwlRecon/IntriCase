<script lang="ts">
	import {
		AccordionItem,
		Badge,
		Heading,
		Listgroup,
		ListgroupItem,
		Progressbar,
	} from 'flowbite-svelte';

	import type { Question } from '../typedefs';

	export let question: Question;

	$: progress =
		(question.action_items.filter((item) => item.status === 'completed').length /
			question.action_items.length) *
		100;

	const getColor = (status: string) => {
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
	{#if question.action_items.length > 0}
		<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
		<Listgroup>
			{#each question.action_items as action_item}
				<ListgroupItem>
					{question.pretty_id}.{action_item.pretty_id}. {action_item.summary}
					<Badge class="float-right" color={getColor(action_item.status)}>
						{getStatusText(action_item.status)}
					</Badge>
				</ListgroupItem>
			{/each}
		</Listgroup>
	{/if}
</AccordionItem>
