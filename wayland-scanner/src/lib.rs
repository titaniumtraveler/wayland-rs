//! Wayland protocol code-generation machinnery
//!
//! This crate provides procedural macros for generating the rust code associated with a
//! Wayland XML protocol specification, for use with the `wayland-client`, `wayland-server`
//! and `wayland-backend` crates.
//!
//! Before trying to use this crate, you may check if the protocol extension you want to use
//! is not already exposed in the `wayland-protocols` crate.
//!
//! ## Example usage
//!
//! Below is a template for generating the code for a custom protocol client-side. Server-side
//! is identical, just replacing `client` by `server`. The path to the XML file is relative to the
//! crate root.
//!
//! ```rust,ignore
//! // Generate the bindings in their own module
//! pub mod my_protocol {
//!     use wayland_client;
//!     // import objects from the core protocol if needed
//!     use wayland_client::protocol::*;
//!
//!     // This module hosts a low-level representation of the protocol objects
//!     // you will not need to interact with it yourself, but the code generated
//!     // by the generate_client_code! macro will use it
//!     pub mod __interfaces {
//!         // import the interfaces from the core protocol if needed
//!         use wayland_client::protocol::__interfaces::*;
//!         wayland_scanner::generate_interfaces!("./path/to/the/protocol.xml");
//!     }
//!     use self::__interfaces::*;
//!
//!     // This macro generates the actual types that represent the wayland objects of
//!     // your custom protocol
//!     wayland_scanner::generate_client_code!("./path/to/the/protocol.xml");
//! }
//! ```

/// Proc-macro for generating low-level interfaces associated with an XML specification
#[proc_macro]
pub fn generate_interfaces(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wayland_scanner_lib::generate_interfaces(stream.into()).into()
}

/// Proc-macro for generating client-side API associated with an XML specification
#[proc_macro]
pub fn generate_client_code(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wayland_scanner_lib::generate_client_code(stream.into()).into()
}

/// Proc-macro for generating server-side API associated with an XML specification
#[proc_macro]
pub fn generate_server_code(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wayland_scanner_lib::generate_server_code(stream.into()).into()
}
