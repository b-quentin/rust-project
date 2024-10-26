"use client";

import { useEffect, useState, FormEvent } from "react";
import { gql, useMutation } from "@apollo/client";
import client from "@lib/graphql/client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useRouter } from "next/navigation";

interface GenerateTokenResponse {
  admin: {
    generateToken: string;
  };
}

// GraphQL mutation to generate token
const GENERATE_TOKEN = gql`
  mutation GenerateToken($input: GenerateTokenInput!) {
    admin {
      generateToken(input: $input)
    }
  }
`;

export default function LoginPage() {
  const [formData, setFormData] = useState({ email: "", password: "" });
  const [error, setError] = useState<string | null>(null);
  const [redirectTo, setRedirectTo] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    if (typeof window !== "undefined") {
      const searchParams = new URLSearchParams(window.location.search);
      setRedirectTo(searchParams.get("redirectTo"));
    }
  }, []);

  const [generateToken, { loading }] = useMutation(GENERATE_TOKEN, {
    client,
    onCompleted: handleLoginSuccess,
    onError: (err) => setError(err.message),
  });

  function handleLoginSuccess(data: GenerateTokenResponse) {
    const token = data.admin.generateToken;
    document.cookie = `auth_token=${token}; path=/admin; secure; samesite=strict`;
    router.push(redirectTo || "/admin");
  }

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleLogin = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);

    try {
      await generateToken({
        variables: { input: formData },
      });
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      }
    }
  };

  return (
    <div className="p-6 max-w-md mx-auto">
      <h1 className="text-3xl font-semibold mb-6">Connexion</h1>
      <form onSubmit={handleLogin}>
        <FormInput
          label="Email"
          type="email"
          name="email"
          value={formData.email}
          onChange={handleInputChange}
          required
        />
        <FormInput
          label="Mot de passe"
          type="password"
          name="password"
          value={formData.password}
          onChange={handleInputChange}
          required
        />
        {error && <p className="text-red-600 mb-4">{error}</p>}
        <Button type="submit" className="w-full" disabled={loading}>
          {loading ? "Connexion..." : "Se connecter"}
        </Button>
      </form>
    </div>
  );
}

interface FormInputProps {
  label: string;
  type: string;
  name: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  required?: boolean;
}

function FormInput({
  label,
  type,
  name,
  value,
  onChange,
  required = false,
}: FormInputProps) {
  return (
    <div className="mb-4">
      <label className="block text-sm font-medium">{label}</label>
      <Input
        type={type}
        name={name}
        value={value}
        onChange={onChange}
        required={required}
        className="mt-1 block w-full"
      />
    </div>
  );
}
