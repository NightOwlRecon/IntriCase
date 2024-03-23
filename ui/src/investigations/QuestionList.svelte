<script lang="ts">
	import { faDivide } from '@fortawesome/free-solid-svg-icons';
	import {
		Accordion,
		AccordionItem,
		Badge,
		Heading,
		Listgroup,
		ListgroupItem,
		Progressbar,
	} from 'flowbite-svelte';
</script>

<Heading class="mb-4 mt-4" tag="h3">Questions</Heading>
<Accordion>
	<AccordionItem open>
		<div slot="header" class="flex w-full">
			<div class="flex-grow">1. Where was John working at the time?</div>
			<Progressbar
				progress="25"
				class="mr-4 mt-1 w-48 text-white"
				color="blue"
				size="h-4"
				labelInside
			/>
		</div>
		<p>
			At the time of his disappearance, John was current on his rent and bills, but we do not have details of his employment. Contact individuals and search social media to attempt to find details.

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
	<AccordionItem>
		<div slot="header">2. Is there any evidence that John made it to Memphis?</div>
	</AccordionItem>
	<AccordionItem>
		<div slot="header">3. Can we fill in further details to John's profile?</div>
	</AccordionItem>
</Accordion>
