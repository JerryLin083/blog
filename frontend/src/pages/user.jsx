import { useParams } from "@solidjs/router";
import { createEffect, createSignal } from "solid-js";

import "./user.css";
import Loading from "../components/loading";

function User() {
  //TODO: get user id and fetch data from server;
  const params = useParams();
  const [user, setUser] = createSignal(null);
  const [isLoading, setIsLoading] = createSignal(false);

  createEffect(async () => {
    setIsLoading(true);
    // let res = await fetch(`/users/${params.id}`);
    // let user = await res.json();

    // setUser(user);
    // setIsLoading(false);
  });

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container user-detail">
          <div class="user-avatar">
            <h2>User name</h2>
          </div>
          <h2 class="pixel-user-title">username</h2>
          {/* 其他使用者詳細資訊 */}
        </div>
      )}
    </>
  );
}

export default User;
