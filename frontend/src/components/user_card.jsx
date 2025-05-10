import "./user_card.css";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import email from "../assets/icons-email.svg";
import phone from "../assets/icons-phone.svg";
import address from "../assets/icons-home.svg";
import { A } from "@solidjs/router";

function UserCard(props) {
  const user = props.user;

  return (
    <li key={user.id}>
      <div class="user-card">
        <img
          src={user.id % 2 == 0 ? sloth_avatar : avacado_avatar}
          alt="avatar"
          height="96"
          width="96"
        />
        <p>
          <A href={`/users/${user.id}`}>
            {user.first_name} {user.last_name}
          </A>
        </p>
        <div class="user-info">
          <div>
            <img src={email} height="20" width="20" alt="email" />
            <span>: {user.email ? user.email : "Unknown"}</span>
          </div>
          <div>
            <img src={phone} height="20" width="20" alt="phone" />
            <span>: {user.phone ? user.phone : "Unknown"}</span>
          </div>
          <div>
            <img src={address} height="20" width="20" alt="address" />
            <span>: {user.address ? user.address : "Unknown"}</span>
          </div>
        </div>
      </div>
    </li>
  );
}

export default UserCard;
