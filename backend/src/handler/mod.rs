mod api;

//get
pub(crate) use api::posts;
pub(crate) use api::the_post;
pub(crate) use api::the_user;
pub(crate) use api::users;

//post
pub(crate) use api::create_post;
pub(crate) use api::create_user;

//patch
pub(crate) use api::edit_post;
pub(crate) use api::patch_user;
pub(crate) use api::publish_post;

//delete
pub(crate) use api::delete_post;
pub(crate) use api::delete_user;

mod static_file;
//get
pub(crate) use static_file::home;
pub(crate) use static_file::not_found;
