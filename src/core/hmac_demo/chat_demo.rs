use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::{thread, time::Duration, iter};

const SLEEP_TIME: Duration = Duration::from_millis(1_000);

const MSG: &str = "\
This message contains a secret code which is essential to arrive intact.\n\
The secret code is: <qwErTY321>\n\
!!!DO NOT DISTRIBUTE!!!\
";
const SECRET: &str = "wasd123";
type HmacSha256 = Hmac<Sha256>;


fn get_hex_hmac(msg: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(msg.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    code_bytes.iter()
        .map(|b: &u8| format!("{:02x}", b))
        .collect::<String>()
}

pub fn chat_demo() {
    let hex = get_hex_hmac(MSG, SECRET);
    println!("Sending message: \"\n{}\n\"\n & the code: {}", MSG, hex);
    println!("{}", iter::repeat("-").take(100).collect::<String>());

    thread::sleep(SLEEP_TIME);

    println!("Receiver received message: \"{}\"", MSG);
    let local_hex = get_hex_hmac(MSG, SECRET);
    println!("Received code {}", hex);
    println!("Local code:   {}", local_hex);
    if hex != local_hex {
        println!("The message is not valid!!!");
    } else {
        println!("The message is valid!!!");
    }
}