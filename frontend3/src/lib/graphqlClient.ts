import { GraphQLClient } from 'graphql-request';

// Initialize the GraphQL client with your endpoint
const client = new GraphQLClient('http://127.0.0.1:8080/graphql');

export default client;

