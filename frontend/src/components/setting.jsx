import "./setting.css";
import logout from "../assets/icons-logout.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import close from "../assets/icons-close.svg";
import email from "../assets/icons-email.svg";
import phone from "../assets/icons-phone.svg";
import address from "../assets/icons-home.svg";
import edit from "../assets/icons-edit.svg";

function Setting(props) {
  let toggle = props.toggle;
  let user = props.user;

  const handleLogout = async () => {
    try {
      let res = await fetch("/api/logout");

      if (res.ok) {
        window.location.assign("/");
      } else {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div class="setting">
      <div class="setting-header">
        <button onClick={handleLogout}>
          <img src={logout} width="32" height="32" alt="logout" />
        </button>
        <button onClick={() => toggle(false)}>
          <img src={close} width="32" height="32" alt="close" />
        </button>
      </div>

      <div class="user-setting">
        <img
          src={user.id % 2 == 0 ? sloth_avatar : avacado_avatar}
          height="96"
          width="96"
          alt="avatar"
        />
        <div class="setting-info">
          <h4>
            {user.first_name} {user.last_name}
          </h4>
          <div class="user-edit">
            <div>
              <img src={email} height="20" width="20" alt="email" />
              <span>: {user.email ? user.email : "Unknown"}</span>
            </div>
            <button>
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
          <div class="user-edit">
            <div>
              <img src={phone} height="20" width="20" alt="phone" />
              <span>: {user.phone ? user.phone : "Unknown"}</span>
            </div>
            <button>
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
          <div class="user-edit">
            <div>
              <img src={address} height="20" width="20" alt="address" />
              <span>: {user.address ? user.address : "Unknown"}</span>
            </div>
            <button>
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
        </div>

        <div></div>
      </div>
    </div>
  );
}

export default Setting;
