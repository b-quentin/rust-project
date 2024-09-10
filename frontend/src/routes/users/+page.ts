import type { PageLoad } from './$types';
import { createApolloClient } from '$lib/apollo';
import { gql } from '@apollo/client';

// Définition de l'interface utilisateur
interface User {
  id: string;
  username: string;
  email: string;
}

// Définition de la requête GraphQL pour récupérer tous les utilisateurs
const GET_USERS = gql`
	query GetUsers {
		users {
			id
			username
			email
		}
	}
`;

// Fonction load pour récupérer les données des utilisateurs
export const load: PageLoad = async ({ fetch }) => {
  const client = createApolloClient(fetch);

  try {
    const { data } = await client.query<{ users: User[] }>({
      query: GET_USERS
    });

    console.log('Données récupérées dans +page.ts:', data.users);

    // Retourner les données des utilisateurs
    return {
      users: data.users // Transmettre les données des utilisateurs comme props
    };
  } catch (error) {
    console.error('Erreur lors de la requête GraphQL:', error);
    return {
      status: 500,
      error: new Error('Erreur de chargement des données des utilisateurs.')
    };
  }
};
