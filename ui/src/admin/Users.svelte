<script lang="ts">
	import {
		Button,
		Dropdown,
		Heading,
		Input,
		Label,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
	} from 'flowbite-svelte';

	import { Fa } from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';

	import { handleSubmitJson } from '../helpers';

	let inviteUserEmail = '';

	const handleInviteUserForm = async (e: Event) => {
		const res = await handleSubmitJson(e);
		if (res && res.ok) {
			getUsers();
		}
	};

	let users = [];
	const getUsers = async () => {
		const res = await fetch('/admin/users/list');
		if (res.ok) {
			users = await res.json();
		}
	};

	getUsers();
</script>

<Button color="blue">Invite <Fa icon={faPlus} class="ml-3" pull="right" /></Button>
<Dropdown class="m-4">
	<!-- form defaults to multipart unless enctype is specified, making deserialization more painful on the backend -->
	<form action="/admin/users/invite" method="POST" on:submit|preventDefault={handleInviteUserForm}>
		<div class="mb-6">
			<Label for="email">Email</Label>
			<Input placeholder="email" name="email" bind:value={inviteUserEmail}></Input>
		</div>
		<div>
			<Button type="submit" color="blue">Invite</Button>
		</div>
	</form>
</Dropdown>

<Table>
	<TableHead>
		<TableHeadCell>Name</TableHeadCell>
		<TableHeadCell>Email</TableHeadCell>
		<TableHeadCell>Status</TableHeadCell>
		<TableHeadCell>Last Login</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each users as user}
			<TableBodyRow>
				<TableBodyCell>{user.display_name ? user.display_name : ''}</TableBodyCell>
				<TableBodyCell>{user.email}</TableBodyCell>
				<TableBodyCell>{user.enabled ? 'enabled' : 'disabled'}</TableBodyCell>
				<TableBodyCell>{user.auth_date ? user.auth_date : 'never'}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

<style>
</style>
