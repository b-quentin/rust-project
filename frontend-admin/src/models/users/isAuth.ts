import { executeQuery } from "@/lib/graphql/client";
import { gql } from "@apollo/client";
import { Err, match, Ok, Result } from "oxide.ts";
import { StatusCode } from "status-code-enum";
import { Logger } from "tslog";

const logs = new Logger({ name: "Access Check" });

const GET_ACCESS_QUERY = gql`
  query GetAccessPage($token: String!, $page: String!) {
    admin {
      getAccessPage(token: $token, page: $page)
    }
  }
`;

async function checkAccessWithRustBackend(
    token: string,
    page: string,
  ): Promise<Result<boolean, { code: StatusCode, message: string }>> {
  return executeQuery<{ admin: { getAccessPage: boolean } }>(GET_ACCESS_QUERY, { token, page }).then((result) =>
    match(result, {
      Ok: (data) => {
        logs.info("Access checked successfully");
        logs.trace("Access:", data.admin.getAccessPage);
        return Ok(data.admin.getAccessPage) as Result<boolean, { code: StatusCode; message: string }>;
      },
      Err: (error) => {
        logs.error("Access check failed:", error.message);
        return Err({ code: error.code, message: error.message });
      },
    })
  );
}

export { checkAccessWithRustBackend };