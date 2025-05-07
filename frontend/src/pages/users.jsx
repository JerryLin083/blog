import { useSearchParams } from "@solidjs/router";
import "./users.css";

function Users() {
  //TODO: get page and fetch users from server
  const [searchParams, _] = useSearchParams();

  return (
    <div class="pixel-container">
      <ul class="user-list">
        <div>Users card</div>
      </ul>
    </div>
  );
}

export default Users;
