import "./layout.css";

function Layout(props) {
  return (
    <>
      <main class="container">{props.children}</main>
    </>
  );
}

export default Layout;
