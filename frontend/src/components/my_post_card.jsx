import { useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

import "./my_post_card.css";
import default_image from "../assets/icons-image.svg";
import edit from "../assets/icons-edit.svg";
import publish from "../assets/icons-publish.svg";
import done from "../assets/icons-done.svg";
import close from "../assets/icons-close.svg";
import delete_img from "../assets/icons-delete.svg";

function MyPostCard(props) {
  let post = props.post;
  let post_published_date = post.published_at
    ? new Date(post.published_at).toLocaleDateString()
    : null;
  const navigate = useNavigate();

  const [isLoading, setIsLoading] = createSignal(false);
  const [title, setTitle] = createSignal(post.title);
  const [content, setContent] = createSignal(post.content);
  const [tempTitle, setTempTitle] = createSignal(post.title);
  const [tempContent, setTempContent] = createSignal(post.content);
  const [isEdit, setIsEdit] = createSignal(false);

  const handlePublish = async () => {
    setIsLoading(true);
    try {
      let res = await fetch(`/api/posts/publish/${post.id}`, {
        method: "PATCH",
      });
      if (!res.ok) {
        throw new Error(`Failed to publish post: ${res.status}`);
      }
      navigate(`../${post.id}`);
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setIsEdit(false);
    setTempTitle(title());
    setTempContent(content());
  };

  const handleDelete = async () => {
    setIsLoading(true);
    try {
      let res = await fetch(`/api/posts/${post.id}`, { method: "DELETE" });
      if (!res.ok) {
        throw new Error(`Failed to delete post: ${res.status}`);
      }

      props.setPosts((prevPosts) => prevPosts.filter((p) => p.id != post.id));
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    setIsLoading(true);
    let postJsonBody = {
      title: tempTitle(),
      content: tempContent(),
    };

    try {
      let res = await fetch(`/api/posts/edit/${post.id}`, {
        method: "PATCH",
        body: JSON.stringify(postJsonBody),
        headers: { "Content-Type": "application/json" },
      });

      if (!res.ok) {
        throw new Error(`Failed to edit post: ${res.status}`);
      }

      setTitle(tempTitle());
      setContent(tempContent());
      setIsEdit(false);
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <li key={post.id}>
      <div class="my-post-card">
        <div class="image-container">
          <img src={default_image} alt="image" height="128" width="128" />
        </div>
        <div class="my-post-info">
          <p>published at: {post_published_date}</p>
          <p>{title()}</p>
        </div>
        <div class="mypost-buttons">
          {post_published_date ? (
            <div>
              <img src={done} height="24" width="24" alt="done" />
            </div>
          ) : (
            <button onClick={handlePublish} disabled={isLoading()}>
              <img src={publish} height="24" width="24" alt="publish" />
            </button>
          )}
          <button onClick={() => setIsEdit(!isEdit())}>
            <img src={edit} height="24" width="24" alt="edit" />
          </button>
          <button onClick={handleDelete} disabled={isLoading()}>
            <img src={delete_img} height="24" width="24" alt="delete" />
          </button>
        </div>

        {/* edit form */}
        {isEdit() ? (
          <form onSubmit={handleSubmit}>
            <div class="mypost-edit-close" onClick={handleClose}>
              <img src={close} height="24" width="24" alt="close" />
            </div>
            <div>
              <h4>
                <label for="title">Title</label>
              </h4>
              <textarea
                id="title"
                value={tempTitle()}
                onInput={(e) => setTempTitle(e.currentTarget.value)}
              />
            </div>
            <div>
              <h4>
                <label for="content">Content</label>
              </h4>
              <textarea
                id="content"
                value={tempContent()}
                onInput={(e) => setTempContent(e.currentTarget.value)}
              />
            </div>
            <p></p>
            <button disabled={isLoading()}>Edit</button>
          </form>
        ) : null}
      </div>
    </li>
  );
}

export default MyPostCard;
