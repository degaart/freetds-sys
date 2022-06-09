#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ptr;

    #[test]
    fn test_ct() {
        unsafe {
            let mut ctx: *mut CS_CONTEXT = ptr::null_mut();
            let ret = cs_ctx_alloc(CS_VERSION_125, &mut ctx);
            assert_eq!(ret, CS_SUCCEED);

            let ret = ct_init(ctx, CS_VERSION_125);
            assert_eq!(ret, CS_SUCCEED);

            let ret = ct_exit(ctx, CS_UNUSED);
            assert_eq!(ret, CS_SUCCEED);
            
            let ret = cs_ctx_drop(ctx);
            assert_eq!(ret, CS_SUCCEED);
        }
    }
}
