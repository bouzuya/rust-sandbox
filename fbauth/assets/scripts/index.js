async function createUser() {
  const response = await fetch("/users", {
    headers: {
      "Content-Type": "application/json"
    },
    method: "POST"
  });
  console.log("create user response status", { status: response.status });
  const responseBody = await response.json();
  const { user_id: userId, user_secret: userSecret } = responseBody;
  console.log("create user parsed response body", { userId, userSecret });
  return { userId, userSecret };
}

async function createSession({ userId, userSecret }) {
  const response = await fetch("/sessions", {
    body: JSON.stringify({
      user_id: userId,
      user_secret: userSecret
    }),
    headers: {
      "Content-Type": "application/json"
    },
    method: "POST",
  });
  console.log("create session response status", { status: response.status });
  const responseBody = await response.json();
  const { session_token: sessionToken } = responseBody;
  console.log("create session parsed response body", { sessionToken });
  return { sessionToken };
}

async function main() {
  const user = await createUser();
  const session = await createSession(user);
  console.log(session);
}

main();
