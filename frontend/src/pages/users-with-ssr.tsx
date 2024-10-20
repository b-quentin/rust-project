import { gql } from "@apollo/client";
import client from "../lib/graphql/client";

const GET_USERS = gql`
  query GetUsers {
    users {
      id
      username
      email
    }
  }
`;

export async function getServerSideProps() {
  const { data } = await client.query({
    query: GET_USERS,
  });

  return {
    props: {
      users: data.users,
    },
  };
}

interface User {
  id: string;
  username: string;
  email: string;
}

interface UsersProps {
  users: User[];
}

const SSRUsers = ({ users }: UsersProps) => {
  return (
    <div>
      <h1>SSR Users</h1>
      <ul>
        {users.map((user) => (
          <li key={user.id}>
            {user.username} - {user.email}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default SSRUsers;
