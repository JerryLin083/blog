import "./my_post_card.css";
import default_image from "../assets/icons-image.svg";
import edit from "../assets/icons-edit.svg";
import publish from "../assets/icons-publish.svg";
import done from "../assets/icons-done.svg";

function MyPostCard(props) {
  let post = props.post;
  let post_published_date = props.published_at
    ? new Date(props.published_at).toLocaleDateString
    : null;

  return (
    <li key={post.id}>
      <div class="my-post-card">
        <div class="image-container">
          <img src={default_image} alt="image" height="128" width="128" />
        </div>
        <div class="my-post-info">
          <p>published at: {post_published_date}</p>
          <p>{post.title}</p>
        </div>
        <div>
          {post_published_date ? (
            <div>
              <img src={done} height="24" width="24" alt="done" />
            </div>
          ) : (
            <button>
              <img src={publish} height="24" width="24" alt="publish" />
            </button>
          )}
          <p></p>
          <button>
            <img src={edit} height="24" width="24" alt="edit" />
          </button>
        </div>
      </div>
    </li>
  );
}

export default MyPostCard;
