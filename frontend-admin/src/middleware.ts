import { NextRequest, NextResponse } from "next/server";
import { checkAccessWithRustBackend } from "@/models/users/isAuth";
import { match, None, Some, Option } from "oxide.ts";
import { match as matchPattern , P } from "ts-pattern";

// Redirect to login page with redirectTo query param
function redirectToLoginWithParams(req: NextRequest): NextResponse {
  const redirectTo = req.nextUrl.pathname + req.nextUrl.search;
  const loginUrl = new URL("/login", req.url);
  loginUrl.searchParams.set("redirectTo", redirectTo);

  return NextResponse.redirect(loginUrl);
}

// Middleware to handle access control
export async function middleware(req: NextRequest) {
  const token: Option<string> = matchPattern(req.cookies.get("auth_token")?.value)
    .with(P.string, () => Some(req.cookies.get("auth_token")?.value) as Option<string>)
    .otherwise(() => None);

  if (token.isNone()) {
     redirectToLoginWithParams(req);
  }

  matchPattern(req.nextUrl.pathname)
    .with("/login", () => NextResponse.next())
    .with("/api/login", () => NextResponse.next())
    .with("/register", () => NextResponse.next())
    .otherwise(() => {});

  match(await checkAccessWithRustBackend(token.unwrap(), req.nextUrl.pathname), {
    Ok: () => NextResponse.next(),
    Err: () => redirectToLoginWithParams(req),
  });
}

export const config = {
  matcher: ["/:path*"],
};