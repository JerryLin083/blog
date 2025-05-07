import solidjs from "../assets/icons-solidjs.svg";
import axum from "../assets/icons-tokio-axum.svg";

import "./footer.css";

function Footer() {
  return (
    <footer class="footer">
      <div class="footer_container">
        <span>Made with</span>
        <a href="https://www.solidjs.com/" target="_blank">
          <img src={solidjs} width="32" height="32" alt="solid_logo" />
        </a>

        <a href="https://github.com/tokio-rs/axum" target="_blank">
          <img src={axum} width="32" height="32" alt="axum_log" />
        </a>
      </div>
    </footer>
  );
}

export default Footer;
