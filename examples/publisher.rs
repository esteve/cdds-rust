use dds::*;
use std::ffi::CString;
use std::time::Duration;
mod sertopic_mod;

fn main() {
    let participant = Participant::new(DDS_DOMAIN_DEFAULT);
    let mut qos = QoS::new();
    qos.history(History::KeepAll);

    let mut sertopic: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let sertopic_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut sertopic;

    let mut st: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let st_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut st;

    let sertopic_ops = libddsc_sys::ddsi_sertopic_ops {
        free: Some(sertopic_mod::sertopic_free),
        zero_samples: Some(sertopic_mod::sertopic_zero_samples),
        realloc_samples: Some(sertopic_mod::sertopic_realloc_samples),
        free_samples: Some(sertopic_mod::sertopic_free_samples),
        equal: Some(sertopic_mod::sertopic_equal),
        hash: Some(sertopic_mod::sertopic_hash),
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
