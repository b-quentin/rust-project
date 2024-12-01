import { gql } from "@apollo/client";
import { Result, Ok, Err, match } from "oxide.ts";
import { executeMutation } from "@/lib/graphql/client";
import { Logger } from "tslog";
import { StatusCode } from "status-code-enum";

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

export async function generateToken(input: GenerateTokenInput): Promise<Result<GenerateTokenResponse, { code: StatusCode; message: string }>> {
  return executeMutation<GenerateTokenResponse>(generateTokenMutation, { input }).then((result) =>
    match(result, {
      Ok: (data) => {
        logs.info("Token generated successfully");
        logs.trace("Token:", data.admin.generateToken);
        return Ok(data) as Result<GenerateTokenResponse, { code: StatusCode; message: string }>;
      },
      Err: (error) => {
        logs.error("Token generation failed:", error.message);
        return Err({ code: error.code, message: error.message });
      },
    })
  );
}