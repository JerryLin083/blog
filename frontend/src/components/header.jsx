import "./header.css";
import { A, useNavigate } from "@solidjs/router";
import { createSignal, onMount } from "solid-js";

import blog from "../assets/icons-blog.svg";
import logout from "../assets/icons-logout.svg";

function Header() {
  const [auth, setAuth] = createSignal(true);
  const navigate = useNavigate();

  const handleLogout = async () => {
    try {
      let res = await fetch("/api/logout");

      if (res.ok) {
        setAuth(false);
        navigate("/");
      } else {
        console.log(res);
      }
    } catch (e) {
      console.error("Logout failed: ", e);
    }
  };

  onMount(async () => {
    try {
      let res = await fetch("/api/auth");
      if (res.ok) {
        setAuth(true);
      } else {
        let msg = await res.text();
        console.log(msg);
        setAuth(false);
      }
    } catch (e) {
      console.error("Fetch auth failed: ", e);
      setAuth(false);
    }
  });

  return (
    <header class="header">
      <nav class="nav_bar">
        <div>
          <img src={blog} alt="logo" width="64" height="64" />
        </div>
        <div class="nav_link">
          <A href="/" class="link">
            Home
          </A>
          <A href="/posts?page=1" class="link">
            Posts
          </A>
          <A href="/users?page=1" class="link">
            Users
          </A>
          {auth() ? (
            <div onClick={handleLogout} class="logout-block">
              <img src={logout} height="48" width="48">
                Logout
              </img>
            </div>
          ) : (
            <A href="/login" class="link">
              Login
            </A>
          )}
        </div>
      </nav>
    </header>
  );
}

export default Header;
