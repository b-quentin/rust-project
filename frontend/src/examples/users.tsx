"use client";

import { useQuery, gql } from "@apollo/client";
import client from "../lib/graphql/client";
import "@styles"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";


const GET_USERS = gql`
  query GetUsers {
    users {
      id
      username
      email
    }
  }
`;

export default function UsersList() {
  const { loading, error, data } = useQuery(GET_USERS, {
    client,
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  type User = {
    id: string;
    username: string;
    email: string;
  };

  return (
    <div className="p-6">
      <h1 className="text-3xl font-semibold mb-6">Users List</h1>
      <Separator className="mb-6" />
      <div className="space-y-4">
        {data.users.map((user: User) => (
          <Card key={user.id} className="shadow">
            <CardHeader>
              <CardTitle>{user.username}</CardTitle>
            </CardHeader>
            <CardContent>
              <p>Email: {user.email}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
