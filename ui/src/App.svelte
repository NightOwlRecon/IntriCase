<script lang="ts">
    import {
        Button,
        Input,
        Label,
    } from 'flowbite-svelte';
</script>

<main>
    <form action="/login">
			<div class="mb-6">
        <Label for="email">Email</Label>
        <Input placeholder="email" name="email"></Input>
			</div>
			<div class="mb-6">
        <Label for="password">Password</Label>
        <Input placeholder="password" name="password" type="password"></Input>
			</div>
			<div class="mb-6">
        <Button type="submit" color="blue">Log In</Button>
			</div>
    </form>
</main>

<style>
</style>
