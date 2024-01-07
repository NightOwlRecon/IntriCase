<script lang="ts">
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
		NavBrand,
		NavHamburger,
		NavLi,
		NavUl,
		Navbar,
		TabItem,
		Tabs,
	} from 'flowbite-svelte';

	import { Fa } from 'svelte-fa';
	import { faCaretDown } from '@fortawesome/free-solid-svg-icons';

	import Users from './admin/Users.svelte';

	import { cookieValue, handleSubmitJson } from './helpers';

	// if the session cookie exists we can assume that it wasn't nuked by the session layer
	// and that the user is logged in
	//let loggedIn = !!cookieValue('session', true);
	let loggedIn = true;
	let userDetails = cookieValue('user_details') ? JSON.parse(cookieValue('user_details')) : {};

	const handleLoginForm = async (e: Event) => {
		const res = await handleSubmitJson(e);
		if (res && res.ok) {
			loggedIn = true;
			userDetails = JSON.parse(cookieValue('user_details'));
		}
		loggedIn = false;
		userDetails = {};
	};

	// this is only used to bind the login form to so that its contents are not
	// reset if the form is closed and re-opened - likely unnecessary most other places
	// TODO: find a better solution
	const loginData = {
		email: '',
		password: '',
	};
</script>

<main>
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
					<form action="/auth/login" method="POST" on:submit|preventDefault={handleLoginForm}>
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

	<Tabs>
		<TabItem open title="Users">
			<Users />
		</TabItem>
	</Tabs>
</main>

<style>
</style>
