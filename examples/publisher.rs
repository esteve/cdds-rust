use dds::*;
use std::ffi::CString;
use std::time::Duration;
mod examples_common;
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let participant = Participant::new(DDS_DOMAIN_DEFAULT);
    let mut qos = QoS::new();
    qos.history(History::KeepAll);

    let layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr = unsafe { alloc(layout) as *mut libddsc_sys::ddsi_sertopic };

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
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &sertopic_ops,
            &serdata_ops,
            false,
        );
    }

    let topic = participant.create_topic_generic(&mut sertopic_ptr, &qos);

    let writer: Writer = participant.create_writer(&topic, &qos);

    writer.set_status_mask(DDSStatusId::PublicationMatched);

    loop {
        println!("Writing CDR");
        let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
        let serdata_ptr = unsafe { alloc(layout) as *mut libddsc_sys::ddsi_serdata };
        unsafe {
            libddsc_sys::ddsi_serdata_init(
                serdata_ptr,
                sertopic_ptr,
                libddsc_sys::ddsi_serdata_kind_SDK_DATA,
            );
        };
        let rc = writer.write_cdr(serdata_ptr);
        sleep_for(Duration::from_millis(200));
    }
}
