#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sg_bindings_work() {
        unsafe {
            sg_init(1);
            let p: &mut u64 = &mut 0;
            let general_stats = sg_get_host_info(p);

            let os_str = std::ffi::CStr::from_ptr((*general_stats).os_name).to_string_lossy().into_owned();

            // TODO - find a better metric to compare against
            assert_eq!(os_str, "Linux");

            sg_shutdown();
        };
    }
}
