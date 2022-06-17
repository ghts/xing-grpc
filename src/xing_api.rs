use std::env;
use std::ffi::CString;
use std::fmt;
use std::mem;
use std::os::raw::*;
use std::path::PathBuf;

use libloading::{Library, Symbol};
use libloading::os::windows::Symbol as RawSymbol;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::UI::WindowsAndMessaging::WM_USER;

use crate::msg_window;

type LPSTR = *mut c_char;
type LPCSTR = *const c_char;
type LPTSTR = *mut c_char;
type LPCTSTR = *const c_char;
type LPVOID = *mut c_void;
type EtkConnectFunc = unsafe extern "stdcall" fn(HWND, LPCSTR, c_int, c_int, c_int, c_int) -> BOOL;
type EtkIsConnectedFunc = unsafe extern "stdcall" fn() -> BOOL;
type EtkLoginFunc = unsafe extern "stdcall" fn(HWND, LPCSTR, LPCSTR, LPCSTR, c_int, BOOL) -> BOOL;
type EtkLogoutFunc = unsafe extern "stdcall" fn(HWND) -> BOOL;
type EtkRequestFunc = unsafe extern "stdcall" fn(HWND, LPCTSTR, LPVOID, c_int, BOOL, LPCTSTR, c_int) -> c_int;
type EtkAdviseRealDataFunc = unsafe extern "stdcall" fn(HWND, LPCTSTR, LPCTSTR, c_int) -> BOOL;
type EtkUnadviseRealDataFunc = unsafe extern "stdcall" fn(HWND, LPCTSTR, LPCTSTR, c_int) -> BOOL;
type EtkUnadviseWindowFunc = unsafe extern "stdcall" fn(HWND) -> BOOL;
type EtkGetAccountListCountFunc = unsafe extern "stdcall" fn() -> c_int;
type EtkGetAccountListFunc = unsafe extern "stdcall" fn(c_int, LPSTR, c_int) -> BOOL;
type EtkGetAccountNameFunc = unsafe extern "stdcall" fn(LPCTSTR, LPSTR, c_int) -> BOOL;
type EtkGetAccountDetailNameFunc = unsafe extern "stdcall" fn(LPCTSTR, LPSTR, c_int) -> BOOL;
type EtkGetAccountNickNameFunc = unsafe extern "stdcall" fn(LPCTSTR, LPSTR, c_int) -> BOOL;
type EtkGetServerNameFunc = unsafe extern "stdcall" fn(LPTSTR);
type EtkGetLastErrorFunc = unsafe extern "stdcall" fn() -> c_int;
type EtkGetErrorMessageFunc = unsafe extern "stdcall" fn(c_int, LPSTR, c_int) -> c_int;
type EtkGetTRCountPerSecFunc = unsafe extern "stdcall" fn(LPCTSTR) -> c_int;
type EtkGetTRCountLimitFunc = unsafe extern "stdcall" fn(LPCTSTR) -> c_int;
type EtkGetTRCountRequestFunc = unsafe extern "stdcall" fn(LPCTSTR) -> c_int;
type EtkReleaseRequestDataFunc = unsafe extern "stdcall" fn(c_int);
type EtkReleaseMessageDataFunc = unsafe extern "stdcall" fn(LPARAM);
type EtkDecompressFunc = unsafe extern "stdcall" fn(LPCTSTR, LPCTSTR, c_int) -> c_int;

fn bool_c2rust(값: BOOL) -> bool {
    if 값 == 0 {
        false
    } else {
        true
    }
}

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

