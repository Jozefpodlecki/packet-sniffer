use std::sync::{mpsc::Receiver, Arc};

use libloading::{Library, Symbol};
use shared::models::Packet;

type StartCaptureFn = unsafe extern "C" fn() -> *mut Receiver<Packet>;

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
        let start_capture: Symbol<StartCaptureFn> = unsafe { self.lib.get(b"start_capture").unwrap() };

        self.receiver = unsafe { Some(start_capture()) };
    }

    pub fn recv(&self) -> Option<Packet> {
        if let Some(rx_ptr) = self.receiver {

            if rx_ptr.is_null() {
                return None;
            }

            let rx = unsafe { &*rx_ptr };

            return rx.recv().ok()
        }

        None
    }
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        if let Some(rx_ptr) = self.receiver.take() {
            if !rx_ptr.is_null() {
                unsafe { drop(Box::from_raw(rx_ptr)) }
            }
        }
    }
}
