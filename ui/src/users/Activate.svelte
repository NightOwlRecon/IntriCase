<script lang="ts">
	import { Alert, Button, Input, Label } from 'flowbite-svelte';
	import { handleSubmitJson } from '../helpers';
	import {Fa} from "svelte-fa";
	import {faExclamationTriangle} from "@fortawesome/free-solid-svg-icons";
	const submitActivateForm = async (e: Event) => {
		if (!(e.target instanceof HTMLFormElement)) throw new Error('Not called on HTMLFormElement');
		const res = await handleSubmitJson(e);
	};

	export let path;

	let state: { 
		valid: boolean, 
		submitted: boolean, 
		submitError: boolean,
		invalidReason: string,
	} = {
		valid: true,
		submitted: false,
		submitError: false,
		invalidReason: "",
	};

	let color: "red" | undefined;
	$: color = state.valid ? undefined : 'red';

	let to: number;
	const remoteCheckPasswordPosture = async (f: FormData) => {
		const res = await fetch('/auth/passwordPosture', {
			headers: new Headers({ 'Content-Type': 'application/json' }),
			method: 'POST',
			body: JSON.stringify(Object.fromEntries(f)),
		});
		state.submitted = true;

		if (res.ok) {
		// we have a result
			const data = await res.json();
			state.valid = data.valid;
			if (state.valid) {
				// result was good
			} else {
				// result was bad
				state.invalidReason = data.reason;
			}
		} else {
			// we have an error
			state.submitError = true;
		}
	};

	const checkPasswordPosture = async (e: Event) => {
		// this gives us the event of the field that was edited rather than the form itself
		if (!(e.target instanceof HTMLInputElement)) throw new Error('Not called on HTMLInputElement');
		if (!(e.target.form instanceof HTMLFormElement))
			throw new Error('Target not inside an HTMLFormElement');
		const formData = new FormData(e.target.form);

		const password = formData.get('password')?.toString();
		const confirm = formData.get('confirm')?.toString();

		// we *could* rely purely on server-side validation here

		// hide errors if the form is empty
		if (!password && !confirm) {
			state.valid = true;
			return;
		}

		if (password) {
			if (password !== confirm) {
				state.valid = false;
				state.invalidReason = 'Passwords do not match.';
				return;
			}

			if (password.length < 8) {
				state.valid = false;
				state.invalidReason = 'Passwords must be at least eight (8) characters in length.';
				return;
			}

			if (password.length > 512) {
				state.valid = false;
				state.invalidReason = 'Passwords must be 512 or fewer characters in length.';
				return;
			}

			state.valid = true;

			// do remote check
			if (to) clearTimeout(to);
			to = setTimeout(remoteCheckPasswordPosture, 500, formData);
		}
	};
</script>

<div class="w-96 mt-64 ml-auto mr-auto">
	{#if !state.submitted || !state.valid}
	<div id="activateForm">
		<form
			action="/auth/activate"
			method="POST"
			on:input={checkPasswordPosture}
			on:submit|preventDefault={submitActivateForm}
		>
			<div class="mb-6">
				<Label for="name">Name</Label>
				<Input name="display_name" type="text" placeholder="name" />
			</div>
			<div class="mb-6">
				<Label for="password" {color}>Password</Label>
				<Input name="password" type="password" {color} placeholder="password" />
			</div>
			<div class="mb-6">
				<Label for="confirm" {color}>Confirm Password</Label>
				<Input name="confirm" type="password" {color} placeholder="confirm" />
			</div>
			{#if !state.valid}
				<Alert class="mb-6" color="red">
					<p>
						<Fa class="inline" icon={faExclamationTriangle} />
						<span class="font-bold">Error:</span>
						{state.invalidReason}
					</p>
				</Alert>
			{/if}
			<Input name="id" type="hidden" value={path[1]} />
			<Input name="otp" type="hidden" value={path[2]} />
			<Button name="submit" type="submit" class="mb-6" color="blue" disabled={state.valid ? '' : 'disabled'}>Submit</Button>
		</form>
	</div>
	{/if}
	{#if state.submitted}
	{#if !state.submitError && state.valid}

	<div id="activateSuccess">
		<p>
			Your password has been set. You may now <a href="/">log in</a>.
		</p>
	</div>
	{/if}
	{#if state.submitError}
	<div id="activateError">
		<p>There was an error activating your account. Please contact an administrator.</p>
	</div>
	{/if}
	{/if}
</div>
