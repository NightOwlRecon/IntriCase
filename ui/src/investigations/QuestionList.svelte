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

	import QuestionItem from './Question.svelte';

	import type { Question } from '../typedefs';

	let question: Question = {
		id: '',
		pretty_id: '1',
		summary: 'Where was John working at the time of his disappearance?',
		details:
			'At the time of his disappearance, John was current on his rent and bills, but we do not have details of his employment. Contact individuals and search social media to attempt to find details.',
		investigation: '',
		creator: '',
		status: '',
		created: '',
		action_items: [
			{
				id: '',
				pretty_id: 'a',
				summary: "Check John's social media posts for mentions",
				status: 'completed',
				created: '',
				creator: '',
				question: '',
			},
			{
				id: '',
				pretty_id: 'b',
				summary: 'Check with previous employers',
				status: 'unassigned',
				created: '',
				creator: '',
				question: '',
			},
			{
				id: '',
				pretty_id: 'c',
				summary: 'Check with known associates',
				status: 'in_progress',
				created: '',
				creator: '',
				question: '',
			},
			{
				id: '',
				pretty_id: 'd',
				summary: 'Check with financial institutions',
				status: 'not_started',
				assignee: '',
				created: '',
				creator: '',
				question: '',
			},
		],
	};
</script>

<Heading class="mb-4 mt-4" tag="h3">Questions</Heading>

<Accordion>
	<QuestionItem {question} />
	<AccordionItem>
		<div slot="header">2. Is there any evidence that John made it to Memphis?</div>
	</AccordionItem>
	<AccordionItem>
		<div slot="header">3. Can we fill in further details to John's profile?</div>
	</AccordionItem>
</Accordion>
