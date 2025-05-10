import { useNavigate, useSearchParams } from "@solidjs/router";
import "./users.css";
import { createEffect, createSignal } from "solid-js";
import Loading from "../components/loading";
import UserCard from "../components/user_card";

function Users() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [users, setUsers] = createSignal([]);
  const [isLoading, setIsLoading] = createSignal(false);
  const navigate = useNavigate();

  const page = () => searchParams.page;

  createEffect(async () => {
    const currentPage = page();
    if (!currentPage) {
      navigate("/not-found", { replace: ture });
      return;
    }

    setIsLoading(true);
    try {
      let res = await fetch(`/api/users?page=${currentPage}`);
      let users = await res.json();

      setUsers(users);
      setIsLoading(false);
    } catch (e) {
      navigate("/not-found", { replace: true });
      return;
    }
  });

  const handlepage = (newPage) => {
    setSearchParams({ page: newPage });
  };

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container users-container">
          <ul class="user-list">
            <For each={users()}>{(user, _) => <UserCard user={user} />}</For>
          </ul>
          <div>
            {Number(page()) > 1 ? (
              <button onClick={() => handlepage(Number(page()) - 1)}>
                Prev
              </button>
            ) : null}{" "}
            {users().length == 10 ? (
              <button onClick={() => handlepage(Number(page()) + 1)}>
                Next
              </button>
            ) : null}
          </div>
        </div>
      )}
    </>
  );
}

export default Users;
