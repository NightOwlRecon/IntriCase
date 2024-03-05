<script lang="ts">
	import { UTCDate } from '@date-fns/utc';
	import { format } from 'date-fns';
	import { Heading } from 'flowbite-svelte';
	import { nc } from '../helpers';

	import type { Investigation } from '../typedefs';

	export let params = {};
	const investigationId = params.investigationId;

	let investigation: undefined | Investigation;

	const getInvestigation = async () => {
		const res = await fetch(`/api/investigations/${investigationId}`);
		if (res.ok) {
			investigation = await res.json();
		}
	};

	getInvestigation();
</script>

{#if investigation}
	<Heading tag="h2">
		{investigation.last_name}, {investigation.first_name}
		{nc(investigation.middle_name)}
	</Heading>
	<Heading tag="h6">
		Missing since {format(new UTCDate(investigation.missing_since), 'PPPP')}
	</Heading>
{/if}
