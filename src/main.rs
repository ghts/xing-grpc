mod xing_api;
mod msg_window;
mod type_c;

#[allow(non_ascii_idents, dead_code, unused_variables)]
fn main() {
    println!("Xing API 초기화 Start");
    let dll = xing_api::초기화();
    println!("Xing API 초기화 Finish");

    println!("Xing API 닫기 Start");
    dll.닫기();
    println!("Xing API 닫기 Finish");
}
