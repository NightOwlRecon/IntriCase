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
	
	import Question from './Question.svelte';

</script>

<Heading class="mb-4 mt-4" tag="h3">Questions</Heading>

<Accordion>
	<Question questionId="1" question="Where was John working at the time of his disappearance?" />
	<AccordionItem>
		<div slot="header">2. Is there any evidence that John made it to Memphis?</div>
	</AccordionItem>
	<AccordionItem>
		<div slot="header">3. Can we fill in further details to John's profile?</div>
	</AccordionItem>
</Accordion>
