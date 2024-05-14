<script lang="ts">
	import { Badge, Button, Input, Label, ListgroupItem, Textarea } from 'flowbite-svelte';
	import type { CreateActionItemDetails } from '../bindings/CreateActionItemDetails';

	export let editing: boolean = false;

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
		___.{actionItem.pretty_id}.
		<Label for="summary">Summary</Label>
		<Input name="summary" bind:value={actionItem.summary} />
		<Label for="details">Details</Label>
		<Textarea name="details" bind:value={actionItem.details} />
		<Button color="blue">Save</Button>
	</ListgroupItem>
{/if}
