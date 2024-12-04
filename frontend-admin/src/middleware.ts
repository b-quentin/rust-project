import { NextRequest, NextResponse } from "next/server";
import { checkAccessWithRustBackend } from "@/models/users/isAuth";
import { match, None, Some, Option } from "oxide.ts";
import { match as matchPattern , P } from "ts-pattern";
import { Logger } from "tslog";

const logs = new Logger({ name: "Middleware" });

// Redirect to login page with redirectTo query param
function redirectToLoginWithParams(req: NextRequest): NextResponse {
  const redirectTo = req.nextUrl.pathname + req.nextUrl.search;
  const loginUrl = new URL("/login", req.url);
  loginUrl.searchParams.set("redirectTo", redirectTo);

  logs.info("Redirecting to login page with params:", loginUrl.toString());

  return NextResponse.redirect(loginUrl);
}

// Middleware to handle access control
export async function middleware(req: NextRequest) {
  const token: Option<string> = matchPattern(req.cookies.get("auth_token")?.value)
    .with(P.string, () => Some(req.cookies.get("auth_token")?.value) as Option<string>)
    .otherwise(() => None);

  matchPattern(req.nextUrl.pathname)
    .with("/login", () => {
      logs.info("Login page, continuing to the requested page");
      return NextResponse.next();
    })
    .with("/api/login", () => {
      logs.info("API login page, continuing to the requested page");
      NextResponse.next();
    })
    .with("/register", () => {
      logs.info("Register page, continuing to the requested page");
      NextResponse.next();
    });

    const tokenMatch = match(token, {
      Some: (token) => token,
      None: () => {
        logs.info("No token found, redirecting to login page");
        redirectToLoginWithParams(req);
        return null;
      },
    });
  
    if (!tokenMatch) return;
  
    logs.info("Token found, checking access");
  
    match(await checkAccessWithRustBackend(tokenMatch, req.nextUrl.pathname), {
      Ok: () => {
        logs.info("Access granted, continuing to the requested page");
        NextResponse.next();
      },
      Err: () => {
        logs.info("Access denied, redirecting to login page");
        redirectToLoginWithParams(req);
      },
    });


}

export const config = {
  matcher: ["/:path*"],
};