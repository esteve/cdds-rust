use dds::*;
use std::alloc::{alloc, Layout};
use std::ffi::CString;

pub static SERTOPIC_OPS: libddsc_sys::ddsi_sertopic_ops = libddsc_sys::ddsi_sertopic_ops {
    free: Some(sertopic_free),
    zero_samples: Some(sertopic_zero_samples),
    realloc_samples: Some(sertopic_realloc_samples),
    free_samples: Some(sertopic_free_samples),
    equal: Some(sertopic_equal),
    hash: Some(sertopic_hash),
};

pub static SERDATA_OPS: libddsc_sys::ddsi_serdata_ops = libddsc_sys::ddsi_serdata_ops {
    get_size: Some(serdata_get_size),
    eqkey: Some(serdata_eqkey),
    free: Some(serdata_free),
    from_ser: Some(serdata_from_ser),
    from_ser_iov: Some(serdata_from_ser_iov),
    from_keyhash: Some(serdata_from_keyhash),
    from_sample: Some(serdata_from_sample),
    to_ser: Some(serdata_to_ser),
    to_sample: Some(serdata_to_sample),
    to_ser_ref: Some(serdata_to_ser_ref),
    to_ser_unref: Some(serdata_to_ser_unref),
    to_topicless: Some(serdata_to_topicless),
    topicless_to_sample: Some(serdata_topicless_to_sample),
    print: Some(serdata_print),
};

#[repr(C)]
struct sampletype {
    key: &'static str,
    value: &'static str,
}

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
    unsafe {
        std::ptr::write_bytes::<sampletype>(samples as *mut sampletype, 0, count as usize);
    }
}

#[no_mangle]
pub extern "C" fn sertopic_realloc_samples(
    ptrs: *mut *mut ::std::os::raw::c_void,
    d: *const libddsc_sys::ddsi_sertopic,
    old: *mut ::std::os::raw::c_void,
    oldcount: libddsc_sys::size_t,
    count: libddsc_sys::size_t,
) {
    let size: libddsc_sys::size_t = std::mem::size_of::<sampletype>() as libddsc_sys::size_t;
    let new = if oldcount == count {
        old
    } else {
        unsafe { libddsc_sys::dds_realloc(old, size * count) }
    };
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
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
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
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
}

#[no_mangle]
pub extern "C" fn serdata_from_keyhash(
    topic: *const libddsc_sys::ddsi_sertopic,
    keyhash: *const libddsc_sys::nn_keyhash,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
}

#[no_mangle]
pub extern "C" fn serdata_from_sample(
    topic: *const libddsc_sys::ddsi_sertopic,
    kind: libddsc_sys::ddsi_serdata_kind,
    sample: *const ::std::os::raw::c_void,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
}

#[no_mangle]
pub extern "C" fn serdata_to_topicless(
    d: *const libddsc_sys::ddsi_serdata,
) -> *mut libddsc_sys::ddsi_serdata {
    // TODO(esteve): implement
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
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
    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &SERTOPIC_OPS,
            &SERDATA_OPS,
            false,
        );
    }

    let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
    let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;

    unsafe {
        libddsc_sys::ddsi_serdata_init(
            serdata_ptr,
            sertopic_ptr,
            libddsc_sys::ddsi_serdata_kind_SDK_KEY,
        );
    };
    serdata_ptr
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
    true
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

pub struct SertopicExample {
    pub sertopic_ptr: *mut libddsc_sys::ddsi_sertopic,
}

impl Sertopic for SertopicExample {
    fn get_sertopic_ptr(&self) -> *mut libddsc_sys::ddsi_sertopic {
        self.sertopic_ptr
    }

    fn free(&self) {
        sertopic_free(self.get_sertopic_ptr())
    }

    fn zero_samples(&self, samples: *mut ::std::os::raw::c_void, count: libddsc_sys::size_t) {
        sertopic_zero_samples(self.get_sertopic_ptr(), samples, count)
    }

    fn realloc_samples(
        &self,
        ptrs: *mut *mut ::std::os::raw::c_void,
        old: *mut ::std::os::raw::c_void,
        oldcount: libddsc_sys::size_t,
        count: libddsc_sys::size_t,
    ) {
        sertopic_realloc_samples(ptrs, self.get_sertopic_ptr(), old, oldcount, count)
    }

    fn free_samples(
        &self,
        ptrs: *mut *mut ::std::os::raw::c_void,
        count: libddsc_sys::size_t,
        op: libddsc_sys::dds_free_op_t,
    ) {
        sertopic_free_samples(self.get_sertopic_ptr(), ptrs, count, op)
    }

    fn equal(&self, b: *const libddsc_sys::ddsi_sertopic) -> bool {
        sertopic_equal(self.get_sertopic_ptr(), b)
    }

    fn hash(&self) -> u32 {
        sertopic_hash(self.get_sertopic_ptr())
    }
}