pub fn 초기화() -> XingDllWrapper {
    let hWnd = msg_window::메시지_윈도우_생성();

    let 원래_디렉토리 = std::env::current_dir();
    let xing_api_파일 = PathBuf::from(r"C:\eBEST\xingAPI\xingAPI.dll");
    let xing_api_디렉토리 = xing_api_파일.parent();

    env::set_current_dir(&xing_api_디렉토리.unwrap());
    let dll = unsafe { Library::new(xing_api_파일).unwrap() };
    env::set_current_dir(원래_디렉토리.unwrap());

    let vConnect: Symbol<EtkConnectFunc> = unsafe { dll.get(b"ETK_Connect").unwrap() };
    let vIsConnected: Symbol<EtkIsConnectedFunc> = unsafe { dll.get(b"ETK_IsConnected").unwrap() };
    let vLogin: Symbol<EtkLoginFunc> = unsafe { dll.get(b"ETK_Login").unwrap() };
    let vLogout: Symbol<EtkLogoutFunc> = unsafe { dll.get(b"ETK_Logout").unwrap() };
    let vRequest: Symbol<EtkRequestFunc> = unsafe { dll.get(b"ETK_Request").unwrap() };
    let vAdviseRealData: Symbol<EtkAdviseRealDataFunc> = unsafe { dll.get(b"ETK_AdviseRealData").unwrap() };
    let vUnadviseRealData: Symbol<EtkUnadviseRealDataFunc> = unsafe { dll.get(b"ETK_UnadviseRealData").unwrap() };
    let vUnadviseWindow: Symbol<EtkUnadviseWindowFunc> = unsafe { dll.get(b"ETK_UnadviseWindow").unwrap() };
    let vGetAccountListCount: Symbol<EtkGetAccountListCountFunc> = unsafe { dll.get(b"ETK_GetAccountListCount").unwrap() };
    let vGetAccountList: Symbol<EtkGetAccountListFunc> = unsafe { dll.get(b"ETK_GetAccountList").unwrap() };
    let vGetAccountName: Symbol<EtkGetAccountNameFunc> = unsafe { dll.get(b"ETK_GetAccountName").unwrap() };
    let vGetAcctDetailName: Symbol<EtkGetAccountDetailNameFunc> = unsafe { dll.get(b"ETK_GetAcctDetailName").unwrap() };
    let vGetAcctNickname: Symbol<EtkGetAccountNickNameFunc> = unsafe { dll.get(b"ETK_GetAcctNickname").unwrap() };
    let vGetServerName: Symbol<EtkGetServerNameFunc> = unsafe { dll.get(b"ETK_GetServerName").unwrap() };
    let vGetLastError: Symbol<EtkGetLastErrorFunc> = unsafe { dll.get(b"ETK_GetLastError").unwrap() };
    let vGetErrorMessage: Symbol<EtkGetErrorMessageFunc> = unsafe { dll.get(b"ETK_GetErrorMessage").unwrap() };
    let vGetTRCountPerSec: Symbol<EtkGetTRCountPerSecFunc> = unsafe { dll.get(b"ETK_GetTRCountPerSec").unwrap() };
    let vGetTRCountLimit: Symbol<EtkGetTRCountLimitFunc> = unsafe { dll.get(b"ETK_GetTRCountLimit").unwrap() };
    let vGetTRCountRequest: Symbol<EtkGetTRCountRequestFunc> = unsafe { dll.get(b"ETK_GetTRCountRequest").unwrap() };
    let vReleaseRequestData: Symbol<EtkReleaseRequestDataFunc> = unsafe { dll.get(b"ETK_ReleaseRequestData").unwrap() };
    let vReleaseMessageData: Symbol<EtkReleaseMessageDataFunc> = unsafe { dll.get(b"ETK_ReleaseMessageData").unwrap() };
    let vDecompress: Symbol<EtkDecompressFunc> = unsafe { dll.get(b"ETK_Decompress").unwrap() };

    XingDllWrapper {
        etkConnect: unsafe { vConnect.into_raw() },
        etkIsConnected: unsafe { vIsConnected.into_raw() },
        etkLogin: unsafe { vLogin.into_raw() },
        etkLogout: unsafe { vLogout.into_raw() },
        etkRequest: unsafe { vRequest.into_raw() },
        etkAdviseRealData: unsafe { vAdviseRealData.into_raw() },
        etkUnadviseRealData: unsafe { vUnadviseRealData.into_raw() },
        etkUnadviseWindow: unsafe { vUnadviseWindow.into_raw() },
        etkGetAccountListCount: unsafe { vGetAccountListCount.into_raw() },
        etkGetAccountList: unsafe { vGetAccountList.into_raw() },
        etkGetAccountName: unsafe { vGetAccountName.into_raw() },
        etkGetAcctDetailName: unsafe { vGetAcctDetailName.into_raw() },
        etkGetAcctNickname: unsafe { vGetAcctNickname.into_raw() },
        etkGetServerName: unsafe { vGetServerName.into_raw() },
        etkGetLastError: unsafe { vGetLastError.into_raw() },
        etkGetErrorMessage: unsafe { vGetErrorMessage.into_raw() },
        etkGetTRCountPerSec: unsafe { vGetTRCountPerSec.into_raw() },
        etkGetTRCountLimit: unsafe { vGetTRCountLimit.into_raw() },
        etkGetTRCountRequest: unsafe { vGetTRCountRequest.into_raw() },
        etkReleaseRequestData: unsafe { vReleaseRequestData.into_raw() },
        etkReleaseMessageData: unsafe { vReleaseMessageData.into_raw() },
        etkDecompress: unsafe { vDecompress.into_raw() },
        dll,
        hWnd,
    }
}

