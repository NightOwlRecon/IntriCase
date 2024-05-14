<script lang="ts">
	import {
		Button,
		Heading,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
	} from 'flowbite-svelte';

	import { Fa } from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';

	import { nc } from '../../helpers';

	import type { Investigation } from '../../typedefs';

	let investigations: Investigation[] = [];

	const getInvestigations = async () => {
		const response = await fetch('/api/investigations');
		investigations = await response.json();
	};

	getInvestigations();
</script>

<Button class="float-right" color="blue" href="/#/admin/investigations/create">Create <Fa icon={faPlus} class="ml-3" pull="right" /></Button>
<Heading class="mb-6">Manage Investigations</Heading>

<div class="clear-both"></div>

<Table striped={false} hoverable={true}>
	<TableHead>
		<TableHeadCell></TableHeadCell>
		<TableHeadCell>Name</TableHeadCell>
		<TableHeadCell>Created</TableHeadCell>
		<TableHeadCell>Actions</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each investigations as investigation}
			<TableBodyRow>
				<TableBodyCell></TableBodyCell>
				<TableBodyCell>
					<a href="/#/investigations/{investigation.id}">
						{investigation.last_name}, {investigation.first_name}
						{nc(investigation.middle_name)}
					</a>
				</TableBodyCell>
				<TableBodyCell>{investigation.created}</TableBodyCell>
				<TableBodyCell></TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
