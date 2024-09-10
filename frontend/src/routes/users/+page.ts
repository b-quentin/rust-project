import type { PageLoad } from './$types';
import { createApolloClient } from '$lib/apollo';
import { gql } from '@apollo/client';

// Definition of the User interface
interface User {
  id: string;
  username: string;
  email: string;
}

// Definition of the GraphQL query to retrieve all users
const GET_USERS = gql`
	query GetUsers {
		users {
			id
			username
			email
		}
	}
`;

// Load function to fetch user data
export const load: PageLoad = async ({ fetch }) => {
  const client = createApolloClient(fetch);

  try {
    const { data } = await client.query<{ users: User[] }>({
      query: GET_USERS
    });

    console.log('Data retrieved in +page.ts:', data.users);

    // Return the user data
    return {
      users: data.users // Pass the user data as props
    };
  } catch (error) {
    console.error('Error during GraphQL query:', error);
    return {
      status: 500,
      error: new Error('Error loading user data.')
    };
  }
};
