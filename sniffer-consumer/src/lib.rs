use std::sync::{mpsc::Receiver, Arc};

use libloading::{Library, Symbol};

pub enum PacketType {

}

pub struct Payload {
    pub kind: PacketType,
    pub data: Vec<u8>
}

pub struct Wrapper {
    lib: Library,
    receiver: Option<*mut Receiver<Payload>>, 
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
        let start_capture: Symbol<unsafe extern "C" fn() -> *mut Receiver<Payload>> = unsafe { self.lib.get(b"start_capture").unwrap() };

        self.receiver = unsafe { Some(start_capture()) };
    }

    pub fn recv(&self) {
        if let Some(rx_ptr) = self.receiver {
            let rx = unsafe { &*rx_ptr };

            let test = rx.recv().unwrap();
        }
    }
}