pub struct XingDllWrapper {
    etkConnect: RawSymbol<EtkConnectFunc>,
    etkIsConnected: RawSymbol<EtkIsConnectedFunc>,
    etkLogin: RawSymbol<EtkLoginFunc>,
    etkLogout: RawSymbol<EtkLogoutFunc>,
    etkRequest: RawSymbol<EtkRequestFunc>,
    etkAdviseRealData: RawSymbol<EtkAdviseRealDataFunc>,
    etkUnadviseRealData: RawSymbol<EtkUnadviseRealDataFunc>,
    etkUnadviseWindow: RawSymbol<EtkUnadviseWindowFunc>,
    etkGetAccountListCount: RawSymbol<EtkGetAccountListCountFunc>,
    etkGetAccountList: RawSymbol<EtkGetAccountListFunc>,
    etkGetAccountName: RawSymbol<EtkGetAccountNameFunc>,
    etkGetAcctDetailName: RawSymbol<EtkGetAccountDetailNameFunc>,
    etkGetAcctNickname: RawSymbol<EtkGetAccountNickNameFunc>,
    etkGetServerName: RawSymbol<EtkGetServerNameFunc>,
    etkGetLastError: RawSymbol<EtkGetLastErrorFunc>,
    etkGetErrorMessage: RawSymbol<EtkGetErrorMessageFunc>,
    etkGetTRCountPerSec: RawSymbol<EtkGetTRCountPerSecFunc>,
    etkGetTRCountLimit: RawSymbol<EtkGetTRCountLimitFunc>,
    etkGetTRCountRequest: RawSymbol<EtkGetTRCountRequestFunc>,
    etkReleaseRequestData: RawSymbol<EtkReleaseRequestDataFunc>,
    etkReleaseMessageData: RawSymbol<EtkReleaseMessageDataFunc>,
    etkDecompress: RawSymbol<EtkDecompressFunc>,
    dll: Library,
    hWnd: HWND,
}

impl XingDllWrapper {
    pub fn Connect(&self, 서버: 서버_구분) -> bool {
        let 서버_이름: &str;
        let 포트_번호 = 20001;

        match 서버 {
            서버_구분::실거래 => 서버_이름 = "hts.ebestsec.co.kr",
            서버_구분::모의투자 => 서버_이름 = "demo.ebestsec.co.kr",
            _ => panic!("예상하지 못한 서버 구분 : {}", 서버),
        }


        bool_c2rust(unsafe {
            (self.etkConnect)(
                self.hWnd,
                CString::new(서버_이름).unwrap().as_ptr(),
                포트_번호,
                WM_USER as i32, -1, -1)
        })
    }

