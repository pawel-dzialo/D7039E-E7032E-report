use std::io::{self, Write};
use std::fs;
use std::time::Duration;
use std::thread;
use serialport::SerialPort;
use serialport::{DataBits, StopBits};
use std::time::SystemTime;

#[allow(unused_parens)]
fn main() {
    let arr:[u16;16] = [0x0000,0x0002,0x0004,0x0006,0x0008,0x000A,0x000C,0x000F,0x1000,0x1000,0x3000,0x5000,0x7000,0x9000,0xB000,0xD000];
    let mut i = 0;
    let builder = serialport::new("/dev/ttyACM2", 115_200)
    .stop_bits(StopBits::One)
    .data_bits(DataBits::Eight);
    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyACM2", e);
        ::std::process::exit(1);
    });
    let now = SystemTime::now();
    //while (i<256) {
        port = sendData(arr,0,port);
       // std::thread::sleep(Duration::from_micros(1_000_000)); //sleep rest of sample period....
        i+=1;
    //}
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
 }


  //  port.write(b"a").expect("Write failed!");   


fn sendData(array:[u16;16],j:i64, mut port:Box<dyn SerialPort>) -> Box<dyn SerialPort>{
    for val in array{
        let mut x = val.to_be_bytes();
        match(port.write(&x)){
            Ok(_) => {
            }
            Err(e) => eprintln!("{:?}", e),
        }      
        std::thread::sleep(Duration::from_micros(100_000)); //16 samples at 256Hz gives max 244 micros of sleep between sending each sample. 
    }
    return port;
}


//1846