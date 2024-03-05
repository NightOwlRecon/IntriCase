<script lang="ts">
	import Router from 'svelte-spa-router'

	const routes = {
		'/': Fa, // Fa makes a decent placeholder, lol
		'/activateAccount/:userId/:otp': UserActivate,
		'/admin/users': Users,
	};

	import NavBar from './NavBar.svelte';
	import UserActivate from './users/Activate.svelte';
	import Users from './admin/Users.svelte';

	import { cookieValue } from './helpers';
	import { Fa } from 'svelte-fa';

	let userDetails;

	const getUserDetailsFromCookie = () => {
		const cookie = cookieValue('user_details');
		if (!cookie) return;

		userDetails = JSON.parse(cookie);
		console.log(userDetails);
	};

	getUserDetailsFromCookie();
</script>

<main>
	<NavBar {userDetails} on:change={getUserDetailsFromCookie} />
	<Router {routes} />
</main>

<style>
</style>
