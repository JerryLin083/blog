import { useParams } from "@solidjs/router";

function User() {
  //TODO: get user id and fetch data from server;

  const params = useParams();

  return <>User, ID:{params.id}</>;
}

export default User;
