import "./post_card.css";
import default_image from "../assets/icons-image.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import { A } from "@solidjs/router";

function PostCard(props) {
  let post = props.post;
  let post_published_date = new Date(post.published_at);

  return (
    <li key={post.id}>
      <div class="post-card">
        <div class="image-container">
          <img src={default_image} alt="image" height="128" width="128" />
        </div>
        <div class="content">
          <A href={`/posts/${post.id}`}>{post.title}</A>
          <p>{post.content}</p>
        </div>
        <div class="article">
          <img
            src={post.user_id % 2 == 0 ? sloth_avatar : avacado_avatar} //TODO: delete after dev
            alt="avatar"
            height="32"
            width="32"
          />
          <div class="article-info">
            <p>{post_published_date.toLocaleDateString()}</p>
            <p>
              {post.author ? (
                <A href={`/users/${post.user_id}`}>{post.author}</A>
              ) : (
                "unknown"
              )}
            </p>
          </div>
        </div>
      </div>
    </li>
  );
}

export default PostCard;
