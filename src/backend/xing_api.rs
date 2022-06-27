use std::env;
use std::ffi::CString;
use std::fmt;
use std::mem;
use std::mem::MaybeUninit;
use std::os::raw::*;
use std::path::PathBuf;
use std::string;
use std::sync;
use std::time;

use crossbeam_channel::{Receiver, Sender, bounded};
use encoding::{DecoderTrap, Encoding};
use encoding::all::WINDOWS_949;
use libloading::{Library, Symbol};
use libloading::os::windows::Symbol as RawSymbol;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{WM_USER,DestroyWindow};

use crate::common::consts::서버_구분;
use crate::backend::msg_window;
use crate::backend::type_c;

type LPSTR = *mut c_char;
type LPCSTR = *const c_char;
type FnConnect = unsafe extern "stdcall" fn(HWND, LPCSTR, c_int, c_int, c_int, c_int) -> BOOL;
type FnIsConnected = unsafe extern "stdcall" fn() -> BOOL;
type FnDisconnect = unsafe extern "stdcall" fn() -> BOOL;
type FnLogin = unsafe extern "stdcall" fn(HWND, LPCSTR, LPCSTR, LPCSTR, c_int, BOOL) -> BOOL;
type FnLogout = unsafe extern "stdcall" fn(HWND) -> BOOL;
type FnRequest = unsafe extern "stdcall" fn(HWND, LPCSTR, LPCSTR, c_int, BOOL, LPCSTR, c_int) -> c_int;
type FnAdviseRealData = unsafe extern "stdcall" fn(HWND, LPCSTR, LPCSTR, c_int) -> BOOL;
type FnUnadviseRealData = unsafe extern "stdcall" fn(HWND, LPCSTR, LPCSTR, c_int) -> BOOL;
type FnUnadviseWindow = unsafe extern "stdcall" fn(HWND) -> BOOL;
type FnGetAccountListCount = unsafe extern "stdcall" fn() -> c_int;
type FnGetAccountList = unsafe extern "stdcall" fn(c_int, LPSTR, c_int) -> BOOL;
type FnGetAccountName = unsafe extern "stdcall" fn(LPCSTR, LPSTR, c_int) -> BOOL;
type FnGetAccountDetailName = unsafe extern "stdcall" fn(LPCSTR, LPSTR, c_int) -> BOOL;
type FnGetAccountNickName = unsafe extern "stdcall" fn(LPCSTR, LPSTR, c_int) -> BOOL;
type FnGetServerName = unsafe extern "stdcall" fn(LPSTR);
type FnGetLastError = unsafe extern "stdcall" fn() -> c_int;
type FnGetErrorMessage = unsafe extern "stdcall" fn(c_int, LPSTR, c_int) -> c_int;
type FnGetTRCountPerSec = unsafe extern "stdcall" fn(LPCSTR) -> c_int;
type FnGetTRCountLimit = unsafe extern "stdcall" fn(LPCSTR) -> c_int;
type FnGetTRCountRequest = unsafe extern "stdcall" fn(LPCSTR) -> c_int;
type FnReleaseRequestData = unsafe extern "stdcall" fn(c_int);
type FnReleaseMessageData = unsafe extern "stdcall" fn(LPARAM);
type FnDecompress = unsafe extern "stdcall" fn(LPCSTR, LPSTR, c_int) -> c_int;

pub(crate) fn 로그인_알림_채널() -> (&'static Sender<bool>, &'static Receiver<bool>) {
    static mut SENDER: MaybeUninit::<Sender<bool>> = MaybeUninit::<Sender<bool>>::uninit();
    static mut RECEIVER: MaybeUninit::<Receiver<bool>> = MaybeUninit::<Receiver<bool>>::uninit();
    static ONCE: sync::Once = sync::Once::new();

    unsafe {
        ONCE.call_once(|| {
            let (s, r) = bounded::<bool>(1);
            SENDER.write(s);
            RECEIVER.write(r);
        });

        (SENDER.assume_init_ref(), RECEIVER.assume_init_ref())
    }
}

pub(crate) fn singleton() -> &'static XingDllWrapper {
    static mut SINGLETON: MaybeUninit::<XingDllWrapper> = MaybeUninit::<XingDllWrapper>::uninit();
    static ONCE: sync::Once = sync::Once::new();

    unsafe {
        ONCE.call_once(|| { SINGLETON.write(초기화()); });
        SINGLETON.assume_init_ref()
    }
}

