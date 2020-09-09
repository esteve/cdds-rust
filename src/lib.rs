extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Duration;

extern crate libddsc_sys;

pub const DDS_DOMAIN_DEFAULT: u32 = 0xffffffffu32;

pub trait Sertopic {
    fn get_sertopic_ptr(&self) -> *mut libddsc_sys::ddsi_sertopic;

    fn free(&self);

    fn zero_samples(&self, samples: *mut ::std::os::raw::c_void, count: libddsc_sys::size_t);

    fn realloc_samples(
        &self,
        ptrs: *mut *mut ::std::os::raw::c_void,
        old: *mut ::std::os::raw::c_void,
        oldcount: libddsc_sys::size_t,
        count: libddsc_sys::size_t,
    );

    fn free_samples(
        &self,
        ptrs: *mut *mut ::std::os::raw::c_void,
        count: libddsc_sys::size_t,
        op: libddsc_sys::dds_free_op_t,
    );

    fn equal(&self, b: *const libddsc_sys::ddsi_sertopic) -> bool;

    fn hash(&self) -> u32;
}

pub enum History {
    KeepLast { n: u32 },
    KeepAll,
}

impl From<History> for libddsc_sys::dds_history_kind_t {
    fn from(history: History) -> Self {
        match history {
            History::KeepLast { n: _u32 } => libddsc_sys::dds_history_kind_DDS_HISTORY_KEEP_LAST,
            History::KeepAll => libddsc_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL,
        }
    }
}

pub enum Durability {
    Volatile,
    TransientLocal,
    Transient,
    Persistent,
}

impl From<Durability> for libddsc_sys::dds_durability_kind_t {
    fn from(durability: Durability) -> Self {
        match durability {
            Durability::Volatile => libddsc_sys::dds_durability_kind_DDS_DURABILITY_VOLATILE,
            Durability::TransientLocal => {
                libddsc_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL
            }
            Durability::Transient => libddsc_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT,
            Durability::Persistent => libddsc_sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT,
        }
    }
}

pub enum Reliability {
    BestEffort,
    Reliable,
}

impl From<Reliability> for libddsc_sys::dds_reliability_kind_t {
    fn from(reliability: Reliability) -> Self {
        match reliability {
            Reliability::BestEffort => {
                libddsc_sys::dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT
            }
            Reliability::Reliable => libddsc_sys::dds_reliability_kind_DDS_RELIABILITY_RELIABLE,
        }
    }
}

pub enum DDSStatusId {
    InconsistentTopic,
    OfferedDeadlineMissed,
    RequestedDeadlineMissed,
    OfferedIncompatibleQos,
    RequestedIncompatibleQos,
    SampleLost,
    SampleRejected,
    DataOnReaders,
    DataAvailable,
    LivelinessLost,
    LivelinessChanged,
    PublicationMatched,
    SubscriptionMatched,
}

impl From<DDSStatusId> for libddsc_sys::dds_status_id_t {
    fn from(dds_status_id: DDSStatusId) -> Self {
        match dds_status_id {
            DDSStatusId::InconsistentTopic => {
                libddsc_sys::dds_status_id_DDS_INCONSISTENT_TOPIC_STATUS_ID
            }
            DDSStatusId::OfferedDeadlineMissed => {
                libddsc_sys::dds_status_id_DDS_OFFERED_DEADLINE_MISSED_STATUS_ID
            }
            DDSStatusId::RequestedDeadlineMissed => {
                libddsc_sys::dds_status_id_DDS_REQUESTED_DEADLINE_MISSED_STATUS_ID
            }
            DDSStatusId::OfferedIncompatibleQos => {
                libddsc_sys::dds_status_id_DDS_OFFERED_INCOMPATIBLE_QOS_STATUS_ID
            }
            DDSStatusId::RequestedIncompatibleQos => {
                libddsc_sys::dds_status_id_DDS_REQUESTED_INCOMPATIBLE_QOS_STATUS_ID
            }
            DDSStatusId::SampleLost => libddsc_sys::dds_status_id_DDS_SAMPLE_LOST_STATUS_ID,
            DDSStatusId::SampleRejected => libddsc_sys::dds_status_id_DDS_SAMPLE_REJECTED_STATUS_ID,
            DDSStatusId::DataOnReaders => libddsc_sys::dds_status_id_DDS_DATA_ON_READERS_STATUS_ID,
            DDSStatusId::DataAvailable => libddsc_sys::dds_status_id_DDS_DATA_AVAILABLE_STATUS_ID,
            DDSStatusId::LivelinessLost => libddsc_sys::dds_status_id_DDS_LIVELINESS_LOST_STATUS_ID,
            DDSStatusId::LivelinessChanged => {
                libddsc_sys::dds_status_id_DDS_LIVELINESS_CHANGED_STATUS_ID
            }
            DDSStatusId::PublicationMatched => {
                libddsc_sys::dds_status_id_DDS_PUBLICATION_MATCHED_STATUS_ID
            }
            DDSStatusId::SubscriptionMatched => {
                libddsc_sys::dds_status_id_DDS_SUBSCRIPTION_MATCHED_STATUS_ID
            }
        }
    }
}

pub struct QoS {
    qos: *mut libddsc_sys::dds_qos_t,
}

