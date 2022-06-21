mod common;
mod backend;

use std::{process, time, thread};
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::common::types::{S백엔드_인수, I질의값};
use crate::backend::backend::실행 as backend_실행;

#[allow(non_ascii_idents, dead_code, unused_variables)]
fn main() {
    let (s질의, r질의) = bounded::<I질의값>(100);
    let (s종료, r종료) = bounded::<()>(100);
    let 백엔드_인수 = S백엔드_인수{ r질의, r종료 };

    println!("Xing API 초기화 Starts.");
    backend_실행(백엔드_인수);
    println!("Xing API 초기화 Ends.");

    println!("Xing API 대기 Starts.");
    thread::sleep(time::Duration::from_secs(20));   // 20초 대기
    println!("Xing API 대기 Ends.");

    process::exit(0);
}
