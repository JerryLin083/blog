import "./not_found.css";
import mushroom from "../assets/icons-mushroom.svg";

function NotFound() {
  return (
    <div class="not-found">
      <h1 class="pixel-error">404</h1>
      <p class="pixel-message">NOT FOUND</p>
      <img src={mushroom} height="360" width="360" />
    </div>
  );
}

export default NotFound;
