import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import "./login.css";

function Login() {
  const [account, setAccount] = createSignal("");
  const [password, setPassword] = createSignal("");

  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

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
        navigate("/");
      } else {
        const errorData = await res.json();
        console.error("Login failed: ", errorData);
      }
    } catch (e) {
      console.log(e);
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

        <button>Login</button>
      </form>
    </div>
  );
}

export default Login;
