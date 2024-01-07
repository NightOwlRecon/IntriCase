<script lang="ts">
	import { TabItem, Tabs } from 'flowbite-svelte';

	import NavBar from './NavBar.svelte';
	import UserActivate from './users/Activate.svelte';
	import Users from './admin/Users.svelte';

	import { cookieValue, handleUriPath } from './helpers';

	let uriPath = handleUriPath();
	window.addEventListener('hashchange', function () {
		uriPath = handleUriPath();
	});

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
	{#if uriPath[0] === 'activateAccount'}
		<UserActivate path={uriPath} />
	{:else if uriPath[0] === 'admin'}
		<Tabs>
			<TabItem open title="Users">
				<Users />
			</TabItem>
		</Tabs>
	{/if}
</main>

<style>
</style>