    pub fn IsConnected(&self) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkIsConnectedFunc> = unsafe { self.dll.get(ETK_IsConnected).unwrap() };
    }

    pub fn Login(&self, hwnd: HWND, str1: LPCSTR, str2: LPCSTR, str3: LPCSTR, i: c_int, bool: BOOL) {// -> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkLoginFunc> = unsafe { self.dll.get(ETK_Login).unwrap() };
    }

    pub fn Logout(&self, hwnd: HWND) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkLogoutFunc> = unsafe { self.dll.get(ETK_Logout).unwrap() };
    }

    pub fn Request(&self, hwnd: HWND, str1: LPCTSTR, void: LPVOID, i1: c_int, bool: BOOL, str2: LPCTSTR, i2: c_int) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkRequestFunc> = unsafe { self.dll.get(ETK_Request).unwrap() };
    }

    pub fn AdviseRealData(&self, hwnd: HWND, str1: LPCTSTR, str2: LPCTSTR, i: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkAdviseRealDataFunc> = unsafe { self.dll.get(ETK_AdviseRealData).unwrap() };
    }

    pub fn UnadviseRealData(&self, hwnd: HWND, str1: LPCTSTR, str2: LPCTSTR, i: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkUnadviseRealDataFunc> = unsafe { self.dll.get(ETK_UnadviseRealData).unwrap() };
    }

    pub fn UnadviseWindow(&self, hwnd: HWND) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkUnadviseWindowFunc> = unsafe { self.dll.get(ETK_UnadviseWindow).unwrap() };
    }

    pub fn GetAccountListCount(&self) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol/**/<EtkGetAccountListCountFunc> = unsafe { self.dll.get(ETK_GetAccountListCount).unwrap() };
    }

    pub fn GetAccountList(&self, i1: c_int, str: LPSTR, i2: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkGetAccountListFunc> = unsafe { self.dll.get(ETK_GetAccountList).unwrap() };
    }

    pub fn GetAccountName(&self, str1: LPCTSTR, str2: LPSTR, i: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkGetAccountNameFunc> = unsafe { self.dll.get(ETK_GetAccountName).unwrap() };
    }

    pub fn GetAcctDetailName(&self, str1: LPCTSTR, str2: LPSTR, i: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkGetAccountDetailNameFunc> = unsafe { self.dll.get(ETK_GetAcctDetailName).unwrap() };
    }
    pub fn GetAcctNickname(&self, str1: LPCTSTR, str2: LPSTR, i: c_int) {//-> BOOL {
        panic!("TODO");
        // let 함수: Symbol<EtkGetAccountNickNameFunc> = unsafe { self.dll.get(ETK_GetAcctNickname).unwrap() };
    }

    pub fn GetServerName(&self, str: LPTSTR) {
        panic!("TODO");
        // let 함수: Symbol<EtkGetServerNameFunc> = unsafe { self.dll.get(ETK_GetServerName).unwrap() };
    }

    pub fn GetLastError(&self) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkGetLastErrorFunc> = unsafe { self.dll.get(ETK_GetLastError).unwrap() };
    }

    pub fn GetErrorMessage(&self, i1: c_int, str: LPSTR, i2: c_int) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkGetErrorMessageFunc> = unsafe { self.dll.get(ETK_GetErrorMessage).unwrap() };
    }

    pub fn GetTRCountPerSec(&self, str: LPCTSTR) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkGetTRCountPerSecFunc> = unsafe { self.dll.get(ETK_GetTRCountPerSec).unwrap() };
    }

    pub fn GetTRCountLimit(&self, str: LPCTSTR) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkGetTRCountLimitFunc> = unsafe { self.dll.get(ETK_GetTRCountLimit).unwrap() };
    }

    pub fn GetTRCountRequest(&self, str: LPCTSTR) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkGetTRCountRequestFunc> = unsafe { self.dll.get(ETK_GetTRCountRequest).unwrap() };
    }

    pub fn ReleaseRequestData(&self, i: c_int) {
        panic!("TODO");
        // let 함수: Symbol<EtkReleaseRequestDataFunc> = unsafe { self.dll.get(ETK_ReleaseRequestData).unwrap() };
    }

    pub fn ReleaseMessageData(&self, lParam: LPARAM) {
        panic!("TODO");
        // let 함수: Symbol<EtkReleaseMessageDataFunc> = unsafe { self.dll.get(ETK_ReleaseMessageData).unwrap() };
    }

    pub fn Decompress(&self, str1: LPCTSTR, str2: LPCTSTR, i: c_int) {//-> c_int {
        panic!("TODO");
        // let 함수: Symbol<EtkDecompressFunc> = unsafe { self.dll.get(ETK_Decompress).unwrap() };
    }

    pub fn 닫기(self) {
        msg_window::메세지_윈도우_닫기(self.hWnd);
        unsafe { self.dll.close() };
    }
}
