import { useNavigate, useParams } from "@solidjs/router";
import { createEffect, createSignal } from "solid-js";

import "./user.css";
import Loading from "../components/loading";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import email from "../assets/icons-email.svg";
import phone from "../assets/icons-phone.svg";
import address from "../assets/icons-home.svg";

function User() {
  const params = useParams();
  const [user, setUser] = createSignal({});
  const [isLoading, setIsLoading] = createSignal(false);
  const navigate = useNavigate();

  createEffect(async () => {
    setIsLoading(true);
    try {
      let res = await fetch(`/api/users/${params.id}`);
      if (!res.ok) {
        throw new Error(`Failed to fetch data ${res.status}`);
      }

      let user = await res.json();

      if (user.length === 0) {
        navigate("/not_found", { replace: true });
        return;
      }

      setUser(user[0]);
      setIsLoading(false);
    } catch (e) {
      console.error(e);
      navigate("/not_found", { replace: true });
      return;
    }
  });

  return (
    <>
      {isLoading() ? (
        <Loading />
      ) : (
        <div class="pixel-container user-detail">
          <div class="user-avatar">
            <img
              src={user().id % 2 == 0 ? sloth_avatar : avacado_avatar}
              height="128"
              width="128"
              alt="avatar"
            />
          </div>
          <h2 class="pixel-user-title">
            {user().first_name} {user().last_name}
          </h2>
          <div class="user-info">
            <div>
              <img src={email} height="20" width="20" alt="email" />
              <span>: {user().email ? user().email : "Unknown"}</span>
            </div>
            <div>
              <img src={phone} height="20" width="20" alt="phone" />
              <span>: {user().phone ? user().phone : "Unknown"}</span>
            </div>
            <div>
              <img src={address} height="20" width="20" alt="address" />
              <span>: {user().address ? user().address : "Unknown"}</span>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

export default User;
