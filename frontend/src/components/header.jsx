import "./header.css";
import { A } from "@solidjs/router";
import { createSignal, onMount } from "solid-js";

import blog from "../assets/icons-blog.svg";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import Setting from "./setting";

function Header() {
  const [auth, setAuth] = createSignal(true);
  const [user, setUser] = createSignal({});
  const [toggle, setToggle] = createSignal(false);

  onMount(async () => {
    try {
      let res = await fetch("/api/auth/user");
      if (res.ok) {
        let user = await res.json();

        setUser(user[0]);
        setAuth(true);
      } else {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }
    } catch (e) {
      console.error(e);
      setAuth(false);
    }
  });

  return (
    <header class="header">
      <nav class="nav_bar">
        <div>
          <A href="/">
            <img src={blog} alt="logo" width="64" height="64" />
          </A>
        </div>
        <div class="nav_link">
          <A href="/posts?page=1" class="link">
            Posts
          </A>
          <A href="/users?page=1" class="link">
            Users
          </A>
          <A href="/posts/myposts?page=1" class="link">
            My Posts
          </A>
          {auth() ? (
            <div class="setting-container">
              {toggle() ? (
                <Setting toggle={setToggle} user={user()} setUser={setUser} />
              ) : (
                <div class="avatar-block" onClick={() => setToggle(!toggle())}>
                  <img
                    src={user().id % 2 == 0 ? sloth_avatar : avacado_avatar}
                    height="48"
                    width="48"
                    alt="avatar"
                  />
                </div>
              )}
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
