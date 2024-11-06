import { NextRequest, NextResponse } from "next/server";
import client from "@lib/graphql/client";
import { gql } from "@apollo/client";

type GraphQLErrorExtension = {
  code: number;
  message: string;
};

type GraphQLErrorLocation = {
  line: number;
  column: number;
};

type GraphQLError = {
  message: string;
  locations?: GraphQLErrorLocation[];
  path?: string[];
  extensions?: GraphQLErrorExtension;
};

type ApolloError = {
  name: string;
  graphQLErrors: GraphQLError[];
};

// GraphQL query for access check
const GET_ACCESS_QUERY = gql`
  query GetAccessPage($token: String!, $page: String!) {
    admin {
      getAccessPage(token: $token, page: $page)
    }
  }
`;

// Function to check access using the Rust backend
async function checkAccessWithRustBackend(
  token: string,
  page: string,
  req: NextRequest,
): Promise<boolean> {
  try {
    console.log(page);
    const { data } = await client.query({
      query: GET_ACCESS_QUERY,
      variables: { token, page },
    });

    return data.admin?.getAccessPage || false;
  } catch (error) {
    handleError(error, req); // Pass 'req' as the second argument
    return false;
  }
}

// Handle specific error scenarios
function handleError(error: unknown, req: NextRequest): NextResponse | void {
  if (isTokenExpiredError(error)) {
    return redirectToLoginWithParams(req);
  } else {
    console.error("Error checking access:", error);
  }
}

// Check if the error is a token expiration error
function isTokenExpiredError(error: unknown): boolean {
  const apolloError = error as ApolloError;
  return (
    apolloError.graphQLErrors?.[0]?.extensions?.code === 401 &&
    apolloError.graphQLErrors?.[0]?.extensions?.message === "TOKEN_EXPIRED"
  );
}

// Redirect to login page with redirectTo query param
function redirectToLoginWithParams(req: NextRequest): NextResponse {
  const redirectTo = req.nextUrl.pathname + req.nextUrl.search;
  const loginUrl = new URL("/admin/login", req.url);
  loginUrl.searchParams.set("redirectTo", redirectTo);

  return NextResponse.redirect(loginUrl);
}

// Middleware to handle access control
export async function middleware(req: NextRequest) {
  const token = req.cookies.get("auth_token")?.value;
  const { pathname } = req.nextUrl;

  if (pathname === "/admin/login") return NextResponse.next();

  if (!token) {
    return redirectToLoginWithParams(req);
  }

  const hasAccess = await checkAccessWithRustBackend(token, pathname, req);
  return hasAccess ? NextResponse.next() : redirectToLoginWithParams(req);
}

export const config = {
  matcher: ["/admin/:path*"],
};
