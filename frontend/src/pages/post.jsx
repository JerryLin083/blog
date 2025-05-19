import { A, useNavigate, useParams } from "@solidjs/router";
import { createEffect, createSignal } from "solid-js";

import "./post.css";
import Loading from "../components/loading";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";

function Post() {
  const params = useParams();
  const [post, setPost] = createSignal({});
  const [isLoading, setIsLoading] = createSignal(false);
  const navigate = useNavigate();

  let post_published_date = () => new Date(post()?.published_at);

  createEffect(async () => {
    setIsLoading(true);
    try {
      let res = await fetch(`/api/posts/${params.id}`);
      if (!res.ok) {
        throw new Error(`Failed to fetch data ${res.status}`);
      }

      let post = await res.json();

      if (post.length === 0) {
        navigate("/not_found", { replace: true });
        return;
      }

      setPost(post[0]);
      setIsLoading(false);
    } catch (e) {
      console.error(e);
      navigate("/not-found", { replace: true });

      return;
    }
  });

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container post-detail">
          <div class="post-info">
            <p>{post_published_date().toLocaleDateString()}</p>
            <p>
              {post().author ? (
                <A href={`/users/${post().user_id}`}>{post().author}</A>
              ) : (
                "unknown"
              )}
            </p>
            <img
              src={post().user_id % 2 == 0 ? sloth_avatar : avacado_avatar}
              alt="avatar"
              height="48"
              width="48"
            />
          </div>
          <h3 class="pixel-post-title">{post().title}</h3>
          <section class="post-content">{post().content}</section>
        </div>
      )}
    </>
  );
}

export default Post;
