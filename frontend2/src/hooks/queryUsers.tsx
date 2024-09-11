import { createGraphQLClient, gql } from "@solid-primitives/graphql";

// Define interfaces for the expected data structure
interface User {
  id: string;
  username: string;
  email: string;
}

interface UsersResponse {
  users: User[];
}

const newQuery = createGraphQLClient("http://127.0.0.1:8080/graphql");

// Use the GraphQL client with the defined interfaces
export function useUsers() {
  const [data] = newQuery<UsersResponse>(
    gql`
      query GetUsers {
        users {
          id
          username
          email
        }
      }
    `,
    {},
  );

  return data;
}
