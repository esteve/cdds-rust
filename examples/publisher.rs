use dds::*;
use std::ffi::CString;
use std::time::Duration;
mod examples_common;
use std::alloc::{alloc, Layout};

fn main() {
    let participant = Participant::new(DDS_DOMAIN_DEFAULT);
    let mut qos = QoS::new();
    qos.history(History::KeepAll);

    let topic_name = CString::new("ddsc_cdr_basic1_pid1_tid1").unwrap();
    let type_name = CString::new("x").unwrap();

    let sertopic_layout: std::alloc::Layout = Layout::new::<libddsc_sys::ddsi_sertopic>();
    let mut sertopic_ptr: *mut libddsc_sys::ddsi_sertopic =
        unsafe { alloc(sertopic_layout) } as *mut libddsc_sys::ddsi_sertopic;

    unsafe {
        libddsc_sys::ddsi_sertopic_init(
            sertopic_ptr,
            topic_name.as_ptr(),
            type_name.as_ptr(),
            &examples_common::SERTOPIC_OPS,
            &examples_common::SERDATA_OPS,
            false,
        );
    }

    let topic = participant.create_topic_generic(&mut sertopic_ptr, &qos);

    let writer: Writer = participant.create_writer(&topic, &qos);

    let xs = vec![
        examples_common::SampleType { key: "aap", value: "banaan"},
        examples_common::SampleType { key: "aap", value: "banaan"},
    ];

    loop {
        println!("Writing CDR");
        let serdata_layout = Layout::new::<libddsc_sys::ddsi_serdata>();
        let serdata_ptr = unsafe { alloc(serdata_layout) } as *mut libddsc_sys::ddsi_serdata;
        unsafe {
            libddsc_sys::ddsi_serdata_init(
                serdata_ptr,
                sertopic_ptr,
                libddsc_sys::ddsi_serdata_kind_SDK_KEY,
            );
        };
        let rc = writer.write(serdata_ptr as *const ::std::os::raw::c_void);
        sleep_for(Duration::from_millis(200));
    }
}
