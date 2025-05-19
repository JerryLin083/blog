import { createSignal } from "solid-js";
import "./login.css";

function Login() {
  const [account, setAccount] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [logging, setLogging] = createSignal(false);
  const [isFail, setIsFail] = createSignal(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    setLogging(true);
    try {
      let loginJsonBody = {
        account: account(),
        password: password(),
      };

      let res = await fetch("api/login", {
        method: "POST",
        body: JSON.stringify(loginJsonBody),
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (res.ok) {
        window.location.assign("/");
      } else {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }
    } catch (e) {
      console.error(e);
      setIsFail(true);
      setLogging(false);
    }
  };

  return (
    <div class="login-container">
      <h3>Login</h3>
      <form class="login-form" onSubmit={handleSubmit}>
        <div>
          <span>Account: </span>
          <input
            value={account()}
            onInput={(e) => {
              setAccount(e.currentTarget.value);
            }}
          />
        </div>

        <div>
          <span>Password: </span>
          <input
            type="password"
            value={password()}
            onInput={(e) => {
              setPassword(e.currentTarget.value);
            }}
          />
        </div>
        {isFail() ? (
          <p style="color: red; font-size: small">Login failed</p>
        ) : null}
        {logging() ? <p>Logging in ...</p> : <button>Login</button>}
      </form>
    </div>
  );
}

export default Login;
