use std::env;
use crossbeam_channel::select;

use crate::backend::msg_window;
use crate::backend::xing_api;
use crate::common::types::{I질의값, S백엔드_인수};

pub(crate) fn 실행(인수: S백엔드_인수) {
    if !로그인() {
        println!("로그인() 실패.");
        return;
    }

    let r질의 = 인수.r질의;
    let r종료 = 인수.r종료;

    loop {
        select! {
            recv(r질의) -> 질의값 => 질의_처리(질의값),
            recv(r종료) -> _ => {
                msg_window::메세지_윈도우_닫기();
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

fn 질의_처리(질의값: dyn I질의값) {
    panic!("TODO");
}