// Copyright (C) 2021 Umang Raghuvanshi.
// This file is a part of the mach-rs project and is licensed under the MIT license.
// See the LICENSE file in the project root for details.

//! Bindgen generated bindings to Mach APIs.

#![allow(non_camel_case_types)]
#![allow(safe_packed_borrows)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(not(feature = "bindgen"))]
include!("prebuilt.rs");

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    pub use crate::*;

    #[test]
    fn port_creation_destruction_test() {
        let mut port: mach_port_name_t = 0;
        let did_create = unsafe { mach_port_allocate(mach_task_self_, 1, &mut port) }; // MACH_PORT_RIGHT_RECEIVE
        assert_eq!(did_create as u32, KERN_SUCCESS);

        let did_destroy = unsafe { mach_port_destroy(mach_task_self_, port) };
        assert_eq!(did_destroy as u32, KERN_SUCCESS);
    }
}
