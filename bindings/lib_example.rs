/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct text_s {
    pub str_: [::core::ffi::c_char; 50usize],
}
pub type text_t = text_s;
extern "C" {
    pub fn lib_init() -> bool;
}
extern "C" {
    pub fn lib_show_text(p_text: *const text_t) -> bool;
}
extern "C" {
    pub fn lib_show_int32(val: i32) -> bool;
}
extern "C" {
    pub fn lib_deinit();
}
