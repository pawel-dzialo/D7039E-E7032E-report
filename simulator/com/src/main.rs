use std::io::{self, Write};
use std::fs;
use std::time::Duration;
use std::thread;
use serialport::SerialPort;
use serialport::{DataBits, StopBits};

#[allow(unused_parens)]
fn main() {
   // let mut port = serialport::new("/dev/ttyACM0", 115_200).open_native().expect("Failed to open port");
    //let output = "This is a test. This is only a test.".as_bytes();
    let file_path = "/home/pawel/genfile/output";
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");


    let mut array: [f32; 16] = [0.0;16];
    let mut v:Vec<[f32;16]> = Vec::new();
    let mut i = 0;
    let mut j:i64 = 0;
    let mut s:String = "".to_string();
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
            j+=1;
            sendData(array,j);
            //v.push(array);           
            i=0;
           // thread::sleep(Duration::from_secs(1/256));
            if(j>1000){
                break;
            }

        }
        else{
            s.push_str(&character.to_string());
        }
    }
    for arr in v{
        for val in arr{
            println!("{}", val);
        }
    }

  //  port.write(b"a").expect("Write failed!");   
}

fn sendData(array:[f32;16],j:i64){
    let builder = serialport::new("/dev/ttyACM0", 115_200)
    .stop_bits(StopBits::One)
    .data_bits(DataBits::Eight);
    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyACM1", e);
        ::std::process::exit(1);
    });
    for val in array{
        let mut x = val.to_be_bytes();
        match(port.write(&x)){
            Ok(_) => {
            }
            Err(e) => eprintln!("{:?}", e),
        }      
        std::thread::sleep(Duration::from_micros(244)); //256Hz
    }
}


//1846