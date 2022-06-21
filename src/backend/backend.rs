use std::fmt;
use std::env;
use crossbeam_channel::{Receiver, select};

use crate::backend::msg_window;
use crate::backend::xing_api;
use crate::common::types::S질의값;

pub(crate) fn 실행(r질의: Receiver<S질의값>) {
    if !로그인() {
        println!("로그인() 실패.");
        return;
    }

    loop {
        select! {
            recv(r질의) -> 질의값 => 질의_처리(질의값)
            recv(r종료) => {
                msg_window::메세지_윈도우_닫기(hWnd);
                return;
            }
            default => msg_window::윈도우_메시지_처리(),
        }
    }
}

fn 로그인() -> bool {
    let dll = xing_api::singleton();

    let id = env::var("XING_LOGIN_ID").unwrap();
    let pwd = env::var("XING_LOGIN_PWD").unwrap();
    let certPwd = env::var("XING_CERT_PWD").unwrap();

    if !dll.Login(id, pwd, certPwd) {
        println!("DLL login() 호출 에러.");
        return false;
    };

    let (_, r) = xing_api::로그인_알림_채널();

    r.recv().unwrap()
}

fn 질의_처리(질의값: S질의값) {
    panic!("TODO");
}