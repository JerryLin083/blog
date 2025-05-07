import { createSignal } from "solid-js";

import Post from "./post";
import "./home.css";

function Home() {
  //TODO: fetch posts from server;
  return (
    <div class="pixel-container">
      <h1 class="pixel-title">Jerry's Blog</h1>
      <section class="latest-posts">
        <h2>Lastest</h2>
        <ul>
          <Post />
        </ul>
      </section>
      {/* 其他首頁內容，例如簡介、特色區塊等 */}
    </div>
  );
}

export default Home;
