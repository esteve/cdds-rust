use dds::*;
use std::ffi::CString;
use std::time::Duration;

#[no_mangle]
extern "C" fn sertopic_free(sertopic: *mut libddsc_sys::ddsi_sertopic) {
    libddsc_sys::ddsi_sertopic_fini(sertopic);
}

#[no_mangle]
extern "C" fn sertopic_zero_samples(
    d: *const libddsc_sys::ddsi_sertopic,
    samples: *mut ::std::os::raw::c_void,
    count: libddsc_sys::size_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
extern "C" fn sertopic_realloc_samples(
    ptrs: *mut *mut ::std::os::raw::c_void,
    d: *const libddsc_sys::ddsi_sertopic,
    old: *mut ::std::os::raw::c_void,
    oldcount: libddsc_sys::size_t,
    count: libddsc_sys::size_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
extern "C" fn sertopic_free_samples(
    d: *const libddsc_sys::ddsi_sertopic,
    ptrs: *mut *mut ::std::os::raw::c_void,
    count: libddsc_sys::size_t,
    op: libddsc_sys::dds_free_op_t,
) {
    // TODO(esteve): implement
}

#[no_mangle]
extern "C" fn sertopic_equal(
    a: *const libddsc_sys::ddsi_sertopic,
    b: *const libddsc_sys::ddsi_sertopic,
) -> bool {
    false
}

#[no_mangle]
extern "C" fn sertopic_hash(tp: *const libddsc_sys::ddsi_sertopic) -> u32 {
    0
}

fn main() {
    let participant = Participant::new(DDS_DOMAIN_DEFAULT);
    let mut qos = QoS::new();
    qos.history(History::KeepAll);

    let mut sertopic: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let sertopic_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut sertopic;

    let mut st: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let st_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut st;

    let sertopic_ops = libddsc_sys::ddsi_sertopic_ops {
        free: Some(sertopic_free),
        zero_samples: Some(sertopic_zero_samples),
        realloc_samples: Some(sertopic_realloc_samples),
        free_samples: Some(sertopic_free_samples),
        equal: Some(sertopic_equal),
        hash: Some(sertopic_hash),
    };

    // let sd_ops: libddsc_sys::ddsi_serdata_ops;

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();
    unsafe { libddsc_sys::ddsi_sertopic_init (st, topic_name.as_ptr(), type_name.as_ptr(), &sertopic_ops, &sd_ops, false); }

    // let topic = participant.create_topic_generic(st_ptr, &qos);

    // let writer: Writer = participant.create_writer(&topic, &qos);

    // writer.set_status_mask(DDSStatusId::PublicationMatched);

    // let status: u32 = 0;
    // while 0 != (status & DDSStatusId::PublicationMatched as u32) {
    //     let (rc, status) = writer.get_status_changes();
    //     sleep_for(Duration::from_millis(20));
    // }

    // // regular write (from_sample(DATA) + to_topicless)
    // struct sampletype xs[] = {
    //     { .key = "aap", .value = "banaan" },
    //     { .key = "kolibrie", .value = "nectar" }
    // };
    // for (int j = 0; j < 2; j++)
    // {
    //     for (size_t i = 0; i < sizeof (xs) / sizeof (xs[0]); i++)
    //     {
    //     rc = writer.write_cdr();
    //     CU_ASSERT_FATAL (rc == 0);
    //     }
    // }
}
