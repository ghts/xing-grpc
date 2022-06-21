use std::any::Any;
use std::fmt::Error;
use crossbeam_channel::{Sender,Receiver};
use crate::common::consts::T질의_구분;

pub(crate) struct S백엔드_인수 {
    pub(crate) r질의: Receiver<dyn I질의값>,
    pub(crate) r종료: Receiver<()>,
}

// Any : downcast_ref, downcast_mut 메소드를 사용 가능.

pub(crate) trait I질의값 {
    fn 구분() -> T질의_구분;
    fn TR코드() -> Option<dyn Into<Vec<u8>>>;
    fn 질의값() -> Option<dyn Any>;
    fn ch응답() -> Option<Sender<Result<dyn Any, Error>>>;
}
