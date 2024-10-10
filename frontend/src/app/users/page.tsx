"use client";

import { useQuery, gql } from "@apollo/client";
import client from "../../lib/graphql/client";
import { ColumnDef } from "@tanstack/react-table";
import { MoreHorizontal } from "lucide-react";

import { DataTable } from "@/components/ui/data-table"; // Shadcn Data Table component
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

const GET_USERS = gql`
  query GetUsers {
    users {
      id
      username
      email
      firstname
      lastname
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

// Define a named function for the action column
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

// Define the table columns
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
    cell: ActionCell, // Use the named function here
  },
];

export default function UsersPage() {
  const { loading, error, data } = useQuery(GET_USERS, {
    client,
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div className="p-6">
      <h1 className="text-3xl font-semibold mb-6">Users List</h1>
      <Separator className="mb-6" />

      {/* Data Table with users */}
      <DataTable columns={columns} data={data.users} />
    </div>
  );
}