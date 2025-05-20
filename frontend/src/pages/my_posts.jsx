import { createEffect, createSignal, onMount } from "solid-js";
import { useNavigate, useSearchParams } from "@solidjs/router";

import "./my_posts.css";
import Loading from "../components/loading";
import MyPostCard from "../components/my_post_card";

function MyPosts() {
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();

  const [isLoading, setIsLoading] = createSignal(false);
  const [posts, setPosts] = createSignal(dummyData);

  const page = () => searchParams.page;

  createEffect(async () => {
    const currentPage = page();
    if (!currentPage) {
      navigate("/not-found", { replace: true });
    }

    setIsLoading(true);
    try {
      let res = await fetch(`api/posts/myposts?paage${currentPage}`);
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

  // onMount(async () => {
  //   try {
  //     let res = await fetch("/api/posts/myposts?page=1");
  //     if (res.ok) {
  //       let myposts = await res.json();

  //       console.log("mypost: ", myposts);

  //       setPosts(myposts);
  //     } else {
  //       throw new Error(`Failed to fetch data ${res.status}`);
  //     }
  //   } catch (e) {
  //     console.error(e);
  //   }
  // });

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container posts-container">
          <ul class="post-list">
            <For each={posts()}>{(post, _) => <MyPostCard post={post} />}</For>
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

const dummyData = [
  {
    id: 64,
    title: "Hello from test123",
    content: "Hi there, I'm test123 how are you everybody",
    user_id: 25,
    author: "",
    create_at: "2025-05-16T01:43:54.043188Z",
    update_at: "2025-05-16T01:43:54.043188Z",
    published_at: null,
  },
  {
    id: 65,
    title: "The Urgent Call for Conservation",
    content:
      "Our planet, a vibrant tapestry of life, is facing an unprecedented biodiversity crisis. From the depths of the ocean to the peaks of towering mountains, countless species are teetering on the brink of extinction due to habitat loss, climate change, pollution, and unsustainable exploitation. Conservation is no longer a mere environmental concern; it is a fundamental imperative for the well-being of our planet and future generations.\n\nProtecting endangered species and their habitats is crucial for maintaining the delicate balance of ecosystems. Each organism, no matter how small, plays a vital role in the intricate web of life, contributing to essential processes like pollination, nutrient cycling, and climate regulation. The loss of even a single species can trigger cascading effects, destabilizing entire ecosystems and diminishing the invaluable services they provide.\n\nConservation efforts encompass a wide range of actions, from establishing protected areas and implementing sustainable resource management practices to combating illegal wildlife trade and raising public awareness. It requires a collaborative approach, involving governments, organizations, communities, and individuals working together to safeguard our natural heritage. By embracing conservation, we invest in a healthier, more resilient planet, ensuring that the wonders of the natural world endure for generations to come.",
    user_id: 25,
    author: "",
    create_at: "2025-05-20T04:35:11.062569Z",
    update_at: "2025-05-20T04:35:11.062569Z",
    published_at: null,
  },
];
