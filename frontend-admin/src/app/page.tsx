import Image from "next/image";
import user from "../data/user";

export default function Home() {
  const test = user.generateToken({ email: "test@test.com", password: "test" });
  console.log(test);

  return (
    <div>
      <h1>Hello World</h1>
    </div>
  );
}
