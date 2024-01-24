#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
extern crate cudarc;
include!("./bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_libsmctrl_set_global_mask() {
        unsafe { libsmctrl_set_global_mask(0) };
    }
}
