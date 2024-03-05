<script lang="ts">
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
	} from 'flowbite-svelte';

	import type { Investigation } from '../typedefs';

	import { nc } from '../helpers';

	let investigations: Investigation[] = [];

	const getInvestigations = async () => {
		const response = await fetch('/api/investigations');
		investigations = await response.json();
	};

	getInvestigations();
</script>

<Table striped={false} hoverable={true}>
	<TableHead>
		<TableHeadCell>Name</TableHeadCell>
		<TableHeadCell>Missing Since</TableHeadCell>
		<TableHeadCell>Synopsis</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each investigations as investigation}
			<TableBodyRow
				class="cursor-pointer"
				on:click={() => (document.location = `/#/investigations/${investigation.id}`)}
			>
				<TableBodyCell>
					{investigation.last_name}, {investigation.first_name}
					{nc(investigation.middle_name)}
				</TableBodyCell>
				<TableBodyCell>{investigation.missing_since}</TableBodyCell>
				<TableBodyCell>{investigation.synopsis}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
