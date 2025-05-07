import Footer from "./footer";
import Header from "./header";
import "./layout.css";

function Layout(props) {
  return (
    <div class="layout">
      <Header />
      <main>{props.children}</main>
      <Footer />
    </div>
  );
}

export default Layout;
