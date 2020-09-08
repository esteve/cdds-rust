use dds::*;

#[no_mangle]
pub extern "C" fn sertopic_free(sertopic: *mut libddsc_sys::ddsi_sertopic) {
    // TODO(esteve): implement
    unsafe {
        libddsc_sys::ddsi_sertopic_fini(sertopic);
    }
}

#[no_mangle]
pub extern "C" fn sertopic_zero_samples(
    d: *const libddsc_sys::ddsi_sertopic,
    samples: *mut ::std::os::raw::c_void,
    count: libddsc_sys::size_t,
) {
    unsafe { std::ptr::write_bytes(samples, 0, count as usize); }
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
    true
}

#[no_mangle]
pub extern "C" fn sertopic_hash(tp: *const libddsc_sys::ddsi_sertopic) -> u32 {
    0
}

use dds::*;

#[no_mangle]
pub extern "C" fn serdata_get_size(d: *const libddsc_sys::ddsi_serdata) -> u32 {
    // TODO(esteve): implement
    0
}

#[no_mangle]
pub extern "C" fn serdata_free(d: *mut libddsc_sys::ddsi_serdata) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn serdata_from_ser(
    topic: *const libddsc_sys::ddsi_sertopic,
    kind: libddsc_sys::ddsi_serdata_kind,
    fragchain: *const libddsc_sys::nn_rdata,
    size: libddsc_sys::size_t,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_from_ser_iov(
    topic: *const libddsc_sys::ddsi_sertopic,
    kind: libddsc_sys::ddsi_serdata_kind,
    niov: libddsc_sys::ddsrt_msg_iovlen_t,
    iov: *const libddsc_sys::ddsrt_iovec_t,
    size: libddsc_sys::size_t,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_from_keyhash(
    topic: *const libddsc_sys::ddsi_sertopic,
    keyhash: *const libddsc_sys::nn_keyhash,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_from_sample(
    topic: *const libddsc_sys::ddsi_sertopic,
    kind: libddsc_sys::ddsi_serdata_kind,
    sample: *const ::std::os::raw::c_void,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_to_topicless(
    d: *const libddsc_sys::ddsi_serdata,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_to_ser(
    d: *const libddsc_sys::ddsi_serdata,
    off: libddsc_sys::size_t,
    sz: libddsc_sys::size_t,
    buf: *mut ::std::os::raw::c_void,
) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn serdata_to_ser_ref(
    d: *const libddsc_sys::ddsi_serdata,
    off: libddsc_sys::size_t,
    sz: libddsc_sys::size_t,
    ref_: *mut libddsc_sys::ddsrt_iovec_t,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn serdata_to_ser_unref(
    d: *mut libddsc_sys::ddsi_serdata,
    ref_: *const libddsc_sys::ddsrt_iovec_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
pub extern "C" fn serdata_to_sample(
    d: *const libddsc_sys::ddsi_serdata,
    sample: *mut ::std::os::raw::c_void,
    bufptr: *mut *mut ::std::os::raw::c_void,
    buflim: *mut ::std::os::raw::c_void,
) -> bool {
    // TODO(esteve): implement
    false
}

#[no_mangle]
pub extern "C" fn serdata_topicless_to_sample(
    topic: *const libddsc_sys::ddsi_sertopic,
    d: *const libddsc_sys::ddsi_serdata,
    sample: *mut ::std::os::raw::c_void,
    bufptr: *mut *mut ::std::os::raw::c_void,
    buflim: *mut ::std::os::raw::c_void,
) -> bool {
    // TODO(esteve): implement
    false
}

#[no_mangle]
pub extern "C" fn serdata_eqkey(
    a: *const libddsc_sys::ddsi_serdata,
    b: *const libddsc_sys::ddsi_serdata,
) -> bool {
    // TODO(esteve): implement
    false
}

#[no_mangle]
pub extern "C" fn serdata_print(
    topic: *const libddsc_sys::ddsi_sertopic,
    d: *const libddsc_sys::ddsi_serdata,
    buf: *mut ::std::os::raw::c_char,
    size: libddsc_sys::size_t,
) -> libddsc_sys::size_t {
    // TODO(esteve): implement
    0
}
