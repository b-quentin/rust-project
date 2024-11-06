"use client";

import { useQuery, gql } from "@apollo/client";
import client from "@/lib/graphql/client";
import { ColumnDef } from "@tanstack/react-table";
import { MoreHorizontal } from "lucide-react";
import { useRouter } from 'next/navigation';

import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Separator } from "@/components/ui/separator";
import { useEffect, useState } from "react";
import { getCookie } from "@/lib/auth/cookies";

const GET_USERS = gql`
  query users($token: String!) {
    admin {
      users(token: $token) {
        id
        username
        email
        firstName
        lastName
      }
    }
  }
`;

type User = {
  id: string;
  username: string;
  email: string;
  firstname: string;
  lastname: string;
};

function ActionCell({ row }: { row: any }) {
  const user = row.original;

  return (
    <div className="flex justify-end">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" className="h-8 w-8 p-0">
            <span className="sr-only">Open menu</span>
            <MoreHorizontal className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuItem
            onClick={() => navigator.clipboard.writeText(user.id)}
          >
            Copy user ID
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem>View Profile</DropdownMenuItem>
          <DropdownMenuItem>Delete User</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}

const columns: ColumnDef<User>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "username",
    header: "Username",
  },
  {
    accessorKey: "firstname",
    header: "First Name",
  },
  {
    accessorKey: "lastname",
    header: "Last Name",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    id: "actions",
    header: "",
    cell: ActionCell,
  },
];

export default function AdminUsersPage() {
  const router = useRouter();
  const [token, setToken] = useState<string | null>(null);

  useEffect(() => {
    const token = getCookie("auth_token");
    if (!token) {
      router.push('/login'); // Redirige vers la page de connexion si le token n'est pas présent
    } else {
      setToken(token);
    }
  }, [router]);

  const { loading, error, data } = useQuery(GET_USERS, {
    client,
    variables: { token },
    skip: !token, // Ne pas exécuter la requête tant que le token n'est pas défini
  });

  const users = data?.admin?.users || [];

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div className="p-6">
      <h1 className="text-3xl font-semibold mb-6">Admin Users List</h1>
      <Separator className="mb-6" />

      <DataTable columns={columns} data={users} />
    </div>
  );
} 