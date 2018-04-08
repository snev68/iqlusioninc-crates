//! RedHat Package Manager (RPM) library low-level FFI bindings
//! (automatically generated by bindgen)
//!
//! This crate isn't intended to be used directly, but instead provides the
//! low-level binding which is used by the idiomatic rpmlib crate.

#![allow(unknown_lints, clippy)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// WARNING: potentially unsafe hax ahead!
//
// `struct timex` (defined manually below) is problematic because the bindgen
// version includes a 352-bit bitfield type for `_bitfield_1`:
//
//     __BindgenBitfieldUnit<[u8; 44], u8>
//
// bindgen bounds the traits of the generic storage type for its bitfield units
// on `AsRef` and `AsMut`, which do not have blanket impls for `[T; N]` where
// N > 32.
//
// To work around the problem, we blacklist the type in `build.rs`, and include
// a (hopefully) equivalent one which actually compiles, namely by removing the
// `__BindgenBitfieldUnit` wraper around `[u8; 44]`, which should hopefully
// (???)
// result in an equivalent-sized type.
//
// `struct timex` is used only one place in the generated binding: as a
// parameter to `clock_adjtime`:
//
//     extern "C" {
//         pub fn clock_adjtime(
//             __clock_id: __clockid_t,
//            __utx: *mut timex
//         ) -> ::std::os::raw::c_int;
//     }
//
// If we could blacklist `clock_adjtime` from the binding, we wouldn't need this
// manually tweaked copy-and-paste definition of `struct timex`.
//
// Ideally we should probably switch to using `whitelisted_functions` and only
// include functions we actually intend to bind in the binding.
//
// Context:
// - https://github.com/iqlusion-io/crates/issues/11
// - https://github.com/iqlusion-io/crates/issues/12
#[repr(C)]
pub struct timex {
    pub modes: ::std::os::raw::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: ::std::os::raw::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: ::std::os::raw::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: [u8; 44], // Was: `__BindgenBitfieldUnit<[u8; 44], u8>`
}
