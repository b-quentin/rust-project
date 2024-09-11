// components/Users.tsx
import { For, Show } from "solid-js";
import { useUsers } from "../hooks/queryUsers";

const Users = () => {
  const usersData = useUsers();

  return (
    <div>
      <h1>Liste des Utilisateurs</h1>
      <Show when={usersData()?.users} fallback={<p>Chargement...</p>}>
        <ul>
          <For each={usersData()?.users}>
            {(user) => (
              <li key={user.id}>
                {user.username} - {user.email}
              </li>
            )}
          </For>
        </ul>
      </Show>
    </div>
  );
};

export default Users;
