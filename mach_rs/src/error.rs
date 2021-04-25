// Copyright (C) 2021 Umang Raghuvanshi.
// This file is a part of the mach-rs project and is licensed under the MIT license.
// See the LICENSE file in the project root for details.

use mach_sys::{KERN_INVALID_RIGHT, KERN_NO_SPACE, KERN_RIGHT_EXISTS, KERN_SUCCESS};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachError {
    #[error("No error")]
    Success,
    #[error("There was no room in task's IPC name space for another right.")]
    NoSpace,
    #[error("The name denotes a right, but not an appropriate right.")]
    InvalidRight,
    #[error("The task already has send or receive rights for the port under another name.")]
    RightExists,
    #[error("Unknown error")]
    Unknown(u32),
}

impl From<u32> for MachError {
    fn from(e: u32) -> MachError {
        match e {
            KERN_SUCCESS => MachError::Success,
            KERN_NO_SPACE => MachError::NoSpace,
            KERN_INVALID_RIGHT => MachError::InvalidRight,
            KERN_RIGHT_EXISTS => MachError::RightExists,
            unknown => MachError::Unknown(unknown),
        }
    }
}

impl From<i32> for MachError {
    fn from(e: i32) -> Self {
        MachError::from(e as u32)
    }
}

impl From<MachError> for Result<(), MachError> {
    fn from(e: MachError) -> Self {
        match e {
            MachError::Success => Ok(()),
            err => Err(err),
        }
    }
}
