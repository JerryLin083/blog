import { createEffect, createSignal, onMount } from "solid-js";
import { useNavigate, useSearchParams } from "@solidjs/router";

import "./my_posts.css";
import Loading from "../components/loading";
import MyPostCard from "../components/my_post_card";
import post_icon from "../assets/icons-post.svg";

function MyPosts() {
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();

  const [isLoading, setIsLoading] = createSignal(false);
  const [posts, setPosts] = createSignal([]);

  const page = () => searchParams.page;

  createEffect(async () => {
    const currentPage = page();
    if (!currentPage) {
      navigate("/not-found", { replace: true });
    }

    setIsLoading(true);
    try {
      let res = await fetch(`/api/posts/myposts?page=${currentPage}`);
      if (!res.ok) {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }

      let myPosts = await res.json();
      setPosts(myPosts);
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  });

  const handlePage = (newPage) => {
    setSearchParams({ page: newPage });
  };

  onMount(async () => {
    try {
      let res = await fetch("/api/posts/myposts?page=1");
      if (res.ok) {
        let myposts = await res.json();
        setPosts(myposts);
      } else {
        if (res.status == 401) {
          navigate("/login");
          return;
        }
        throw new Error(`Failed to fetch data ${res.status}`);
      }
    } catch (e) {
      console.error(e);
    }
  });

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container posts-container">
          <div class="posts-header">
            <button onClick={() => navigate("/posts/new")}>
              <img src={post_icon} height="36" width="36" alt="new post" />
            </button>
          </div>

          <ul class="post-list">
            <For each={posts()}>
              {(post, _) => <MyPostCard post={post} setPosts={setPosts} />}
            </For>
          </ul>

          <div>
            {Number(page()) > 1 ? (
              <button onClick={() => handlePage(Number(page()) - 1)}>
                Prev
              </button>
            ) : null}{" "}
            {posts().length == 10 ? (
              <button onClick={() => handlePage(Number(page()) + 1)}>
                Next
              </button>
            ) : null}
          </div>
        </div>
      )}
    </>
  );
}
export default MyPosts;
