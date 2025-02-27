use std::{thread, time::Duration};

use tokio::{runtime::Runtime, sync::mpsc::{self, Receiver}, time::sleep};

#[repr(C)] 
pub enum Packet {
    Damage {
        object_id: i32,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn start_capture() -> *mut std::sync::mpsc::Receiver<Packet> {
    let (tx, rx) = std::sync::mpsc::channel::<Packet>();

    std::thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(2));
            let payload = Packet::Damage {
                object_id: 1,
            };
            tx.send(payload).unwrap();
        }
    });

    Box::into_raw(Box::new(rx))
}

#[unsafe(no_mangle)]
pub extern "C" fn start_capture_tokio_mpsc() -> *mut Receiver<Packet> {
    let (tx, rx) = mpsc::channel::<Packet>(10);

    std::thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {

            loop {
                sleep(Duration::from_secs(2)).await;
                let payload = Packet::Damage {
                    object_id: 1,
                };
                tx.send(payload).await.unwrap();

            }
        });
    });

    Box::into_raw(Box::new(rx))
}

// #[unsafe(no_mangle)]
// pub extern "C" fn create_struct() -> *mut MyStruct {
//     let name = CString::new("Hello from DLL").unwrap();
    
//     let s = Box::new(MyStruct {
//         value: 42,
//         name: name.into_raw(), // Pass ownership of the string
//     });

//     Box::into_raw(s) // Pass ownership of the struct
// }