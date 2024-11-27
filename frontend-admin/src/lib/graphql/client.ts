import { ApolloClient, ApolloError, InMemoryCache } from '@apollo/client';
import { Result, Ok, Err } from 'oxide.ts';
import { StatusCode } from 'status-code-enum'
import { Logger } from "tslog";

const logs = new Logger({ name: "GraphQL Client" });

const client = new ApolloClient({
  uri: 'http://127.0.0.1:8080/graphql',
  cache: new InMemoryCache()
});


export async function executeQuery<T>(query: any, variables: any = {}): Promise<Result<T, { message: string }>> {
  try {
    const result = await client.query<T>({ query, variables });

    logs.trace("GraphQL query result:", result.data);

    return Ok(result.data);
  } catch (error) {
    logs.error("GraphQL query error:", error);

    return Err({ message: "GraphQL query failed" });
  }
}

export async function executeMutation<T>(mutation: any, variables: any = {}): Promise<Result<T, { code: StatusCode, message: string }>> {
  try {
    const result = await client.mutate<T>({ mutation, variables });

    logs.trace("GraphQL mutation result:", result.data);

    if (!result.data) {
      logs.error("No data returned from mutation");
      return Err({ code: StatusCode.ClientErrorNotFound, message: "No data returned from mutation" });
    }

    return Ok(result.data);
  } catch (error) {
    logs.error("GraphQL mutation error structure:", JSON.stringify(error, null, 2));

    if (error instanceof ApolloError) {
      const extensions = error.graphQLErrors[0]?.extensions;
      if (extensions) {
        return Err({ code: extensions.code as StatusCode, message: extensions.message as string });
      }

      logs.error("GraphQL mutation error:", error);

      return Err({ code: StatusCode.ServerErrorInternal, message: "Unexpected error during GraphQL mutation: " + error });
    } else {
      logs.error("Unexpected error details:", error);

      return Err({ code: StatusCode.ServerErrorInternal, message: "Unexpected error during GraphQL mutation" });
    }
  }
}

export default client;