use std::io;
use std::io::{Read, stdout, Write};
use std::str;

use clap::error::ContextValue::String;
use freertos_rust::*;
use rand::Rng;
use serialport::{available_ports, SerialPort, SerialPortType};

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;


fn main() {
    let x = Box::new(30);
    println!("Boxed int '{}' (allocator test)", x);

    unsafe {
        FREERTOS_HOOKS.set_on_assert(|| { println!("Assert hook called") });
    }

    //println!("Calling assert ...");
    //FreeRtosUtils::invoke_assert();

    let port_name = "COM3";
    let mut port = serialport::new(port_name, 9600)
        .open()
        .expect("Failed to open serial port");

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");
    println!("Starting FreeRTOS app ...");

    Task::new().name("Serial Send Task").stack_size(256).priority(TaskPriority(2)).start(|| { serial_send_task(clone); }).unwrap();
    Task::new().name("Serial Listen Task").stack_size(256).priority(TaskPriority(2)).start(|| { serial_listen_task(port); }).unwrap();

    //Task::new().name("Hello Task").stack_size(128).priority(TaskPriority(5)).start(||  {hello_task();}).unwrap();


    println!("Tasks registered");
    //let free = freertos_rs_xPortGetFreeHeapSize();
    // println!("Free Memory: {}!", free);
    println!("Starting scheduler");
    FreeRtosUtils::start_scheduler();
    loop {
        println!("Loop forever!");
    }
}

fn hello_task() {
    let mut i = 0;

    loop {
        println!("{}", i);
        CurrentTask::delay(Duration::ms(1000));
        i = i + 1;
    }
}


fn serial_listen_task(mut port: Box<dyn SerialPort>) {
    let mut serial_buf: Vec<u8> = vec![0; 8];

    let mut x_handle: freertos_rust::FreeRtosTaskHandle;
    unsafe { x_handle = freertos_rs_get_current_task(); }

    unsafe {
        loop {
            let mut task_info: FreeRtosCharPtr = freertos_rs_task_get_name(x_handle);
            let byte = *task_info;
            let bytes = [byte];
            let mut task_name = str::from_utf8(bytes.as_ref()).unwrap();
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{}:\n: {}",task_name ,
                                  std::string::String::from_utf8(serial_buf[..t].to_owned()).unwrap()),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}

fn serial_send_task(mut clone: Box<dyn SerialPort>) {
    loop {
        let num = rand::thread_rng().gen_range(1..4);
        clone
            .write(&[num])
            .expect("Failed to write to serial port");
        println!("Send {} to the Ardunio.", num);
        CurrentTask::delay(Duration::ms(1000));
    }
}

#[test]
fn many_boxes() {
    init_allocator();
    println!("many_boxes... ");
    for i in 0..10 { // .. HEAP_SIZE
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    println!("[ok]");
}