fn 초기화() -> XingDllWrapper {
    let hWnd = msg_window::메시지_윈도우_생성();

    let 원래_디렉토리 = std::env::current_dir();
    let xing_api_파일 = PathBuf::from(r"C:\eBEST\xingAPI\xingAPI.dll");
    let xing_api_디렉토리 = xing_api_파일.parent();

    env::set_current_dir(&xing_api_디렉토리.unwrap());
    let dll = unsafe { Library::new(xing_api_파일).unwrap() };
    env::set_current_dir(원래_디렉토리.unwrap());

    unsafe {
        XingDllWrapper {
            etkConnect:  dll.get::<FnConnect>(b"ETK_Connect").unwrap().into_raw() ,
            etkIsConnected:  dll.get::<FnIsConnected>(b"ETK_IsConnected").unwrap().into_raw() ,
            etkDisconnect:  dll.get::<FnDisconnect>(b"ETK_Disconnect").unwrap().into_raw() ,
            etkLogin:  dll.get::<FnLogin>(b"ETK_Login").unwrap().into_raw() ,
            etkLogout:  dll.get::<FnLogout>(b"ETK_Logout").unwrap().into_raw() ,
            etkRequest:  dll.get::<FnRequest>(b"ETK_Request").unwrap().into_raw() ,
            etkAdviseRealData:  dll.get::<FnAdviseRealData>(b"ETK_AdviseRealData").unwrap().into_raw() ,
            etkUnadviseRealData:  dll.get::<FnUnadviseRealData>(b"ETK_UnadviseRealData").unwrap().into_raw() ,
            etkUnadviseWindow:  dll.get::<FnUnadviseWindow>(b"ETK_UnadviseWindow").unwrap().into_raw() ,
            etkGetAccountListCount:  dll.get::<FnGetAccountListCount>(b"ETK_GetAccountListCount").unwrap().into_raw() ,
            etkGetAccountList:  dll.get::<FnGetAccountList>(b"ETK_GetAccountList").unwrap().into_raw() ,
            etkGetAccountName:  dll.get::<FnGetAccountName>(b"ETK_GetAccountName").unwrap().into_raw() ,
            etkGetAcctDetailName:  dll.get::<FnGetAccountDetailName>(b"ETK_GetAcctDetailName").unwrap().into_raw() ,
            etkGetAcctNickname:  dll.get::<FnGetAccountNickName>(b"ETK_GetAcctNickname").unwrap().into_raw() ,
            etkGetServerName:  dll.get::<FnGetServerName>(b"ETK_GetServerName").unwrap().into_raw() ,
            etkGetLastError:  dll.get::<FnGetLastError>(b"ETK_GetLastError").unwrap().into_raw() ,
            etkGetErrorMessage:  dll.get::<FnGetErrorMessage>(b"ETK_GetErrorMessage").unwrap().into_raw() ,
            etkGetTRCountPerSec:  dll.get::<FnGetTRCountPerSec>(b"ETK_GetTRCountPerSec").unwrap().into_raw() ,
            etkGetTRCountLimit:  dll.get::<FnGetTRCountLimit>(b"ETK_GetTRCountLimit").unwrap().into_raw() ,
            etkGetTRCountRequest:  dll.get::<FnGetTRCountRequest>(b"ETK_GetTRCountRequest").unwrap().into_raw() ,
            etkReleaseRequestData:  dll.get::<FnReleaseRequestData>(b"ETK_ReleaseRequestData").unwrap().into_raw() ,
            etkReleaseMessageData:  dll.get::<FnReleaseMessageData>(b"ETK_ReleaseMessageData").unwrap().into_raw() ,
            etkDecompress:  dll.get::<FnDecompress>(b"ETK_Decompress").unwrap().into_raw() ,
            dll,
            hWnd,
        }
    }
}

