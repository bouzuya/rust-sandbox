async function createAuthorizationUrl({ sessionToken }) {
  const response = await fetch("/authorization_urls", {
    headers: {
      "Authorization": `Bearer ${sessionToken}`,
      "Content-Type": "application/json"
    },
    method: "POST"
  });
  if (((response.status / 100) | 0) !== 2) {
    throw new Error(`response status code is not success (code = ${response.status})`);
  }
  const responseBody = await response.json();
  const { authorization_url: authorizationUrl } = responseBody;
  console.log("create authorization url parsed response body", { authorizationUrl });
  return { authorizationUrl };
}

async function createUser() {
  const response = await fetch("/users", {
    headers: {
      "Content-Type": "application/json"
    },
    method: "POST"
  });
  if (((response.status / 100) | 0) !== 2) {
    throw new Error(`response status code is not success (code = ${response.status})`);
  }
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
  if (((response.status / 100) | 0) !== 2) {
    throw new Error(`response status code is not success (code = ${response.status})`);
  }
  const responseBody = await response.json();
  const { session_token: sessionToken } = responseBody;
  console.log("create session parsed response body", { sessionToken });
  return { sessionToken };
}

async function main() {
  const user = await createUser();
  console.log(user);
  const session = await createSession(user);
  console.log(session);

  const bodyElement = document.querySelector('body');
  const rootElement = document.createElement('div');
  const signUpButtonElement = document.createElement('button');
  signUpButtonElement.appendChild(document.createTextNode('SignUp'));
  signUpButtonElement.addEventListener('click', (e) => {
    void (async () => {
      const { authorizationUrl } = await createAuthorizationUrl(session);
      console.log(authorizationUrl);
    })();
  });
  rootElement.appendChild(signUpButtonElement);
  const signInButtonElement = document.createElement('button');
  signInButtonElement.appendChild(document.createTextNode('SignUp'));
  signInButtonElement.addEventListener('click', (e) => {
    void (async () => {
      console.log('FIXME: sign in');
    })();
  });
  bodyElement.appendChild(rootElement);
}

main();
