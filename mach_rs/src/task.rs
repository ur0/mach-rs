// Copyright (C) 2021 Umang Raghuvanshi.
// This file is a part of the mach-rs project and is licensed under the MIT license.
// See the LICENSE file in the project root for details.

use mach_sys::mach_task_self_;

use crate::port::Port;

pub struct Task(pub(crate) Port);

impl Task {
    pub fn current() -> Task {
        let mach_task_self = unsafe { mach_task_self_ };

        Task(Port {
            name: mach_task_self,
            task: mach_task_self,
        })
    }
}

impl AsRef<Port> for Task {
    fn as_ref(&self) -> &Port {
        &self.0
    }
}

impl AsMut<Port> for Task {
    fn as_mut(&mut self) -> &mut Port {
        &mut self.0
    }
}
