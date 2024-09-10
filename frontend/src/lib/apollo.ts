import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

export function createApolloClient(customFetch: typeof fetch) {
  const httpLink = new HttpLink({
    uri: 'http://127.0.0.1:8080/graphql',
    fetch: customFetch,
    headers: {
      'Content-Type': 'application/json'
    }
  });

  const authLink = setContext((_, { headers }) => {
    return {
      headers: {
        ...headers
        // Add any custom headers like Authorization if necessary
      }
    };
  });

  return new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache(),
    ssrMode: true // Enable SSR mode
  });
}
