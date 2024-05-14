<script lang="ts">
	import Router, { type RouteLoadedEvent, type RouteLoadingEvent } from 'svelte-spa-router';

	import NavBar from './NavBar.svelte';
	import Home from './Home.svelte';

	import InvestigationList from './investigations/InvestigationList.svelte';
	import Investigation from './investigations/Investigation.svelte';
	import UserActivate from './users/Activate.svelte';

	import AdminUsers from './admin/Users.svelte';
	import AdminInvestigationList from './admin/investigations/InvestigationList.svelte';
	import AdminCreateInvestigation from './admin/investigations/CreateInvestigation.svelte';

	const routeLoaded = (e: RouteLoadedEvent) => {
		// restore scroll state ?
		// store in "recent" list locally
		console.log(e.detail);
		console.log(window.location);
	};

	const routeLoading = (e: RouteLoadingEvent) => {
		// store scroll state ?
		console.log(e.detail);
		console.log(window.location);
	};

	const routes = {
		'/': Home,
		'/investigations': InvestigationList,
		'/investigations/:investigationId': Investigation,
		'/activateAccount/:userId/:otp': UserActivate,
		'/admin/users': AdminUsers,
		'/admin/investigations': AdminInvestigationList,
		'/admin/investigations/create': AdminCreateInvestigation,
	};
</script>

<main>
	<NavBar />
	<div class="ml-96 mr-96">
		<Router {routes} on:routeLoaded={routeLoaded} on:routeLoading={routeLoading} />
	</div>
</main>

<style>
</style>
