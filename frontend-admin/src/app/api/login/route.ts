import { NextRequest } from 'next/server';
import { generateToken } from '@/models/users/generateToken';
import { match } from 'oxide.ts';
import { StatusCode } from "status-code-enum";

export async function POST(req: NextRequest) {
  if (req.method === 'POST') {
    const { email, password } = await req.json();

    console.log('Email:', email, 'Password:', password);

    if (!email || !password) {
      return new Response(JSON.stringify({ message: 'Email et mot de passe sont requis' }), {
        status: StatusCode.ClientErrorBadRequest,
        headers: { 'Content-Type': 'application/json' },
      });
    }

    const result = await generateToken({ email, password });
    return match(result, {
      Ok: (data) => new Response(JSON.stringify({ token: data.admin.generateToken }), {
        status: StatusCode.SuccessOK,
        headers: { 'Content-Type': 'application/json' },
      }),
      Err: (error) => new Response(JSON.stringify({ message: error.message }), {
        status: error.code || StatusCode.ServerErrorInternal,
        headers: { 'Content-Type': 'application/json' },
      }),
    });
  } else {
    return new Response(`Method ${req.method} Not Allowed`, {
      status: StatusCode.ClientErrorMethodNotAllowed,
      headers: { 'Allow': 'POST' },
    });
  }
}