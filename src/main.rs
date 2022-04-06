use enigo::*;
use std::error::Error;
use std::thread;
use std::time;
macro_rules! input {
    ($($var:ident)*) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut words = buf.split_whitespace();
        $($var = words.next().unwrap().parse().unwrap();)*
    }
}
fn main() {
    // computer in computer time
    print!("Please input how much delay you want to start after the program.. \n :");

    let num : u32;
    input!(num);

    let time_computer = time::Duration::from_secs_f32(num as f32);

    print!("Please input the delay in each command you want to input \n: ");
    let delay : u32 ;
    input!(delay);

    let delay_computer = time::Duration::from_secs_f32(delay as f32);

    let mut msg : String = String::new();
    print!("Please type the message you want to spam LOL\n: ");
    std::io::stdin().read_line(&mut msg).expect("is this really a message?");
    thread::sleep(time_computer);
    enigo_making(delay_computer,msg);
}
fn enigo_making(time: time::Duration,message : String){
    let mut enigo = Enigo::new();
    loop{
    enigo.key_sequence(message.as_str());
    enigo.key_down(Key::Return);
    enigo.key_up(Key::Return);
    thread::sleep(time);
    }
}
