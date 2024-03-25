<script lang="ts">
	import {
		Accordion,
		AccordionItem,
		Badge,
		Heading,
		Listgroup,
		ListgroupItem,
		Progressbar,
	} from 'flowbite-svelte';

	export let questionId;
	export let question;
</script>

<AccordionItem open>
	<div slot="header" class="flex w-full">
		<div class="flex-grow">{questionId}. {question}</div>
		<Progressbar
			progress="25"
			class="mr-4 mt-1 w-48 text-white"
			color="blue"
			size="h-4"
			labelInside
		/>
	</div>
	<p>
		At the time of his disappearance, John was current on his rent and bills, but we do not have
		details of his employment. Contact individuals and search social media to attempt to find
		details.
	</p>
	<Heading tag="h4" class="mt-4 mb-4">Action Items</Heading>
	<Listgroup>
		<ListgroupItem>
			1a. Check John's social media posts for mentions
			<Badge class="float-right" color="green">Completed</Badge>
		</ListgroupItem>
		<ListgroupItem>
			1b. Check with previous employers
			<Badge class="float-right" color="red">Unassigned</Badge>
		</ListgroupItem>
		<ListgroupItem>
			1c. Check with known associates
			<Badge class="float-right" color="yellow">In Progress</Badge>
		</ListgroupItem>
		<ListgroupItem>
			1d. Check with financial institutions
			<Badge class="float-right" color="dark">Not Started</Badge>
		</ListgroupItem>
	</Listgroup>
</AccordionItem>
