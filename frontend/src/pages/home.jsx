import "./home.css";
import { A } from "@solidjs/router";

import rat from "../assets/icons-rat.svg";
import racoon from "../assets/icons-racoon.svg";
import turtle from "../assets/icons-turtle.svg";
import dolphin from "../assets/icons-dolphin.svg";
import shark from "../assets/icons-shark.svg";

function Home() {
  return (
    <div class="pixel-container">
      <h1 class="pixel-title">Welcome</h1>
      <section class="latest-posts">
        <p>
          <strong>Hey there, awesome internet wanderer!</strong>
        </p>
        <p>
          <strong>Welcome</strong> to our blog your new favorite spot to{" "}
          <em>rant, rave, and ramble!</em>
        </p>
        <p>
          Feeling chatty? Join us and let those thoughts <em>fly!</em>
        </p>
        <p>Share your mood swings, your latest discoveries,</p>
        <p>
          or just whateverhilarious thing happened to you <em>today.</em>
        </p>
        <p>
          <strong>Everyone's invited</strong> to join the fun and unleash their
          inner storyteller.
        </p>
        <p>
          So{" "}
          <A href="/signup">
            <strong class="signup">SIGNUP</strong>
          </A>{" "}
          , <strong>speak up</strong>, and let's get this blogging party{" "}
          <em>started!</em>
        </p>
      </section>

      <div class="animals_area">
        <img src={rat} alt="rat" height="64" width="64" />
        <img src={shark} alt="shark" height="64" width="64" />
        <img src={racoon} alt="racoon" height="64" width="64" />
        <img src={turtle} alt="turtle" height="64" width="64" />
        <img src={dolphin} alt="dolphin" height="64" width="64" />
      </div>

      <div class="button_area">
        <A href="/posts?page=1">Wonder around</A>
      </div>
    </div>
  );
}

export default Home;
