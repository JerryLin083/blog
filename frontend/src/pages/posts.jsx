import { useSearchParams, useNavigate } from "@solidjs/router";
import { createSignal, createEffect } from "solid-js";

import "./posts.css";
import PostCard from "../components/post_card";
import Loading from "../components/loading";

function Posts() {
  const [isLoading, setIsLoading] = createSignal(false);
  const [posts, setPosts] = createSignal([]); // 假設 dummyData 已經定義
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();

  // 從 searchParams 中獲取 page 值，並在改變時觸發 effect
  const page = () => searchParams.page;

  createEffect(async () => {
    const currentPage = page();
    if (!currentPage) {
      navigate("/not-found", { replace: true });
      return;
    }

    setIsLoading(true);
    // TODO: 使用 currentPage 獲取對應頁碼的資料
    let res = await fetch(`/api/posts?page=${currentPage}`);
    try {
      let posts = await res.json();
      setPosts(posts);
      setIsLoading(false);
    } catch (e) {
      navigate("/not-found", { replace: true });
      return;
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
            {posts().length > 0 ? (
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
