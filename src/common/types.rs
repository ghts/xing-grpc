use std::any::Any;
use std::fmt::Error;
use crossbeam_channel::Sender;

use crate::common::consts::T질의_구분;

// Any : downcast_ref, downcast_mut 메소드를 사용 가능.

pub(crate) struct S질의값 {
    pub(crate) 구분: T질의_구분,
    pub(crate) TR코드: Option<Into<Vec<u8>>>,
    pub(crate) 질의값: Option<Any>,
    pub(crate) ch응답: Option<Sender<Result<Any, Error>>>,
}
