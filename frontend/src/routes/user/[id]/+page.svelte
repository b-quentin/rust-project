<script lang="ts">
	import { onMount } from 'svelte';

	// Definition of the user data type
	export let data: {
		user: {
			id: string;
			username: string;
			email: string;
		} | null;
	};

	onMount(() => {
		console.log('Data received in onMount:', data);
	});
</script>

<h1>User Details</h1>

{#if data.user}
	<div class="user-details">
		<p><strong>ID:</strong> {data.user.id}</p>
		<p><strong>Username:</strong> {data.user.username}</p>
		<p><strong>Email:</strong> {data.user.email}</p>
	</div>
{:else}
	<p>Loading...</p>
{/if}

<style>
	.user-details {
		padding: 1rem;
		border: 1px solid #ddd;
		border-radius: 4px;
	}
</style>
