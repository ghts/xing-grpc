use std::any::Any;
use std::fmt::Error;
use crossbeam_channel::{Sender,Receiver};
use crate::common::consts::T질의_구분;

pub(crate) struct S백엔드_인수 {
    pub(crate) r질의: Receiver<S질의>,
    pub(crate) r종료: Receiver<()>,
}

// Any : downcast_ref, downcast_mut 메소드를 사용 가능.

pub(crate) struct S질의 {
    구분: T질의_구분,
    TR코드: String,
    질의값: Option<Box<dyn Any>>,
    ch응답: Option<Sender<Box<dyn Any>>>,
}

pub(crate) enum S질의값 {

}
