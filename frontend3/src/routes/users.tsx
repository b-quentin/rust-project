import { Component, createEffect, createSignal, onMount } from "solid-js";
import client from "../lib/graphqlClient";
import { gql } from "@solid-primitives/graphql";

interface User {
  id: string;
  username: string;
  email: string;
}

interface UsersResponse {
  users: User[];
}

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
  const [users, setUsers] = createSignal<User[]>([]);

  onMount(() => {
    const [data, { refetch, mutate }] = client<UsersResponse>(GetUsersQuery);

    createEffect(() => {
      const response = data();

      if (response) {
        console.log("Data received:", response.users);
        setUsers(response.users);
      } else {
        console.error("Unexpected response structure:", response);
      }
    });
  });

  return (
    <div>
      <h1>Liste des Utilisateurs</h1>
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
}; // Correction: ajout de l'accolade fermante et du point-virgule ici

export default UsersPage;

