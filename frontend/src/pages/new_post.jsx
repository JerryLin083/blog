import { useNavigate } from "@solidjs/router";
import { createSignal, onMount } from "solid-js";

import "./new_post.css";

function NewPost() {
  const navigate = useNavigate();

  const [title, setTitle] = createSignal("");
  const [content, setContent] = createSignal("");
  const [submitting, setSubmitting] = createSignal(false);
  const [empty, setEmpty] = createSignal(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (title().length == 0 || content().length == 0) {
      setEmpty(true);
      return;
    }

    try {
      setSubmitting(true);
      let postJsonBody = {
        title: title(),
        content: content(),
      };

      let res = await fetch("/api/posts", {
        method: "POST",
        body: JSON.stringify(postJsonBody),
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (res.ok) {
        navigate(`/posts/myposts?page=1`);
      } else {
        throw new Error(`Failed to create post ${res.status}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setSubmitting(false);
      setEmpty(false);
    }
  };

  onMount(async () => {
    let auth_res = await fetch("/api/auth");
    try {
      if (auth_res.ok) {
        return;
      } else {
        throw new Error(`Unauthenticated ${auth_res.status}`);
      }
    } catch (e) {
      console.error(e);
      navigate("/login");
    }
  });

  return (
    <div class="new-post-container">
      <form onSubmit={handleSubmit}>
        <div>
          <h4>
            <label for="title">Title</label>
          </h4>
          <textarea
            id="title"
            value={title()}
            onInput={(e) => setTitle(e.currentTarget.value)}
          />
        </div>
        <div>
          <h4>
            <lable for="content">Content</lable>
          </h4>
          <textarea
            id="content"
            value={content()}
            onInput={(e) => setContent(e.currentTarget.value)}
          />
        </div>
        {empty() ? (
          <p style="color: red; font-size: small">
            Title or content can not be empty
          </p>
        ) : null}

        <div class="submit-button">
          {submitting() ? <span>waiting...</span> : <button>Save</button>}
        </div>
      </form>
    </div>
  );
}

export default NewPost;
