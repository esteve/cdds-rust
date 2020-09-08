use dds::*;
use std::ffi::CString;
use std::time::Duration;
mod serdata_mod;
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

    let serdata_ops = libddsc_sys::ddsi_serdata_ops {
        get_size: Some(serdata_mod::serdata_get_size),
        eqkey: Some(serdata_mod::serdata_eqkey),
        free: Some(serdata_mod::serdata_free),
        from_ser: Some(serdata_mod::serdata_from_ser),
        from_ser_iov: Some(serdata_mod::serdata_from_ser_iov),
        from_keyhash: Some(serdata_mod::serdata_from_keyhash),
        from_sample: Some(serdata_mod::serdata_from_sample),
        to_ser: Some(serdata_mod::serdata_to_ser),
        to_sample: Some(serdata_mod::serdata_to_sample),
        to_ser_ref: Some(serdata_mod::serdata_to_ser_ref),
        to_ser_unref: Some(serdata_mod::serdata_to_ser_unref),
        to_topicless: Some(serdata_mod::serdata_to_topicless),
        topicless_to_sample: Some(serdata_mod::serdata_topicless_to_sample),
        print: Some(serdata_mod::serdata_print),
    };

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();
    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            st,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &sertopic_ops,
            &serdata_ops,
            false,
        );
    }

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
