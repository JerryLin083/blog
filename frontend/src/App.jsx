import { Route, Router } from "@solidjs/router";

import "./App.css";
import Layout from "./components/layout";
import Home from "./pages/home";
import Posts from "./pages/posts";
import Users from "./pages/users";
import NotFound from "./pages/not_found";
import Post from "./pages/post";
import User from "./pages/user";
import SignUp from "./pages/signup";
import Login from "./pages/login";

function App() {
  return (
    <Router root={Layout}>
      <Route path="/" component={Home} />
      <Route path="/posts" component={Posts} />
      <Route path="posts/:id" component={Post} />
      <Route path="/users" component={Users} />
      <Route path="users/:id" component={User} />
      <Route path="*404" component={NotFound} />
      <Route path="/signup" component={SignUp} />
      <Route path="/login" component={Login} />
    </Router>
  );
}

export default App;
