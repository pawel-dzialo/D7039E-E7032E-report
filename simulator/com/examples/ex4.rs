//use core::num::dec2flt::float;
use std::io::{Write};
use std::{fs};
use std::time::Duration;
//use std::thread;
use serialport::SerialPort;
use serialport::{DataBits, StopBits};
//use std::time::SystemTime;

#[allow(unused_parens)]
fn main() {
// let mut port = serialport::new("/dev/ttyACM0", 115_200).open_native().expect("Failed to open port");
    //let output = "This is a test. This is only a test.".as_bytes();
    
    let file_path = "/home/pawel/genfile/genfile/output";
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");


    let mut array: [i16; 8] = [0;8];
    let mut buffer: [u8;16] = [0;16];
    //let mut v:Vec<[f32;16]> = Vec::new();
    let mut i = 0;
    let mut j:i64 = 0;
    let mut s:String = "".to_string();
    let builder = serialport::new("/dev/ttyACM2", 115_200)
    .stop_bits(StopBits::One)
    .data_bits(DataBits::Eight);
    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyACM2", e);
        ::std::process::exit(1);
    });
    for character in contents.chars(){
        if(character == '['){
            continue;
        }
        else if(character == ','){
            array[i] = s.parse().unwrap();
            i+=1;
            s = "".to_string();
        }
        else if(character == ']'){
            let mut iter = 0;
            for int in array{

                let temp = int.to_be_bytes();
                buffer[iter] = temp[0];
                buffer[iter+1] = temp[1];
                iter +=2;
            }
            port = sendData(buffer,port);
            j+=1;
            //v.push(array);           
            i=0;
            std::thread::sleep(Duration::from_micros(4_000)); //sleep rest of sample period....
            if(j>20000){
                print!("{:?}",buffer);
                break;
            }
        }
        else{
            s.push_str(&character.to_string());
        }
    }
// ---------------------------------


/*



    //let mut arr:[f32;8] = [0.0001,0.0002,0.0003,0.0004,0.0005,0.0006,0.0007,0.0008];
    let mut arr:[u8;16] = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0xff,0x0c,0x0d,0x0f,0x10];
    let mut buffer:[u8;32] = [0;32];
    let mut i = 0;
   /* for val in arr{
        let mut j = 0;
        let temp = arr[i].to_be_bytes();
        while j<4{
            buffer[(i*4)+j]=temp[j];
            j+=1;
        }
        i+=1;
    }*/
    //println!("{:?}",buffer);
    let mut i = 0;
    let builder = serialport::new("/dev/ttyACM2", 115_200)
    .stop_bits(StopBits::One)
    .data_bits(DataBits::Eight);
    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyACM2", e);
        ::std::process::exit(1);
    });
    let now = SystemTime::now();
  // while (i<10240) {
       // if i%1000 == 0{
           // arr[0]+=1;
       // }
        port = sendData(arr,port);
        //println!("Done dumping");
        
        i+=1;
   // }
   println!("{:?}",arr);
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
    print!("{}",arr[0]);
  */  
 }


  //  port.write(b"a").expect("Write failed!");   

  #[allow(non_snake_case, unused_parens)]
fn sendData(array:[u8;16], mut port:Box<dyn SerialPort>) -> Box<dyn SerialPort>{
    
        match(port.write(&array)){
            Ok(_) => {
            }
            Err(e) => eprintln!("{:?}", e),
        }      
      //  std::thread::sleep(Duration::from_micros(500)); //16 samples at 256Hz gives max 244 micros of sleep between sending each sample. 
    return port;
}


//1846