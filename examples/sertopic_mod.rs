use dds::*;

#[no_mangle]
pub extern "C" fn sertopic_free(sertopic: *mut libddsc_sys::ddsi_sertopic) {
    libddsc_sys::ddsi_sertopic_fini(sertopic);
}

#[no_mangle]
pub extern "C" fn sertopic_zero_samples(
    d: *const libddsc_sys::ddsi_sertopic,
    samples: *mut ::std::os::raw::c_void,
    count: libddsc_sys::size_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn sertopic_realloc_samples(
    ptrs: *mut *mut ::std::os::raw::c_void,
    d: *const libddsc_sys::ddsi_sertopic,
    old: *mut ::std::os::raw::c_void,
    oldcount: libddsc_sys::size_t,
    count: libddsc_sys::size_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn sertopic_free_samples(
    d: *const libddsc_sys::ddsi_sertopic,
    ptrs: *mut *mut ::std::os::raw::c_void,
    count: libddsc_sys::size_t,
    op: libddsc_sys::dds_free_op_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn sertopic_equal(
    a: *const libddsc_sys::ddsi_sertopic,
    b: *const libddsc_sys::ddsi_sertopic,
) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn sertopic_hash(tp: *const libddsc_sys::ddsi_sertopic) -> u32 {
    0
}
