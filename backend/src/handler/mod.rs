mod api;

//get
pub(crate) use api::account;
pub(crate) use api::auth;
pub(crate) use api::posts;
pub(crate) use api::the_post;
pub(crate) use api::the_user;
pub(crate) use api::users;

//post
pub(crate) use api::create_post;
pub(crate) use api::login;
pub(crate) use api::logout;
pub(crate) use api::signup;

//patch
pub(crate) use api::edit_post;
pub(crate) use api::patch_user;
pub(crate) use api::publish_post;

//delete
pub(crate) use api::delete_post;
pub(crate) use api::delete_user;

mod static_file;

//static get
pub(crate) use static_file::home;
