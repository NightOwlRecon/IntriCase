<script lang="ts">
	import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
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
	import { cookieValue, handleSubmitJson } from './helpers.js';

	import { userDetails } from './stores';

	// this is only used to bind the login form to so that its contents are not
	// reset if the form is closed and re-opened - likely unnecessary most other places
	const loginData = {
		email: '',
		password: '',
	};

	const getUserDetailsFromCookie = () => {
		const cookie = cookieValue('user_details');
		if (!cookie) return;
		userDetails.set(JSON.parse(cookie));
	};

	const submitLoginForm = async (e: Event) => {
		const res = await handleSubmitJson(e);
		if (res.ok) {
			getUserDetailsFromCookie();
		}
	};

	getUserDetailsFromCookie();
</script>

<Navbar class="p-0 bg-slate-50 mb-6">
	<NavBrand href="/">
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
			IntriCase
		</span>
	</NavBrand>
	<NavHamburger />
	<NavUl>
		{#if $userDetails}
			<NavLi class="m-2" href="/#/investigations">Investigations</NavLi>
			<NavLi class="m-2 cursor-pointer">
				Admin <Fa class="inline-block ml-1" icon={faCaretDown} />
			</NavLi>
			<Dropdown class="m-4">
				<DropdownItem href="/#/admin/investigations">Investigations</DropdownItem>
				<DropdownItem href="/#/admin/users">Users</DropdownItem>
			</Dropdown>
			<NavLi class="cursor-pointer">
				<div>
					<Avatar class="float-left" />
					<Fa class="align-middle inline-block ml-2 mt-3" icon={faCaretDown} />
				</div>
				<Dropdown class="m-4">
					<DropdownHeader>
						{$userDetails.display_name}
					</DropdownHeader>
					<DropdownItem>Profile</DropdownItem>
					<DropdownItem>Admin</DropdownItem>
					<DropdownDivider />
					<DropdownItem href="/api/auth/logout">Log Out</DropdownItem>
				</Dropdown>
			</NavLi>
		{:else}
			<NavLi class="m-2" href="#">
				Log In <Fa icon={faCaretDown} class="ml-3 mt-1 inline-block" />
			</NavLi>
			<Dropdown class="m-4">
				<!-- form defaults to multipart unless enctype is specified, making deserialization more painful on the backend -->
				<form action="/api/auth/login" method="POST" on:submit|preventDefault={submitLoginForm}>
					<div class="mb-6">
						<Label for="email">Email</Label>
						<Input placeholder="email" name="email" bind:value={loginData.email} />
					</div>
					<div class="mb-6">
						<Label for="password">Password</Label>
						<Input
							placeholder="password"
							name="password"
							type="password"
							bind:value={loginData.password}
						/>
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
