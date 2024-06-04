<script lang="ts">
	import { Badge, Button, Heading, Input, Label, ListgroupItem, Select, Textarea } from 'flowbite-svelte';
	import type { CreateActionItemDetails } from '../bindings/CreateActionItemDetails';

	import { users } from '../stores';

	export let editing: boolean = false;

	let assignee = 'unassigned';

	export let actionItem: CreateActionItemDetails = {
		pretty_id: '',
		summary: '',
		status: 'not_started',
		details: '',
		outcome: '',
		assignee: '',
		resolved: '',
	};

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

	const saveActionItem = () => {
		editing = false;
	}
</script>

{#if !editing}
	<ListgroupItem>
		___.{actionItem.pretty_id}. {actionItem.summary}
		<Badge class="float-right" color={getStatusColor(actionItem.status)}>
			{getStatusText(actionItem.status)}
		</Badge>
	</ListgroupItem>
{:else}
	<ListgroupItem>
		<Heading class="mb-4" tag="h6">___.{actionItem.pretty_id}.</Heading>
		<Label for="summary">Summary</Label>
		<Input class="mb-4" name="summary" bind:value={actionItem.summary} />

		<Label for="details">Details</Label>
		<Textarea class="mb-4" name="details" bind:value={actionItem.details} />

		<!-- We add unassigned to the top of the list of users as a default value -->
		<Label for="assignee">Assignee</Label>
		<Select
			name="assignee"
			class="mb-4"
			items={[{ value: 'unassigned', name: 'Unassigned' }, ...$users]}
			bind:value={assignee}
		/>

		<Button color="blue" on:click={saveActionItem}>Save</Button>
	</ListgroupItem>
{/if}
