use dds::*;
use std::ffi::CString;
use std::time::Duration;
mod examples_common;

fn main() {
    let participant = Participant::new(DDS_DOMAIN_DEFAULT);
    let mut qos = QoS::new();
    qos.history(History::KeepAll);

    let mut sertopic: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let sertopic_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut sertopic;

    let se = examples_common::SertopicExample {sertopic_ptr: sertopic};

    let f = |sertopic: *mut libddsc_sys::ddsi_sertopic| { se.free() };

    let f1 : extern "C" fn(*mut libddsc_sys::ddsi_sertopic) = f;
    let sertopic_ops = libddsc_sys::ddsi_sertopic_ops {
        free: Some(examples_common::sertopic_free),
        zero_samples: Some(examples_common::sertopic_zero_samples),
        realloc_samples: Some(examples_common::sertopic_realloc_samples),
        free_samples: Some(examples_common::sertopic_free_samples),
        equal: Some(examples_common::sertopic_equal),
        hash: Some(examples_common::sertopic_hash),
    };

    let serdata_ops = libddsc_sys::ddsi_serdata_ops {
        get_size: Some(examples_common::serdata_get_size),
        eqkey: Some(examples_common::serdata_eqkey),
        free: Some(examples_common::serdata_free),
        from_ser: Some(examples_common::serdata_from_ser),
        from_ser_iov: Some(examples_common::serdata_from_ser_iov),
        from_keyhash: Some(examples_common::serdata_from_keyhash),
        from_sample: Some(examples_common::serdata_from_sample),
        to_ser: Some(examples_common::serdata_to_ser),
        to_sample: Some(examples_common::serdata_to_sample),
        to_ser_ref: Some(examples_common::serdata_to_ser_ref),
        to_ser_unref: Some(examples_common::serdata_to_ser_unref),
        to_topicless: Some(examples_common::serdata_to_topicless),
        topicless_to_sample: Some(examples_common::serdata_topicless_to_sample),
        print: Some(examples_common::serdata_print),
    };

    let topic_name = CString::new("ddsc_cdr_basic").unwrap();
    let type_name = CString::new("x").unwrap();
    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &sertopic_ops,
            &serdata_ops,
            false,
        );
    }

    let topic = participant.create_topic_generic(sertopic_ptr, &qos);

    let writer: Writer = participant.create_writer(&topic, &qos);

    writer.set_status_mask(DDSStatusId::PublicationMatched);

    let status: u32 = 0;
    while 0 != (status & DDSStatusId::PublicationMatched as u32) {
        let (rc, status) = writer.get_status_changes();
        sleep_for(Duration::from_millis(20));
    }
}
