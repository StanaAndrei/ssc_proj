use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::{thread, time::Duration, iter};
use rand::Rng;

const SLEEP_TIME: Duration = Duration::from_millis(500);

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

fn try_intercept_and_update_msg(mut msg: String, p: f64) -> String {
    if msg.is_empty() {
        return msg;
    }
    let mut rng = rand::rng();
    let success = rng.random_bool(p);
    if success {
        let pos = rng.random_range(0..msg.len());
        if let Some(char_indices) = msg.char_indices().nth(pos) {
            let start = char_indices.0;
            let char_len = msg[start..].chars().next().unwrap().len_utf8();
            msg.replace_range(start..start+char_len, "Z");
        }
    }
    msg
}

fn receive(sent_msg: String, hex: String) -> bool {
    println!("Receiver received message: \n\"{}\"", sent_msg);
    let local_hex = get_hex_hmac(&*sent_msg, SECRET);
    println!("Received code {}", hex);
    println!("Local code:   {}", local_hex);
    if hex != local_hex {
        println!("The message is not valid!!!");
        return false;
    }
    println!("The message is valid!!!");
    true
}

pub fn chat_demo() {
    let hex = get_hex_hmac(MSG, SECRET);
    println!("Sending message: \"\n{}\n\"\n & the code: {}", MSG, hex);

    let mut p = 0.9;
    loop {
        println!("{}", iter::repeat("-").take(100).collect::<String>());
        thread::sleep(SLEEP_TIME);
        let sent_msg = try_intercept_and_update_msg(MSG.to_string(), p);
        let ok = receive(sent_msg.to_string(), hex.clone());
        if ok {
            break;
        }
        p -= 0.3;
    }
}