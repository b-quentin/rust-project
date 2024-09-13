import { gql } from '@apollo/client';
import client from '../lib/graphql/client';

const GET_USERS = gql`
  query GetUsers {
    users {
      id
      username
      email
    }
  }
`;

export async function getStaticProps() {
  const { data } = await client.query({
    query: GET_USERS,
  });

  return {
    props: {
      users: data.users,
    },
    revalidate: 10, // Revalidation toutes les 10 secondes
  };
}

const SSGUsers = ({ users }) => {
  return (
    <div>
      <h1>SSG Users</h1>
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

export default SSGUsers;
