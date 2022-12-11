// Take a look at the license at the top of the repository in the LICENSE file.

mod action_group;
mod action_map;
mod application;
mod async_initable;
mod initable;
mod input_stream;
mod io_stream;
mod list_model;
mod output_stream;
mod seekable;

pub use self::application::ArgumentList;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::{
        action_group::{ActionGroupImpl, ActionGroupImplExt},
        action_map::{ActionMapImpl, ActionMapImplExt},
        application::{ApplicationImpl, ApplicationImplExt},
        async_initable::{AsyncInitableImpl, AsyncInitableImplExt},
        initable::{InitableImpl, InitableImplExt},
        input_stream::{InputStreamImpl, InputStreamImplExt},
        io_stream::{IOStreamImpl, IOStreamImplExt},
        list_model::{ListModelImpl, ListModelImplExt},
        output_stream::{OutputStreamImpl, OutputStreamImplExt},
        seekable::{SeekableImpl, SeekableImplExt},
    };
}
