import type { PageLoad } from './$types';
import { createApolloClient } from '$lib/apollo';
import { gql } from '@apollo/client';

// Définition de l'interface utilisateur
interface User {
  id: string;
  username: string;
  email: string;
}

// Définition de la requête GraphQL
const GET_USER = gql`
	query GetUser($id: UUID!) {
		user(id: $id) {
			id
			username
			email
		}
	}
`;

// Fonction load pour récupérer les données utilisateur
export const load: PageLoad = async ({ params, fetch }) => {
  const { id } = params;
  const client = createApolloClient(fetch);

  try {
    const { data } = await client.query<{ user: User | null }>({
      query: GET_USER,
      variables: { id }
    });

    console.log('Données récupérées dans +page.ts:', data.user);

    // Retourner les données utilisateur
    return {
      user: data.user // Transmettre les données utilisateur comme props
    };
  } catch (error) {
    console.error('Erreur lors de la requête GraphQL:', error);
    return {
      status: 500,
      error: new Error('Erreur de chargement des données utilisateur.')
    };
  }
};
