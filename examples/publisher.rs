use dds::*;
use std::time::Duration;

fn main() {
    let participant = Participant::new(0);
    let mut qos = QoS::new();
    qos.history(&History::KeepAll);

    let mut sertopic: *mut libddsc_sys::ddsi_sertopic = std::ptr::null_mut();
    let sertopic_ptr: *mut *mut libddsc_sys::ddsi_sertopic = &mut sertopic;
    let topic = participant.create_topic_generic("hello", sertopic_ptr, &qos);

    let writer: Writer = participant.create_writer(&topic, &qos);

    writer.set_status_mask(libddsc_sys::dds_status_id_DDS_PUBLICATION_MATCHED_STATUS_ID);

    let status: u32 = 0;
    while (0 != (status & libddsc_sys::dds_status_id_DDS_PUBLICATION_MATCHED_STATUS_ID)) {
        let (rc, status) = writer.get_status_changes();
        sleep_for(Duration::from_millis(20));
    }
}
