import { NextRequest, NextResponse } from "next/server";
import { checkAccessWithRustBackend } from "@/models/users/isAuth";
import { match, None, Some, Option, Err, Ok, Result } from "oxide.ts";

// Redirect to login page with redirectTo query param
function redirectToLoginWithParams(req: NextRequest): NextResponse {
  const redirectTo = req.nextUrl.pathname + req.nextUrl.search;
  const loginUrl = new URL("/login", req.url);
  loginUrl.searchParams.set("redirectTo", redirectTo);

  return NextResponse.redirect(loginUrl);
}

function getToken(req: NextRequest): Option<string> {
  const token = req.cookies.get("auth_token")?.value;
  if (!token) return None;
  return Some(token);
}

// Middleware to handle access control
export async function middleware(req: NextRequest) {
  const token = getToken(req);

  if (token.isNone()) {
    return redirectToLoginWithParams(req);
  }

  const { pathname } = req.nextUrl;
  if (pathname === "/login") return NextResponse.next();

  const result = await checkAccessWithRustBackend(token.unwrap(), pathname);

  if (result.isErr()) {
    console.error("Access check failed:", result.unwrapErr());
    return redirectToLoginWithParams(req);
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/:path*"],
};

