use std::ffi::CStr;
use std::mem;
use std::os::raw::*;
use std::ptr::copy_nonoverlapping;

use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::UI::WindowsAndMessaging::{CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, HWND_MESSAGE, MSG, PeekMessageA, PostQuitMessage, RegisterClassA, WM_DESTROY, WM_QUIT, WM_USER, WNDCLASSA};

use crate::backend::consts;
use crate::backend::type_c;
use crate::backend::xing_api;

const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

const XM_INIT: u32 = WM_USER;
const XM_DISCONNECT: u32 = XM_INIT + 1;
const XM_RECEIVE_DATA: u32 = XM_INIT + 3;
const XM_RECEIVE_REAL_DATA: u32 = XM_INIT + 4;
const XM_LOGIN: u32 = XM_INIT + 5;
const XM_LOGOUT: u32 = XM_INIT + 6;
const XM_TIMEOUT: u32 = XM_INIT + 7;
const XM_RECEIVE_LINK_DATA: u32 = XM_INIT + 8;
const XM_RECEIVE_REAL_DATA_CHART: u32 = XM_INIT + 10;

const RCV_TR_DATA: WPARAM = 1;
const RCV_MSG_DATA: WPARAM = 2;
const RCV_SYSTEM_ERROR: WPARAM = 3;
const RCV_RELEASE: WPARAM = 4;

// static mut hWndValue:HWND = 0;

pub(crate) fn 메시지_윈도우_생성() -> HWND {
    unsafe {
        let instance = GetModuleHandleA(std::ptr::null());
        debug_assert!(instance != 0);

        let 클래스명 = b"MessageWindowClass\0".as_ptr();
        let 타이틀 = b"MessageWindow\0".as_ptr();

        let wc = WNDCLASSA {
            hCursor: 0,
            hInstance: instance,
            lpszClassName: 클래스명,
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0,
            hbrBackground: 0,
            lpszMenuName: std::ptr::null(),
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(0, 클래스명, 타이틀,
                               0, 0, 0, 0, 0, HWND_MESSAGE,
                               0, instance, std::ptr::null())
    }
}

extern "system" fn wndproc(hWnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            XM_DISCONNECT => {
                println!("XM_DISCONNECT");
                OnDisconnected();
                TRUE as LRESULT
            }
            XM_RECEIVE_DATA => {
                println!("XM_RECEIVE_DATA");
                match wParam {
                    RCV_TR_DATA => {
                        println!("RCV_TR_DATA");
                        OnTrData(lParam as *const type_c::TR_DATA);
                        TRUE as LRESULT
                    }
                    RCV_MSG_DATA | RCV_SYSTEM_ERROR => {
                        OnMessageAndError(lParam as *const type_c::MSG_DATA);
                        TRUE as LRESULT
                    }
                    RCV_RELEASE => {
                        println!("RCV_RELEASE");
                        OnReleaseData(lParam as i32);
                        TRUE as LRESULT
                    }
                    _ => panic!("Unexpectd wParam value : {}", wParam),
                }
            }
            XM_RECEIVE_REAL_DATA => {
                println!("XM_RECEIVE_REAL_DATA");
                OnRealtimeData(lParam as *const type_c::REALTIME_DATA);
                TRUE as LRESULT
            }
            XM_LOGIN => {
                println!("XM_LOGIN");
                OnLogin(wParam as *const c_char, lParam as *const c_char);
                TRUE as LRESULT
            }
            XM_LOGOUT => {
                println!("XM_LOGOUT");
                OnLogout();
                TRUE as LRESULT
            }
            XM_TIMEOUT => {
                println!("XM_TIMEOUT");
                OnTimeout(lParam as i32);
                TRUE as LRESULT
            }
            XM_RECEIVE_LINK_DATA => {
                println!("XM_RECEIVE_LINK_DATA");
                panic!("XM_RECEIVE_LINK_DATA not implemented.");
            }
            XM_RECEIVE_REAL_DATA_CHART => {
                println!("XM_RECEIVE_REAL_DATA");
                panic!("XM_RECEIVE_REAL_DATA not implemented.");
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                FALSE as LRESULT
            }
            _ => DefWindowProcA(hWnd, message, wParam, lParam),
        }
    }
}

pub(crate) fn 윈도우_메시지_처리() {
    unsafe {
        let mut 메시지: MSG = mem::zeroed();

        loop {
            if PeekMessageA(&mut 메시지, 0, 0, 0, 1) == 0 {
                return;
            } else if 메시지.message == WM_QUIT {
                return;
            } else {
                DispatchMessageA(&메시지);
            }
        }
    }
}

