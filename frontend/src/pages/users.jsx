import { useSearchParams } from "@solidjs/router";

function Users() {
  //TODO: get page and fetch users from server
  const [searchParams, _] = useSearchParams();

  return <>page: {searchParams.page}, user</>;
}

export default Users;
