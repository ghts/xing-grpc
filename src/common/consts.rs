use std::fmt;

#[derive(Debug)]
pub enum 서버_구분 {
    실거래,
    모의투자,
}

impl fmt::Display for 서버_구분 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            서버_구분::실거래 => write!(f, "실거래"),
            서버_구분::모의투자 => write!(f, "모의투자"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
pub enum T질의_구분 {
    조회,
    주문,
    실시간_정보_구독,
    실시간_정보_해지,
    실시간_정보_일괄_해지,
    접속_및_로그인,
    접속_여부,
    서버_이름,
    계좌_수량,
    계좌번호_모음,
    계좌_이름,
    계좌_상세명,
    계좌_별명,
    전송_제한,
    십분간_질의_횟수,
    서버_구분,
    종료,
}

impl fmt::Display for T질의_구분 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            T질의_구분::조회 => write!(f, "조회"),
            T질의_구분::주문 => write!(f, "주문"),
            T질의_구분::실시간_정보_구독 => write!(f, "실시간 정보 구독"),
            T질의_구분::실시간_정보_해지 => write!(f, "실시간 정보 해지"),
            T질의_구분::실시간_정보_일괄_해지 => write!(f, "실시간 정보 일괄 해지"),
            T질의_구분::접속_및_로그인 => write!(f, "접속 및 로그인"),
            T질의_구분::접속_여부 => write!(f, "접속 여부"),
            T질의_구분::서버_이름 => write!(f, "접속 여부"),
            T질의_구분::계좌_수량 => write!(f, "계좌 수량"),
            T질의_구분::계좌번호_모음 => write!(f, "계좌 번호 모음"),
            T질의_구분::계좌_이름 => write!(f, "계좌 이름"),
            T질의_구분::계좌_상세명 => write!(f, "계좌 상세명"),
            T질의_구분::계좌_별명 => write!(f, "계좌 별명"),
            T질의_구분::전송_제한 => write!(f, "전송 제한"),
            T질의_구분::십분간_질의_횟수 => write!(f, "십분간 질의 횟수"),
            T질의_구분::서버_구분 => write!(f, "서버 구분"),
            T질의_구분::종료 => write!(f, "종료"),
            _ => write!(f, "{:?}", self),
        }
    }
}