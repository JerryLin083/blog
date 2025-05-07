import solidjs from "../assets/icons-solidjs.svg";
import axum from "../assets/icons-tokio-axum.svg";

import "./footer.css";

function Footer() {
  return (
    <footer class="footer">
      <div class="footer_container">
        <span>Made with</span>
        <img src={solidjs} width="32" height="32" alt="solid_logo" />
        <img src={axum} width="32" height="32" alt="axum_log" />
      </div>
    </footer>
  );
}

export default Footer;