pub(crate) struct XingDllWrapper {
    etkConnect: RawSymbol<FnConnect>,
    etkIsConnected: RawSymbol<FnIsConnected>,
    etkDisconnect: RawSymbol<FnDisconnect>,
    etkLogin: RawSymbol<FnLogin>,
    etkLogout: RawSymbol<FnLogout>,
    etkRequest: RawSymbol<FnRequest>,
    etkAdviseRealData: RawSymbol<FnAdviseRealData>,
    etkUnadviseRealData: RawSymbol<FnUnadviseRealData>,
    etkUnadviseWindow: RawSymbol<FnUnadviseWindow>,
    etkGetAccountListCount: RawSymbol<FnGetAccountListCount>,
    etkGetAccountList: RawSymbol<FnGetAccountList>,
    etkGetAccountName: RawSymbol<FnGetAccountName>,
    etkGetAcctDetailName: RawSymbol<FnGetAccountDetailName>,
    etkGetAcctNickname: RawSymbol<FnGetAccountNickName>,
    etkGetServerName: RawSymbol<FnGetServerName>,
    etkGetLastError: RawSymbol<FnGetLastError>,
    etkGetErrorMessage: RawSymbol<FnGetErrorMessage>,
    etkGetTRCountPerSec: RawSymbol<FnGetTRCountPerSec>,
    etkGetTRCountLimit: RawSymbol<FnGetTRCountLimit>,
    etkGetTRCountRequest: RawSymbol<FnGetTRCountRequest>,
    etkReleaseRequestData: RawSymbol<FnReleaseRequestData>,
    etkReleaseMessageData: RawSymbol<FnReleaseMessageData>,
    etkDecompress: RawSymbol<FnDecompress>,
    dll: Library,
    hWnd: HWND,
}

impl XingDllWrapper {
    pub(crate) fn Connect(&self, 서버: 서버_구분) -> bool {
        let 서버_이름: &str;
        let 포트_번호 = 20001;

        match 서버 {
            서버_구분::실거래 => 서버_이름 = "hts.ebestsec.co.kr",
            서버_구분::모의투자 => 서버_이름 = "demo.ebestsec.co.kr",
            _ => panic!("예상하지 못한 서버 구분 : {}", 서버),
        }

        bool_rust(unsafe {
            (self.etkConnect)(
                self.hWnd,
                CString::new(서버_이름).unwrap().as_ptr(),
                포트_번호,
                WM_USER as i32, -1, -1)
        })
    }

    pub(crate) fn IsConnected(&self) -> bool {
        bool_rust(unsafe { (self.etkIsConnected)() })
    }
    pub(crate) fn Disconnect(&self) -> bool {
        bool_rust(unsafe { (self.etkDisconnect)() })
    }

    pub(crate) fn Login<T: Into<Vec<u8>>>(&self, strId: T, strPwd: T, strCertPwd: T) -> bool {
        let id = CString::new(strId).unwrap().as_ptr();
        let pwd = CString::new(strPwd).unwrap().as_ptr();
        let certPwd = CString::new(strCertPwd).unwrap().as_ptr();

        bool_rust(unsafe {
            (self.etkLogin)(
                self.hWnd,
                id,
                pwd,
                certPwd,
                0, 0)
        })
    }

    pub(crate) fn Logout(&self) -> bool {
        bool_rust(unsafe { (self.etkLogout)(self.hWnd) })
    }

    pub(crate) fn Request<T: Into<Vec<u8>>>(&self, TR코드: T, c데이터: &[u8], 연속_조회_여부: bool,
                                     연속키: T, 타임아웃: time::Duration) -> isize {
        unsafe {
            (self.etkRequest)(
                self.hWnd,
                CString::new(TR코드).unwrap().as_ptr(),
                c데이터.as_ptr() as *const i8,
                c데이터.len() as c_int,
                if 연속_조회_여부 { 1 } else { 0 },
                CString::new(연속키).unwrap().as_ptr(),
                타임아웃.as_secs() as c_int) as isize
        }
    }

    pub(crate) fn AdviseRealData<T: Into<Vec<u8>>>(&self, TR코드: T, 전체_종목코드: T, 단위_길이: isize) -> bool {
        bool_rust(unsafe {
            (self.etkAdviseRealData)(
                self.hWnd,
                CString::new(TR코드).unwrap().as_ptr(),
                CString::new(전체_종목코드).unwrap().as_ptr(),
                단위_길이 as c_int)
        })
    }

    pub(crate) fn UnadviseRealData<T: Into<Vec<u8>>>(&self, TR코드: T, 전체_종목코드: T, 단위_길이: isize) -> bool {
        bool_rust(unsafe {
            (self.etkUnadviseRealData)(
                self.hWnd,
                CString::new(TR코드).unwrap().as_ptr(),
                CString::new(전체_종목코드).unwrap().as_ptr(),
                단위_길이 as c_int)
        })
    }

    pub(crate) fn UnadviseWindow(&self) -> bool { bool_rust(unsafe { (self.etkUnadviseWindow)(self.hWnd) }) }

    pub(crate) fn GetAccountListCount(&self) -> usize { unsafe { (self.etkGetAccountListCount)() as usize } }

