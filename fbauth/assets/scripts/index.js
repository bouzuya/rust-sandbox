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

async function signIn({ sessionToken }, { code, state }) {
  const response = await fetch("/sign_in", {
    body: JSON.stringify({
      code,
      state,
    }),
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
  const { session_token: newSessionToken } = responseBody;
  console.log("sign in parsed response body", JSON.stringify(responseBody));
  return { sessionToken: newSessionToken };
}

async function signUp({ sessionToken }, { code, state }) {
  const response = await fetch("/sign_up", {
    body: JSON.stringify({
      code,
      state,
    }),
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
  console.log("associate google account parsed response body", JSON.stringify(responseBody));
  return responseBody;
}

async function callback(code, state) {
  const callbackAction = localStorage.getItem("callback_action");
  if (!(callbackAction === "sign_in" || callbackAction === "sign_up")) {
    throw new Error("stored callback_action");
  }
  localStorage.removeItem("callback_action");
  const stored = localStorage.getItem("session");
  if (stored === null) {
    throw new Error("stored session is invalid");
  }
  const session = JSON.parse(stored);

  switch (callbackAction) {
    case "sign_in": {
      const response = await signIn(session, { code, state });
      console.log(response);
      return;
    }
    case "sign_up": {
      const response = await signUp(session, { code, state });
      console.log(response);
      return;
    }
    default: {
      throw new Error("unknown callback_action");
    }
  }
}

async function initial() {
  const bodyElement = document.querySelector('body');
  const rootElement = document.createElement('div');
  bodyElement.appendChild(rootElement);
  const resetButtonElement = document.createElement('button');
  resetButtonElement.appendChild(document.createTextNode('Reset'));
  resetButtonElement.addEventListener('click', (e) => {
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      localStorage.removeItem(key);
    }
  });
  rootElement.appendChild(resetButtonElement);

  const user = await (async () => {
    const key = "user";
    const stored = localStorage.getItem(key);
    if (stored === null) {
      const user = await createUser();
      localStorage.setItem(key, JSON.stringify(user));
      return user;
    } else {
      const user = JSON.parse(stored);
      if (
        typeof user === "object" &&
          user !== null &&
          "userId" in user &&
          typeof user["userId"] === "string" &&
          "userSecret" in user &&
          typeof user["userSecret"] === "string"
      ) {
        return user;
      } else {
        localStorage.removeItem(key);
        throw new Error("stored user is invalid");
      }
    }
  })();
  console.log(user);

  const session = await (async (user) => {
    const key = "session";
    const stored = localStorage.getItem(key);
    if (stored === null) {
      const session = await createSession(user);
      localStorage.setItem(key, JSON.stringify(session));
      return session;
    } else {
      const session = JSON.parse(stored);
      if (
        typeof session === "object" &&
          session !== null &&
          "sessionToken" in session &&
          typeof user["sessionToken"] === "string"
      ) {
        return session;
      } else {
        localStorage.removeItem(key);
        throw new Error("stored session is invalid");
      }
     }
  })(user);
  console.log(session);

  const signUpButtonElement = document.createElement('button');
  signUpButtonElement.appendChild(document.createTextNode('SignUp'));
  signUpButtonElement.addEventListener('click', (_e) => {
    void (async () => {
      const { authorizationUrl } = await createAuthorizationUrl(session);
      localStorage.setItem("callback_action", "sign_up");
      window.location.href = authorizationUrl;
      // redirect with code and state parameters
    })();
  });
  rootElement.appendChild(signUpButtonElement);
  const signInButtonElement = document.createElement('button');
  signInButtonElement.appendChild(document.createTextNode('SignIn'));
  signInButtonElement.addEventListener('click', (_e) => {
    void (async () => {
      const { authorizationUrl } = await createAuthorizationUrl(session);
      localStorage.setItem("callback_action", "sign_in");
      window.location.href = authorizationUrl;
      // redirect with code and state parameters
    })();
  });
  rootElement.appendChild(signInButtonElement);
}

async function main() {
  const url = new URL(window.location.href);
  if (url.searchParams.get("code") === null || url.searchParams.get("state") === null) {
    initial();
  } else {
    callback(url.searchParams.get("code"), url.searchParams.get("state"));
  }
}

main();
