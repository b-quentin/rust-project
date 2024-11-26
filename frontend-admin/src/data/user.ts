import client from "@/lib/graphql/client";
import { gql } from "@apollo/client";

interface GenerateTokenInput {
  email: string;
  password: string;
}

interface GenerateTokenResponse {
  admin: {
    generateToken: string | undefined;
  };
}

const generateTokenMutation = gql`
  mutation GenerateToken($input: GenerateTokenInput!) {
    admin {
      generateToken(input: $input)
    }
  }
`;

async function generateToken(input: GenerateTokenInput): Promise<Result<GenerateTokenResponse, Err>> {
  return match(
    await client.mutate<GenerateTokenResponse>({ mutation: generateTokenMutation, variables: { input } }).then(
      (result) => ({ value: result.data?.admin.generateToken }),
      (error) => {
        console.log("Error during token generation:", error);
        return { error: {
          message: "Token generation failed" 
        } };
      }
    ),
    {
      Ok: (value) => value,
      Err: (error) => ({ message: error.message }),
    }
  );
}


export default { generateToken };