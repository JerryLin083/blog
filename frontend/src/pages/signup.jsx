import { createSignal } from "solid-js";

import "./signup.css";

function SignUp() {
  const [account, setAccount] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");

  const [isAccountExist, setIsAccountExist] = createSignal(false);
  const [isEqual, setIsEqual] = createSignal(true);
  const [isSubmit, setIsSubmit] = createSignal(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (password() !== confirmPassword()) {
      setIsEqual(false);

      return;
    }

    try {
      setIsSubmit(true);
      let signUpJsonBody = {
        account: account(),
        password: password(),
      };

      let accountRes = await fetch(`/api/account/${account()}`);
      if (accountRes.ok) {
        let accountResult = await accountRes.json();
        if (accountResult.result == "1") {
          let res = await fetch("/api/signup", {
            method: "POST",
            body: JSON.stringify(signUpJsonBody),
            headers: {
              "Content-Type": "application/json",
            },
          });

          if (res.ok) {
            window.location.assign("/");
          } else {
            throw new Error(`Failed to create account: ${res.status}`);
          }
        } else {
          setIsAccountExist(true);
        }
      } else {
        throw new Error(`Failed to check account: ${accountRes.status}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsSubmit(false);
      setIsEqual(true);
    }
  };

  return (
    <div class="signup-container">
      <h3>Sign up</h3>
      <form class="signup-form" onSubmit={handleSubmit}>
        <div>
          <span>Account: </span>
          <input
            value={account()}
            onInput={(e) => setAccount(e.currentTarget.value)}
          />
        </div>
        {isAccountExist() ? <p>account has been registered</p> : null}
        <div>
          <span>Password: </span>
          <input
            type="password"
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
          />
        </div>
        <div>
          <span>Confirm Password: </span>
          <input
            type="password"
            value={confirmPassword()}
            onInput={(e) => setConfirmPassword(e.currentTarget.value)}
          />
        </div>
        {isEqual() ? null : <p>Password do not match!!</p>}
        {isSubmit() ? null : <button type="submit">Confirm</button>}
      </form>
    </div>
  );
}

export default SignUp;
