import { useParams, useSearchParams } from "@solidjs/router";
import "./posts.css";

function Posts() {
  //TODO: Get page and fetch posts from server
  const [searchParams, _] = useSearchParams();

  return (
    <div class="pixel-container">
      <ul class="post-list">
        <div>Post card</div>
      </ul>
    </div>
  );
}

export default Posts;
