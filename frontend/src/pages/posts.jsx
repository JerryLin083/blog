import { useParams, useSearchParams } from "@solidjs/router";

function Posts() {
  //TODO: Get page and fetch posts from server
  const [searchParams, _] = useSearchParams();

  return <>page: {searchParams.page}, Posts</>;
}

export default Posts;
