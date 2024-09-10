<script lang="ts">
	import { onMount } from 'svelte';

	// Définition du type des données utilisateurs
	export let data: {
		users: {
			id: string;
			username: string;
			email: string;
		}[];
	};

	onMount(() => {
		console.log('Données reçues dans onMount:', data);
	});
</script>

<h1>Liste des Utilisateurs</h1>

{#if data.users && data.users.length > 0}
	<ul class="user-list">
		{#each data.users as user}
			<li class="user-details">
				<p><strong>ID:</strong> {user.id}</p>
				<p><strong>Nom d'utilisateur:</strong> {user.username}</p>
				<p><strong>Email:</strong> {user.email}</p>
			</li>
		{/each}
	</ul>
{:else}
	<p>Chargement...</p>
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
