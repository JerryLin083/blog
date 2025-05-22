<h1 align="center">My Rust Blog Project</h1>

<p align="center">A full-stack blog application built with Rust (Axum) for the backend and SolidJS for the frontend, powered by a PostgreSQL database.</p>

---

<h2>🚀 Project Overview</h2>

<p>This project is a complete blog platform, demonstrating a modern web application architecture. It features user authentication, post management, and a responsive single-page application (SPA) experience.</p>

---

<h2>📦 Database Schema (PostgreSQL)</h2>

<p>The backend interacts with a PostgreSQL database, structured with the following tables:</p>

<h3><code>rust_blog</code> Database</h3>

<h4>`accounts` Table</h4>
<ul>
  <li><code>account_id</code>: <code>SERIAL PRIMARY KEY</code></li>
  <li><code>account</code>: <code>VARCHAR(255)</code></li>
  <li><code>password</code>: <code>VARCHAR(255)</code></li>
</ul>

<h4>`users` Table</h4>
<ul>
  <li><code>user_id</code>: <code>SERIAL PRIMARY KEY</code></li>
  <li><code>account_id</code>: <code>INTEGER</code> (Foreign Key to <code>accounts.account_id</code>)</li>
  <li><code>first_name</code>: <code>VARCHAR(255)</code></li>
  <li><code>last_name</code>: <code>VARCHAR(255)</code></li>
  <li><code>email</code>: <code>VARCHAR(255)</code></li>
  <li><code>phone</code>: <code>VARCHAR(255)</code></li>
  <li><code>address</code>: <code>VARCHAR(255)</code></li>
</ul>

<h4>`posts` Table</h4>
<ul>
  <li><code>id</code>: <code>SERIAL PRIMARY KEY</code></li>
  <li><code>title</code>: <code>VARCHAR(255)</code></li>
  <li><code>content</code>: <code>VARCHAR(MAX)</code></li>
  <li><code>user_id</code>: <code>INT</code> (Foreign Key to <code>users.user_id</code>)</li>
  <li><code>create_at</code>: <code>TIMESTAMPTZ DEFAULT NOW()</code></li>
  <li><code>update_at</code>: <code>TIMESTAMPTZ DEFAULT NOW()</code></li>
  <li><code>published_at</code>: <code>TIMESTAMPTZ</code></li>
</ul>

---

<h2>⚙️ Backend (Axum)</h2>

<p>The backend is built with <strong>Axum</strong>, a web application framework for Rust. It handles all API requests, database interactions, and session management.</p>

<h3>Database Connection</h3>
<ul>
  <li><strong>Self-signed SSL:</strong> Secure database connections using self-signed SSL certificates.</li>
  <li><strong>Connection Pool:</strong> Utilizes a database connection pool to prevent resource exhaustion and blocking, ensuring efficient and scalable database access.</li>
  <li><strong>Session Management:</strong> Manages <code>session_id</code> with a <code>HashMap</code> and handles session expiration in background tasks using a <code>BinaryHeap</code> for efficient clean-up.</li>
</ul>

<h3>Router (API Endpoints)</h3>
<p>The router is configured with the connection pool and session manager as router state and extensions, making them accessible across handlers.</p>

<h4>GET Endpoints:</h4>
<ul>
  <li><code>/api/users</code>: Get all users.</li>
  <li><code>/api/users/{id}</code>: Get a specific user by ID.</li>
  <li><code>/api/posts</code>: Get all posts.</li>
  <li><code>/api/posts/{id}</code>: Get a specific post by ID.</li>
  <li><code>/api/posts/myposts</code>: Get posts by the authenticated user (<strong>authentication required</strong>).</li>
  <li><code>/api/account/{account}</code>: Check if an account exists.</li>
  <li><code>/api/logout</code>: Log out the current user (removes <code>session_id</code>).</li>
  <li><code>/api/auth</code>: Check the authentication state of the current <code>session_id</code>.</li>
  <li><code>/api/auth/user</code>: Get authenticated user information by <code>session_id</code>.</li>
</ul>

<h4>POST Endpoints:</h4>
<ul>
  <li><code>/api/signup</code>: Create a new user account and return a <code>session_id</code>.</li>
  <li><code>/api/login</code>: Log in an existing user and return a <code>session_id</code>.</li>
  <li><code>/api/posts</code>: Create a new post (<strong>authentication required</strong>).</li>
</ul>

<h4>PATCH Endpoints:</h4>
<ul>
  <li><code>/api/user</code>: Update user information (<strong>authentication required</strong>).</li>
  <li><code>/api/posts/edit/{id}</code>: Edit a specific post (<strong>authentication required</strong>).</li>
  <li><code>/api/posts/published/{id}</code>: Change the published status of a post (<strong>authentication required</strong>).</li>
</ul>

<h4>DELETE Endpoints:</h4>
<ul>
  <li><code>/api/users/{id}</code>: Delete a user by ID.</li>
  <li><code>/api/posts/{id}</code>: Delete a post by ID.</li>
</ul>

<h3>Static File Serving:</h3>
<ul>
  <li><code>/</code>: Serves the <code>index.html</code> (home page).</li>
  <li><code>/assets</code>: Serves static assets (e.g., <code>.svg</code>, <code>.jpg</code>, <code>.js</code>, <code>.css</code> files) from the <code>static/assets</code> directory.</li>
</ul>

---

<h2>💻 Frontend (SolidJS)</h2>

<p>The frontend is a Single-Page Application (SPA) built with <strong>SolidJS</strong>, offering a fast and reactive user experience.</p>

<h3>HTML Security Headers:</h3>
<ul>
  <li><code>script-src 'self'</code>: Only allows scripts from the same origin.</li>
  <li><code>object-src 'none'</code>: Prevents the embedding of external objects.</li>
  <li><code>base-url 'self'</code>: Sets the base URL to the document's origin.</li>
</ul>

<h3>Router (SPA Routes):</h3>
<p>Handles client-side routing for seamless navigation without full page reloads.</p>
<ul>
  <li><code>/</code>: Home Page (Layout)</li>
  <li><code>/posts</code>: Displays a list of blog posts.</li>
  <li><code>/posts/:id</code>: Displays a single blog post.</li>
  <li><code>/posts/new</code>: Allows authenticated users to create a new post (<strong>authentication required</strong>).</li>
  <li><code>/posts/myposts</code>: Displays posts created by the authenticated user (<strong>authentication required</strong>).</li>
  <li><code>/users</code>: Displays a list of users.</li>
  <li><code>/users/:id</code>: Displays a single user's profile.</li>
  <li><code>/signup</code>: User registration page.</li>
  <li><code>/login</code>: User login page.</li>
  <li><code>*404</code>: Catch-all route for unfound pages.</li>
</ul>

---

<h2>🌟 Features</h2>
<ul>
  <li>User Authentication (Signup, Login, Logout)</li>
  <li>Session Management with Expiration</li>
  <li>CRUD Operations for Users and Posts</li>
  <li>Secure Backend with SSL and Connection Pooling</li>
  <li>Reactive Frontend with SolidJS</li>
  <li>Static File Serving</li>
</ul>