impl QoS {
    pub fn new() -> QoS {
        QoS {
            qos: unsafe { libddsc_sys::dds_create_qos() },
        }
    }

    pub fn reset(&mut self) {
        unsafe { libddsc_sys::dds_qos_reset(self.qos) }
    }

    pub fn history<'a>(&'a mut self, history: History) -> &'a mut Self {
        match history {
            History::KeepLast { n } => unsafe {
                libddsc_sys::dds_qset_history(self.qos, history.into(), n as i32)
            },
            History::KeepAll => unsafe {
                libddsc_sys::dds_qset_history(self.qos, history.into(), 0)
            },
        }
        self
    }

    pub fn durability<'a>(&'a mut self, durability: Durability) -> &'a mut Self {
        unsafe { libddsc_sys::dds_qset_durability(self.qos, durability.into()) }
        self
    }

    pub fn reliability<'a>(
        &'a mut self,
        reliability: Reliability,
        duration: &Duration,
    ) -> &'a mut Self {
        unsafe {
            libddsc_sys::dds_qset_reliability(
                self.qos,
                reliability.into(),
                duration.as_nanos() as i64,
            )
        }
        self
    }

    pub fn partitions(&mut self, ps: &[String]) {
        // let mut xs : [*const c_char; ps.len()] = [ std::ptr::null(); ps.len()];
        // let p = CString::new(ps[0]).unwrap().as_ptr();
        let mut cps: Vec<*const c_char> = ps
            .iter()
            .map(|s| CString::new(String::from(s)).unwrap().as_ptr())
            .collect();
        unsafe {
            libddsc_sys::dds_qset_partition(
                self.qos,
                ps.len() as u32,
                cps.as_mut_ptr() as *mut *const ::std::os::raw::c_char,
            )
        }
    }
}

impl PartialEq for QoS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { libddsc_sys::dds_qos_equal(self.qos, other.qos) }
    }
}

impl Eq for QoS {}

impl Clone for QoS {
    fn clone(&self) -> Self {
        let dst = QoS {
            qos: unsafe { libddsc_sys::dds_create_qos() },
        };
        unsafe { libddsc_sys::dds_copy_qos(dst.qos, self.qos as *const libddsc_sys::dds_qos_t) };
        dst
    }
}

impl Drop for QoS {
    fn drop(&mut self) {
        unsafe { libddsc_sys::dds_qos_delete(self.qos) };
    }
}

pub struct Participant {
    entity: libddsc_sys::dds_entity_t,
}

impl Drop for Participant {
    fn drop(&mut self) {
        unsafe { libddsc_sys::dds_delete(self.entity) };
    }
}

impl Participant {
    pub fn new(d: libddsc_sys::dds_domainid_t) -> Participant {
        let e =
            unsafe { libddsc_sys::dds_create_participant(d, std::ptr::null(), std::ptr::null()) };
        Participant { entity: e }
    }

    pub fn create_publisher(&self, qos: &QoS) -> Publisher {
        let e =
            unsafe { libddsc_sys::dds_create_publisher(self.entity, qos.qos, std::ptr::null()) };
        Publisher { entity: e }
    }

    pub fn create_writer(&self, topic: &Topic, qos: &QoS) -> Writer {
        let e = unsafe {
            libddsc_sys::dds_create_writer(self.entity, topic.entity, qos.qos, std::ptr::null())
        };
        Writer { entity: e }
    }

    pub fn create_topic(&self, name: &str, qos: &QoS) -> Topic {
        let e = unsafe {
            libddsc_sys::dds_create_topic(
                self.entity,
                std::ptr::null(),
                CString::new(name).unwrap().as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };
        Topic { entity: e }
    }

    pub fn create_topic_generic(
        &self,
        sertopic: *mut *mut libddsc_sys::ddsi_sertopic,
        qos: &QoS,
    ) -> Topic {
        let e = unsafe {
            libddsc_sys::dds_create_topic_generic(
                self.entity,
                sertopic,
                qos.qos,
                std::ptr::null(),
                std::ptr::null(),
            )
        };
        Topic { entity: e }
    }
}

pub struct Publisher {
    entity: libddsc_sys::dds_entity_t,
}

pub struct Writer {
    entity: libddsc_sys::dds_entity_t,
}

impl Writer {
    pub fn set_status_mask(&self, mask: DDSStatusId) -> libddsc_sys::dds_return_t {
        unsafe { libddsc_sys::dds_set_status_mask(self.entity, mask as u32) }
    }

    pub fn get_status_changes(&self) -> (libddsc_sys::dds_return_t, u32) {
        let mut status: u32 = 0;
        let status_ptr: *mut u32 = &mut status;
        let rc: libddsc_sys::dds_return_t =
            unsafe { libddsc_sys::dds_get_status_changes(self.entity, status_ptr) };
        (rc, status)
    }

    pub fn write_cdr(&self, cdr: *mut libddsc_sys::ddsi_serdata) -> libddsc_sys::dds_return_t {
        unsafe { libddsc_sys::dds_writecdr(self.entity, cdr) }
    }
}

pub struct Topic {
    entity: libddsc_sys::dds_entity_t,
}

pub fn sleep_for(duration: Duration) {
    let nanos = duration.as_nanos() as i64;
    unsafe {
        libddsc_sys::dds_sleepfor(nanos);
    }
}
