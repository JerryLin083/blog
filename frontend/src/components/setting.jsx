import "./setting.css";
import logout from "../assets/icons-logout.svg";
import avacado_avatar from "../assets/icons-avatar-avacado.svg";
import sloth_avatar from "../assets/icons-avatar-sloth.svg";
import close from "../assets/icons-close.svg";
import email_icon from "../assets/icons-email.svg";
import phone_icon from "../assets/icons-phone.svg";
import address_icon from "../assets/icons-home.svg";
import edit from "../assets/icons-edit.svg";
import { createSignal } from "solid-js";

function Setting(props) {
  let toggle = props.toggle;
  let user = props.user;

  let [submitting, setSubmitting] = createSignal(false);
  let [infoEdit, setInfoEdit] = createSignal({
    username: true,
    email: true,
    phone: true,
    address: true,
  });

  let [firstName, setFirstName] = createSignal(user.first_name);
  let [lastName, setLastName] = createSignal(user.last_name);
  let [email, setEmail] = createSignal(user.email);
  let [phone, setPhone] = createSignal(user.phone);
  let [address, setAddress] = createSignal(user.address);

  const handleLogout = async () => {
    try {
      let res = await fetch("/api/logout");
      if (!res.ok) {
        throw new Error(`Failed to fetch data: ${res.status}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      window.location.assign("/");
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      setSubmitting(true);
      let edit_user_body = {
        first_name: firstName(),
        last_name: lastName(),
        email: email(),
        phone: phone(),
        address: address(),
      };
      let res = await fetch("/api/user", {
        method: "PATCH",
        body: JSON.stringify(edit_user_body),
        headers: { "Content-Type": "application/json" },
      });

      if (res.ok) {
      } else {
        throw new Error(`Failed to patch data: ${res.status}`);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setInfoEdit({
        username: true,
        email: true,
        phone: true,
        address: true,
      });

      setSubmitting(false);
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
          <div class="user-name">
            {infoEdit().username ? (
              <h4>
                {firstName() && lastName()
                  ? firstName() + " " + lastName()
                  : "Unknown"}
              </h4>
            ) : (
              <div>
                <input
                  value={firstName() ? firstName() : ""}
                  onInput={(e) => setFirstName(e.currentTarget.value)}
                  hidden={infoEdit().username}
                />{" "}
                <input
                  value={lastName() ? lastName() : ""}
                  onInput={(e) => setLastName(e.currentTarget.value)}
                  hidden={infoEdit().username}
                />
              </div>
            )}
            <button
              onClick={() =>
                setInfoEdit({ ...infoEdit(), username: !infoEdit().username })
              }
            >
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
          <div class="user-edit">
            <div>
              <img src={email_icon} height="20" width="20" alt="email" />
              <span>: &nbsp</span>
              {infoEdit().email ? (
                <span>{email() ? email() : "Unknown"}</span>
              ) : (
                <input
                  value={email() ? email() : ""}
                  onInput={(e) => setEmail(e.currentTarget.value)}
                  hidden={infoEdit().email}
                />
              )}
            </div>
            <button
              onClick={() =>
                setInfoEdit({ ...infoEdit(), email: !infoEdit().email })
              }
            >
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
          <div class="user-edit">
            <div>
              <img src={phone_icon} height="20" width="20" alt="phone" />
              <span>: &nbsp</span>
              {infoEdit().phone ? (
                <span>{phone() ? phone() : "Unknown"}</span>
              ) : (
                <input
                  value={phone() ? phone() : ""}
                  onInput={(e) => setPhone(e.currentTarget.value)}
                  hidden={infoEdit().phone}
                />
              )}
            </div>
            <button
              onClick={() =>
                setInfoEdit({ ...infoEdit(), phone: !infoEdit().phone })
              }
            >
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
          <div class="user-edit">
            <div>
              <img src={address_icon} height="20" width="20" alt="address" />
              <span>: &nbsp</span>
              {infoEdit().address ? (
                <span>{address() ? address() : "Unknown"}</span>
              ) : (
                <input
                  value={address() ? address() : ""}
                  onInput={(e) => setAddress(e.currentTarget.value)}
                  hidden={infoEdit().address}
                />
              )}
            </div>
            <button
              onClick={() =>
                setInfoEdit({ ...infoEdit(), address: !infoEdit().address })
              }
            >
              <img src={edit} height="20" width="20" alt="edit" />
            </button>
          </div>
        </div>

        <div class="edit-button">
          {submitting() ? (
            <span>waiting</span>
          ) : (
            <button onClick={handleSubmit}>Save</button>
          )}
        </div>
      </div>
    </div>
  );
}

export default Setting;