    pub(crate) fn GetAccountList(&self, 인덱스: isize) -> String {
        unsafe {
            let mut 버퍼: [u8; 12] = mem::zeroed();

            (self.etkGetAccountList)(
                인덱스 as c_int,
                버퍼.as_mut_ptr() as *mut i8,
                버퍼.len() as c_int);

            CString::new(버퍼).unwrap().into_string().unwrap()
        }
    }

    pub(crate) fn GetAccountName<T: Into<Vec<u8>>>(&self, 계좌번호: T) -> String {
        unsafe {
            let mut 버퍼: [u8; 41] = mem::zeroed();

            (self.etkGetAccountName)(
                CString::new(계좌번호).unwrap().as_ptr(),
                버퍼.as_mut_ptr() as *mut i8,
                버퍼.len() as c_int);

            WINDOWS_949.decode(버퍼.as_slice(), DecoderTrap::Strict).unwrap()
        }
    }

    pub(crate) fn GetAcctDetailName<T: Into<Vec<u8>>>(&self, 계좌번호: T) -> String {
        unsafe {
            let mut 버퍼: [u8; 41] = mem::zeroed();

            (self.etkGetAcctDetailName)(
                CString::new(계좌번호).unwrap().as_ptr(),
                버퍼.as_mut_ptr() as *mut i8,
                버퍼.len() as c_int);

            WINDOWS_949.decode(버퍼.as_slice(), DecoderTrap::Strict).unwrap()
        }
    }
    pub(crate) fn GetAcctNickname<T: Into<Vec<u8>>>(&self, 계좌번호: T) -> String {
        unsafe {
            let mut 버퍼: [u8; 41] = mem::zeroed();

            (self.etkGetAcctNickname)(
                CString::new(계좌번호).unwrap().as_ptr(),
                버퍼.as_mut_ptr() as *mut i8,
                버퍼.len() as c_int);

            WINDOWS_949.decode(버퍼.as_slice(), DecoderTrap::Strict).unwrap()
        }
    }

    pub(crate) fn GetServerName(&self) -> String {
        unsafe {
            let mut 버퍼: [u8; 51] = mem::zeroed();

            (self.etkGetServerName)(버퍼.as_mut_ptr() as *mut i8);

            CString::new(버퍼).unwrap().into_string().unwrap()
        }
    }

    pub(crate) fn GetLastError(&self) -> isize {
        unsafe { (self.etkGetLastError)() as isize }
    }

    pub(crate) fn GetErrorMessage(&self, 에러코드: isize) -> String {
        unsafe {
            let mut 버퍼: [u8; 1024] = mem::zeroed();

            (self.etkGetErrorMessage)(
                에러코드 as c_int,
                버퍼.as_mut_ptr() as *mut i8,
                버퍼.len() as c_int);

            WINDOWS_949.decode(버퍼.as_slice(), DecoderTrap::Strict).unwrap()
        }
    }

    pub(crate) fn GetTRCountPerSec<T: Into<Vec<u8>>>(&self, TR코드: T) -> isize {
        unsafe { (self.etkGetTRCountPerSec)(CString::new(TR코드).unwrap().as_ptr()) as isize }
    }

    pub(crate) fn GetTRCountLimit<T: Into<Vec<u8>>>(&self, TR코드: T) -> isize {
        unsafe { (self.etkGetTRCountLimit)(CString::new(TR코드).unwrap().as_ptr()) as isize }
    }

    pub(crate) fn GetTRCountRequest<T: Into<Vec<u8>>>(&self, TR코드: T) -> isize {
        unsafe { (self.etkGetTRCountRequest)(CString::new(TR코드).unwrap().as_ptr()) as isize }
    }

    pub(crate) fn ReleaseRequestData(&self, 식별번호: i32) {
        unsafe { (self.etkReleaseRequestData)(식별번호 as c_int) }
    }

    pub(crate) fn ReleaseMessageData(&self, lParam: *const type_c::REALTIME_DATA) {
        unsafe { (self.etkReleaseMessageData)(lParam as LPARAM) }
    }

    pub(crate) fn Decompress(&self, 버퍼: *mut i8, 압축_원본_데이터: *const c_uchar, 압축_데이터_길이: c_int) -> isize {
        unsafe {
            ((self.etkDecompress)(
                압축_원본_데이터 as *const i8,
                버퍼,
                압축_데이터_길이) as isize)
        }
    }

    pub(crate) fn 메시지_윈도우_닫기(&self) {
        unsafe { DestroyWindow(self.hWnd); }
    }
}

fn bool_rust(값: BOOL) -> bool {
    if 값 == 0 {
        false
    } else {
        true
    }
}
