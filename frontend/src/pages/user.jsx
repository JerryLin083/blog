import { useParams } from "@solidjs/router";
import { createSignal } from "solid-js";

import "./user.css";

function User() {
  //TODO: get user id and fetch data from server;
  const params = useParams();
  const [user, setUser] = createSignal(null);

  return (
    <div class="pixel-container user-detail">
      {user() ? (
        <>
          <div class="user-avatar">
            <h2>User name</h2>
          </div>
          <h2 class="pixel-user-title">{user().username}</h2>
          {/* 其他使用者詳細資訊 */}
        </>
      ) : (
        //TODO: use animation loading
        <p>Loading...</p>
      )}
    </div>
  );
}

export default User;
