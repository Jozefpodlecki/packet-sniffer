use std::time::Duration;

use tokio::{sync::mpsc::{self, Receiver}, time::sleep};

#[unsafe(no_mangle)]
pub extern "C" fn start_capture() -> *mut Receiver<i32> {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        let mut index = 0;

        loop {
            sleep(Duration::from_secs(2)).await;
            tx.send(index).await.unwrap();
            index += 1;
            index = index % 100;
        }
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