// pub(crate) fn 메세지_윈도우_닫기() {
//     unsafe {
//         if hWndValue != 0 {
//             DestroyWindow(hWndValue);
//             hWndValue = 0;
//         }
//     }
// }

fn OnDisconnected() {
    panic!("TODO : 재접속 기능 구현.");
}

fn OnTrData(ptr: *const type_c::TR_DATA) {
    let TR코드 = unsafe { CStr::from_ptr((*ptr).TrCode.as_ptr()).to_str().unwrap() };
    let 블록명 = unsafe { CStr::from_ptr((*ptr).BlockName.as_ptr()).to_str().unwrap() };
    let mut raw데이터:Vec<u8>;
    let mut 길이: usize;

    // t8411, t8412, t8413 반복값은 압축되어 있음. 압축해제가 필요.
    match 블록명 {
        "t8411OutBlock1"|"t8412OutBlock1"|"t8413OutBlock1" => {
            let 단위_길이 = match 블록명 {
                "t8411OutBlock1" => consts::SizeT8411OutBlock1,
                "t8412OutBlock1" => consts::SizeT8412OutBlock1,
                "t8413OutBlock1" => consts::SizeT8413OutBlock1,
                _ => panic!("예상하지 못한 경우"),
            };

            raw데이터 = Vec::<u8>::with_capacity(단위_길이*2000);

            unsafe {
                길이 = xing_api::singleton().Decompress(
                    raw데이터.as_mut_ptr() as *mut i8,
                    (*ptr).Data,
                    (*ptr).DataLength) as usize;
                assert_eq!(길이 % 단위_길이, 0);
                raw데이터.set_len(길이);
            }
        }
        _ => {
            unsafe {
                // 메모리 안전성을 위해서 복사해서 사용. 복사한 데이터는 Rust가 알아서 관리해 줄테니까.
                길이 = (*ptr).DataLength as usize;
                raw데이터 = Vec::<u8>::with_capacity(길이);
                copy_nonoverlapping((*ptr).Data, raw데이터.as_mut_ptr(), 길이);
                raw데이터.set_len(길이);
            }
        }
    }

    let 추가_연속조회_필요_문자열 = unsafe { CStr::from_ptr((*ptr).Cont.as_ptr()).to_str().unwrap() };

    let 추가_연속조회_필요 = match 추가_연속조회_필요_문자열 {
        ""|"0"|"N" =>  false,
        "1"|"Y" => true,
        _ => panic!("예상하지 못한 값 : '{}'", 추가_연속조회_필요_문자열),
    };

    let 연속키: &str;

    if 추가_연속조회_필요 {
        unsafe {
            연속키 = CStr::from_ptr((*ptr).ContKey.as_ptr()).to_str().unwrap().to_owned().replace(" ", "").as_str();
        }
    } else {
        연속키 = "";
    }

    panic!("TODO")

    // 콜백을 하는 대신 로컬 저장소에 등록을 해야 함.
    // 자료형_문자열 := lib.F확인(f자료형_문자열_해석(g)).(string)
    // raw값 = f민감정보_삭제(raw값, 자료형_문자열);
    // 콜백값 := lib.New콜백_TR데이터(int(g.RequestID), 바이트_변환값, TR코드, 추가_연속조회_필요, 연속키)
    // F콜백(콜백값)
}

fn OnMessageAndError(ptr_msg_data: *const type_c::MSG_DATA) {
    panic!("TODO");
}

fn OnReleaseData(식별번호: i32) {
    xing_api::singleton().ReleaseRequestData(식별번호);
}

fn OnRealtimeData(lParam: *const type_c::REALTIME_DATA) {
    panic!("TODO");
}

fn OnLogin(wParam: *const c_char, lParam: *const c_char) {
    panic!("TODO");

}

fn OnLogout() {
    panic!("TODO");
}

fn OnTimeout(식별번호: i32) {
    panic!("TODO");
}

// fn 압축_해제(압축된_원본_데이터 mut , 버퍼 *byte, 원본_데이터_길이 int32) int {
// TODO

// 압축_해제된_데이터_길이, _, 에러_번호 := syscall.Syscall(etkDecompress, 3,
// uintptr(압축된_원본_데이터),
// uintptr(unsafe.Pointer(버퍼)),
// uintptr(원본_데이터_길이))
//
// if 에러_번호 != 0 {
// lib.New에러with출력("F압축_해제() 에러 발생. 에러 코드 : '%v'", 에러_번호)
// }
//
// return int(압축_해제된_데이터_길이)
// }
