import { Component, createSignal, onMount } from "solid-js";
import client from "../lib/graphqlClient";
import { gql } from "graphql-request";

// Define the User interface
interface User {
  id: string;
  username: string;
  email: string;
}

// Define the expected response structure
interface UsersResponse {
  users: User[];
}

// Define the GraphQL query
const GetUsersQuery = gql`
  query GetUsers {
    users {
      id
      username
      email
    }
  }
`;

const UsersPage: Component = () => {
  // Initialize the users state
  const [users, setUsers] = createSignal<User[]>([]);

  // Fetch users data on component mount
  onMount(async () => {
    try {
      // Fetch data using the GraphQL client
      const data = await client.request<UsersResponse>(GetUsersQuery);
      console.log("Data received:", data.users);
      setUsers(data.users);
    } catch (error) {
      console.error("Error fetching users:", error);
    }
  });

  return (
    <div>
      <h1>Liste des Utilisateurs</h1>
      {/* Render users list or fallback message */}
      {users().length > 0 ? (
        <ul>
          {users().map((user) => (
            <li key={user.id}>
              {user.username} - {user.email}
            </li>
          ))}
        </ul>
      ) : (
        <p>Aucun utilisateur trouvé.</p>
      )}
    </div>
  );
};

export default UsersPage;

