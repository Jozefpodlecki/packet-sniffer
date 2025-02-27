use std::sync::{mpsc::Receiver, Arc};

use libloading::{Library, Symbol};

#[repr(C)]
#[derive(Debug)]
pub enum Packet {
    Damage {
        object_id: i32,
    }
}

pub struct Wrapper {
    lib: Library,
    receiver: Option<*mut Receiver<Packet>>, 
}

impl Wrapper {
    pub fn new() -> Self {
        let lib = unsafe { Library::new("sniffer_lib.dll").unwrap() };

        Self {
            lib,
            receiver: None
        }
    }

    pub fn start_capture(&mut self) {
        let start_capture: Symbol<unsafe extern "C" fn() -> *mut Receiver<Packet>> = unsafe { self.lib.get(b"start_capture").unwrap() };

        self.receiver = unsafe { Some(start_capture()) };
    }

    pub fn recv(&self) -> Option<Packet> {
        if let Some(rx_ptr) = self.receiver {
            let rx = unsafe { &*rx_ptr };

            return rx.recv().ok()
        }

        None
    }
}