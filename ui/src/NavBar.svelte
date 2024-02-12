<script lang="ts">
	import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
	import { createEventDispatcher } from 'svelte';
	import {
		Avatar,
		Button,
		DarkMode,
		Dropdown,
		DropdownDivider,
		DropdownHeader,
		DropdownItem,
		Input,
		Label,
		Navbar,
		NavBrand,
		NavHamburger,
		NavLi,
		NavUl,
	} from 'flowbite-svelte';
	import { Fa } from 'svelte-fa';
	import { handleSubmitJson } from './helpers.js';

	export let userDetails;
	// if we have user details, it's safe to say that we're logged in
	$: loggedIn = !!userDetails;

	const dispatch = createEventDispatcher();

	// this is only used to bind the login form to so that its contents are not
	// reset if the form is closed and re-opened - likely unnecessary most other places
	// TODO: find a better solution
	const loginData = {
		email: '',
		password: '',
	};

	const submitLoginForm = async (e: Event) => {
		const res = await handleSubmitJson(e);
		// tell the parent that the request has completed and we should have new cookies
		dispatch('change');
	};
</script>

<Navbar class="p-0 bg-slate-50">
	<NavBrand href="/">
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
			IntriCase
		</span>
	</NavBrand>
	<NavHamburger />
	<NavUl>
		{#if loggedIn}
			<NavLi class="m-2" href="/contact">Contact</NavLi>
			<NavLi>
				<Avatar />
				<Dropdown class="m-4">
					<DropdownHeader>
						{userDetails.display_name}
					</DropdownHeader>
					<DropdownItem>Profile</DropdownItem>
					<DropdownItem>Admin</DropdownItem>
					<DropdownDivider />
					<DropdownItem href="/auth/logout">Log Out</DropdownItem>
				</Dropdown>
			</NavLi>
		{:else}
			<NavLi class="m-2" href="#">
				Log In <Fa icon={faCaretDown} class="ml-3 mt-1" pull="right" />
			</NavLi>
			<Dropdown class="m-4">
				<!-- form defaults to multipart unless enctype is specified, making deserialization more painful on the backend -->
				<form action="/auth/login" method="POST" on:submit|preventDefault={submitLoginForm}>
					<div class="mb-6">
						<Label for="email">Email</Label>
						<Input placeholder="email" name="email" bind:value={loginData.email}></Input>
					</div>
					<div class="mb-6">
						<Label for="password">Password</Label>
						<Input
							placeholder="password"
							name="password"
							type="password"
							bind:value={loginData.password}
						></Input>
					</div>
					<div>
						<Button type="submit" color="blue">Log In</Button>
					</div>
				</form>
			</Dropdown>
		{/if}
		<NavLi>
			<DarkMode />
		</NavLi>
	</NavUl>
</Navbar>
