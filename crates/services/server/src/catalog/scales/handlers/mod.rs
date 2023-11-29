mod delete_scale;
mod get_all_scales;
mod get_scale_by_id;
mod post_scale;
mod put_scale;

pub use delete_scale::handle as delete_scale;
pub use get_all_scales::handle as get_all_scales;
pub use get_scale_by_id::handle as get_scale_by_id;
pub use post_scale::handle as post_scale;
pub use put_scale::handle as put_scale;
