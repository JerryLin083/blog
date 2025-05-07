import "./header.css";
import blog from "../assets/icons-blog.svg";
import { A } from "@solidjs/router";

function Header() {
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
        </div>
      </nav>
    </header>
  );
}

export default Header;
