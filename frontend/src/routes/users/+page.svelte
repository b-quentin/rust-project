<script lang="ts">
	import { onMount } from 'svelte';

	// Definition of the user data type
	export let data: {
		users: {
			id: string;
			username: string;
			email: string;
		}[];
	};

	onMount(() => {
		console.log('Data received in onMount:', data);
	});
</script>

<h1>User List</h1>

{#if data.users && data.users.length > 0}
	<ul class="user-list">
		{#each data.users as user}
			<li class="user-details">
				<p><strong>ID:</strong> {user.id}</p>
				<p><strong>Username:</strong> {user.username}</p>
				<p><strong>Email:</strong> {user.email}</p>
			</li>
		{/each}
	</ul>
{:else}
	<p>Loading...</p>
{/if}

<style>
	.user-list {
		list-style: none;
		padding: 0;
	}

	.user-details {
		padding: 1rem;
		border: 1px solid #ddd;
		border-radius: 4px;
		margin-bottom: 1rem;
	}
</style>
