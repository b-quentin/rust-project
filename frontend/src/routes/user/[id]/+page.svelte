<script lang="ts">
	import { onMount } from 'svelte';

	// Définition du type des données utilisateur
	export let data: {
		user: {
			id: string;
			username: string;
			email: string;
		} | null;
	};

	onMount(() => {
		console.log('Données reçues dans onMount:', data);
	});
</script>

<h1>Détails de l'Utilisateur</h1>

{#if data.user}
	<div class="user-details">
		<p><strong>ID:</strong> {data.user.id}</p>
		<p><strong>Nom d'utilisateur:</strong> {data.user.username}</p>
		<p><strong>Email:</strong> {data.user.email}</p>
	</div>
{:else}
	<p>Chargement...</p>
{/if}

<style>
	.user-details {
		padding: 1rem;
		border: 1px solid #ddd;
		border-radius: 4px;
	}
</style>
