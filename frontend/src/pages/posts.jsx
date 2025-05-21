import { useSearchParams, useNavigate } from "@solidjs/router";
import { createSignal, createEffect, onMount } from "solid-js";

import "./posts.css";
import PostCard from "../components/post_card";
import Loading from "../components/loading";

function Posts() {
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
      let res = await fetch(`/api/posts?page=${currentPage}`);
      if (!res.ok) {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }

      let posts = await res.json();
      setPosts(posts);
    } catch (e) {
      console.error(e);
      navigate("/not-found", { replace: true });
    } finally {
      setIsLoading(false);
    }
  });

  const handlePage = (newPage) => {
    setSearchParams({ page: newPage });
  };

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container posts-container">
          <ul class="post-list">
            <For each={posts()}>{(post, _) => <PostCard post={post} />}</For>
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

export default Posts;
