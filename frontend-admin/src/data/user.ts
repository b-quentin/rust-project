import client from "@/lib/graphql/client";
import { gql } from "@apollo/client";
import { Result, Ok, Err, match } from "oxide.ts";
import { executeMutation } from "@/lib/graphql/client";
import { Logger } from "tslog";

const logs = new Logger({ name: "Token Generation" });

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

async function generateToken(input: GenerateTokenInput): Promise<Result<GenerateTokenResponse, { message: string }>> {
    return match(await executeMutation<GenerateTokenResponse>(generateTokenMutation, { input }), {
      Ok: (data) => {
        if (data.admin && data.admin.generateToken) {
          logs.info("Token generated successfully");
          logs.trace("Token:", data.admin.generateToken);

          return Ok(data);
        } else {
          logs.error("Token generation failed: No token returned");

          return Err({ message: "Token generation failed: No token returned" });
        }
      },
      Err: (error) => {
        logs.error("Token generation failed");
        logs.trace("Token generation failed:", error);

        return Err(error);
    }
  });
}


export default { generateToken };