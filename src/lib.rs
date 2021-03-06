//! # Rpc-lib
//!
//! Rpc-lib is a crate to compile an RPC-Definition in the ONC-RPC format ([`RFC 5531`]) into Rust-code. It comes
//! with the necessary network-code to execute the RPC-Calls and to serialize and deserialize the
//! data.
//!
//! [`RFC 5531`]: https://datatracker.ietf.org/doc/html/rfc5531
//!
//! # Example
//! 
//! Creates a connection to 127.0.0.1, makes an Rpc-Call and prints the result.
//! ```rust
//! extern crate rpc_lib;
//! use rpc_lib::include_rpcl;
//! 
//! #[include_rpcl("my_rpcl_file.x")]
//! struct RPCStruct;
//! 
//! fn main() {
//!     let mut rpc = RPCStruct::new("127.0.0.1").expect("Can't connect to server");
//!     let result = rpc.MY_RPC_PROCEDURE(&1, &2).expect("Rpc call failed");
//!     println!("MY_RPC_PROCEDURE returned: {}", result);
//! }
//! ```
extern crate quote;
extern crate rpc_lib_impl;

mod rpc_struct;

/// Reads file and generates Rustcode according to contents
///
/// # Examples
/// Reads `my_file.x` and adds associated functions to `MyStruct` according to procedure-definitions in
/// `my_file.x` 
/// ```
/// #[include_rpcl("my_file.x")]
/// struct MyStruct;
/// ```
pub use rpc_lib_impl::include_rpcl;

pub use rpc_struct::clnt_create;
pub use rpc_struct::rpc_call;
pub use rpc_struct::RpcClient;

pub use rpc_struct::xdr::*;
