import { useParams } from "@solidjs/router";
import { createSignal } from "solid-js";

import "./post.css";

function Post() {
  //TODO: get post id and fetch data from server;
  const params = useParams();

  const [post, setPost] = createSignal(null);

  return (
    <div class="pixel-container post-detail">
      {post() ? (
        <>
          <h1 class="pixel-post-title">{post().title}</h1>
          <div class="post-content">{post().content}</div>{" "}
          {/* 使用 dangerouslySetInnerHTML 注意安全性 */}
          {/* 其他文章詳細資訊，例如作者、日期等 */}
        </>
      ) : (
        // TODO: use animation loading
        <p>Loading...</p>
      )}
    </div>
  );
}

export default Post;
