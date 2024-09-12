import { createGraphQLClient } from "@solid-primitives/graphql";

const client = createGraphQLClient("http://localhost:8080/graphql");

export default client;
