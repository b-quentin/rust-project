import type { PageLoad } from './$types';
import { createApolloClient } from '$lib/apollo';
import { gql } from '@apollo/client';

// Definition of the User interface
interface User {
  id: string;
  username: string;
  email: string;
}

// Definition of the GraphQL query
const GET_USER = gql`
	query GetUser($id: UUID!) {
		user(id: $id) {
			id
			username
			email
		}
	}
`;

// Load function to fetch user data
export const load: PageLoad = async ({ params, fetch }) => {
  const { id } = params;
  const client = createApolloClient(fetch);

  try {
    const { data } = await client.query<{ user: User | null }>({
      query: GET_USER,
      variables: { id }
    });

    console.log('Data retrieved in +page.ts:', data.user);

    // Return the user data
    return {
      user: data.user // Pass the user data as props
    };
  } catch (error) {
    console.error('Error during GraphQL query:', error);
    return {
      status: 500,
      error: new Error('Error loading user data.')
    };
  }
};
