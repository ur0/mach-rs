// Copyright (C) 2021 Umang Raghuvanshi.
// This file is a part of the mach-rs project and is licensed under the MIT license.
// See the LICENSE file in the project root for details.

use crate::task::Task;

use crate::error::MachError;
use mach_sys::{
    ipc_space_t, mach_port_allocate, mach_port_deallocate, mach_port_insert_right, mach_port_t,
    mach_task_self_, KERN_SUCCESS,
};

const MACH_PORT_RIGHT_SEND: u32 = 0;
const MACH_PORT_RIGHT_RECEIVE: u32 = 1;
const MACH_PORT_RIGHT_SEND_ONCE: u32 = 2;
const MACH_PORT_RIGHT_PORT_SET: u32 = 3;
const MACH_PORT_RIGHT_DEAD_NAME: u32 = 4;

/// A right that can be gained for a port.
#[repr(u32)]
pub enum Right {
    Send = MACH_PORT_RIGHT_SEND,
    Receive = MACH_PORT_RIGHT_RECEIVE,
    SendOnce = MACH_PORT_RIGHT_SEND_ONCE,
    PortSet = MACH_PORT_RIGHT_PORT_SET,
    DeadName = MACH_PORT_RIGHT_DEAD_NAME,
}

const MACH_MSG_TYPE_MOVE_RECEIVE: u32 = 16;
const MACH_MSG_TYPE_MOVE_SEND: u32 = 17;
const MACH_MSG_TYPE_MOVE_SEND_ONCE: u32 = 18;
const MACH_MSG_TYPE_COPY_SEND: u32 = 19;
const MACH_MSG_TYPE_MAKE_SEND: u32 = 20;
const MACH_MSG_TYPE_MAKE_SEND_ONCE: u32 = 21;
const MACH_MSG_TYPE_COPY_RECEIVE: u32 = 22;
const MACH_MSG_TYPE_DISPOSE_RECEIVE: u32 = 24;
const MACH_MSG_TYPE_DISPOSE_SEND: u32 = 25;
const MACH_MSG_TYPE_DISPOSE_SEND_ONCE: u32 = 26;

/// A right that can be inserted into a port.
#[repr(u32)]
pub enum InsertableRight {
    MoveReceive = MACH_MSG_TYPE_MOVE_RECEIVE,
    MoveSend = MACH_MSG_TYPE_MOVE_SEND,
    CopySend = MACH_MSG_TYPE_COPY_SEND,
    MakeSend = MACH_MSG_TYPE_MAKE_SEND,
    MoveSendOnce = MACH_MSG_TYPE_MOVE_SEND_ONCE,
    MakeSendOnce = MACH_MSG_TYPE_MAKE_SEND_ONCE,
    CopyReceive = MACH_MSG_TYPE_COPY_RECEIVE,
    DisposeReceive = MACH_MSG_TYPE_DISPOSE_RECEIVE,
    DisposeSend = MACH_MSG_TYPE_DISPOSE_SEND,
    DisposeSendOnce = MACH_MSG_TYPE_DISPOSE_SEND_ONCE,
}

/// Represents a Mach port which is deallocated when dropped.
pub struct Port {
    pub(crate) name: mach_port_t,
    pub(crate) task: ipc_space_t,
}

impl Port {
    fn allocate(task: &Task, right: Right) -> Result<Self, MachError> {
        let mut port = Port {
            name: 0,
            task: task.0.name,
        };
        Ok(Result::from(MachError::from(unsafe {
            mach_port_allocate(task.0.name, right as u32, &mut port.name)
        }))
        .map(|_| port)?)
    }

    fn insert_right(&self, name: &Port, right_type: InsertableRight) -> Result<(), MachError> {
        Ok(Result::from(MachError::from(unsafe {
            mach_port_insert_right(name.task, name.name, self.name, right_type as u32)
        }))?)
    }
}

impl Drop for Port {
    #[inline]
    fn drop(&mut self) {
        debug_assert_ne!(self.name, 0);

        if unsafe { self.name != mach_task_self_ && self.task != mach_task_self_ } {
            let did_deallocate = unsafe { mach_port_deallocate(self.task, self.name) } as u32;
            debug_assert_eq!(did_deallocate, KERN_SUCCESS);
        }
    }
}
/// Represents a Mach port for which the task owns both a send right and a receive right.
pub struct SendReceiveRight(pub(crate) Port);
impl AsRef<Port> for SendReceiveRight {
    fn as_ref(&self) -> &Port {
        &self.0
    }
}

impl AsMut<Port> for SendReceiveRight {
    fn as_mut(&mut self) -> &mut Port {
        &mut self.0
    }
}

impl Into<SendRight> for SendReceiveRight {
    fn into(self) -> SendRight {
        SendRight(self.0)
    }
}

impl Into<ReceiveRight> for SendReceiveRight {
    fn into(self) -> ReceiveRight {
        ReceiveRight(self.0)
    }
}

/// Represents a Mach port for which the task owns a send right.
pub struct SendRight(pub(crate) Port);
impl AsRef<Port> for SendRight {
    fn as_ref(&self) -> &Port {
        &self.0
    }
}

impl AsMut<Port> for SendRight {
    fn as_mut(&mut self) -> &mut Port {
        &mut self.0
    }
}

/// Represents a Mach port for which the task owns a receive right.
pub struct ReceiveRight(pub(crate) Port);

impl ReceiveRight {
    pub fn new_in_task(task: &Task) -> Result<Self, MachError> {
        Ok(ReceiveRight(Port::allocate(task, Right::Receive)?))
    }

    pub fn new() -> Result<Self, MachError> {
        Self::new_in_task(&Task::current())
    }

    pub fn insert_send_right(self) -> Result<SendReceiveRight, MachError> {
        self.0.insert_right(&self.0, InsertableRight::MakeSend)?;
        Ok(SendReceiveRight(self.0))
    }

    pub fn insert_send_right_into_port<T: AsRef<Port>>(&self, port: &T) -> Result<(), MachError> {
        self.0
            .insert_right(port.as_ref(), InsertableRight::MakeSend)
    }
}

impl AsRef<Port> for ReceiveRight {
    fn as_ref(&self) -> &Port {
        &self.0
    }
}

impl AsMut<Port> for ReceiveRight {
    fn as_mut(&mut self) -> &mut Port {
        &mut self.0
    }
}

/// Represents a Mach port for which the task owns a receive right.
pub struct DeadName(pub(crate) Port);

impl DeadName {
    pub fn new_in_task(task: &Task) -> Result<Self, MachError> {
        Ok(DeadName(Port::allocate(task, Right::DeadName)?))
    }

    pub fn new() -> Result<Self, MachError> {
        Self::new_in_task(&Task::current())
    }
}

impl AsRef<Port> for DeadName {
    fn as_ref(&self) -> &Port {
        &self.0
    }
}

impl AsMut<Port> for DeadName {
    fn as_mut(&mut self) -> &mut Port {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::port::*;

    #[test]
    fn allocation_test() {
        let recv = ReceiveRight::new();
        assert!(recv.is_ok());
    }

    #[test]
    fn insert_sright_test() {
        let recv = ReceiveRight::new().unwrap();
        let send_recv = recv.insert_send_right();
        assert_eq!(send_recv.is_ok(), true);
    }

    #[test]
    fn dead_name_creation_test() {
        let dead = DeadName::new();
        assert_eq!(dead.is_ok(), true);
    }
}
