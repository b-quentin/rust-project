import user from "../data/user";
import { match } from "oxide.ts";

export default function Home() {
  async function handleGenerateToken() {
    match(await user.generateToken({ email: "test@test.com", password: "test" }), {
      Ok: (value) => {
        console.log("Token generated successfully:", value);
      },
      Err: (error) => {
        console.error("Error generating token:", error.message);
      }
    });
  }

  handleGenerateToken();

  return (
    <div>
      <h1>Hello World</h1>
    </div>
  );
}
