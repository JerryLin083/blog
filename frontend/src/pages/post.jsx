import { useParams } from "@solidjs/router";

function Post() {
  //TODO: get post id and fetch data from server;
  const params = useParams();

  return <>Post, ID: {params.id}</>;
}

export default Post;
