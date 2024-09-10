import { writable, type Writable } from 'svelte/store';
import { client } from '../apollo';
import { gql } from '@apollo/client';

// Définition des types pour les données utilisateur
interface User {
  id: string;
  username: string;
  email: string;
}

// Définition des types pour la réponse de la requête GraphQL
interface GetUserResponse {
  user: User;
}

// Créer un store Svelte pour les données de l'utilisateur
export const userResult: Writable<User | null> = writable(null);

// Fonction pour exécuter la requête GetUser avec un ID donné
export const fetchUser = async (id: string): Promise<void> => {
  try {
    const { data } = await client.query<GetUserResponse>({
      query: gql`
				query GetUser($id: UUID!) {
					user(id: $id) {
						id
						username
						email
					}
				}
			`,
      variables: { id } // Passer l'ID en tant que variable à la requête
    });
    userResult.set(data.user); // Met à jour le store avec les résultats
  } catch (error) {
    console.error('Erreur lors de la requête GraphQL:', error);
  }
};
