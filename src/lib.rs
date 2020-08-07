extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Duration;

extern crate libddsc_sys;

pub enum History {
    KeepLast { n: u32 },
    KeepAll,
}

pub enum Durability {
    Volatile,
    TransientLocal,
    Transient,
    Persistent,
}

pub enum Reliability {
    BestEffort,
    Reliable,
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

    pub fn history(&mut self, history: &History) -> &mut Self {
        match history {
            History::KeepLast { n } => unsafe {
                libddsc_sys::dds_qset_history(
                    self.qos,
                    libddsc_sys::dds_history_kind_DDS_HISTORY_KEEP_LAST,
                    *n as i32,
                )
            },
            History::KeepAll => unsafe {
                libddsc_sys::dds_qset_history(
                    self.qos,
                    libddsc_sys::dds_history_kind_DDS_HISTORY_KEEP_ALL,
                    0,
                )
            },
        }
        self
    }

    pub fn durability(&mut self, durability: &Durability) -> &mut Self {
        match durability {
            Durability::Volatile => unsafe {
                libddsc_sys::dds_qset_durability(
                    self.qos,
                    libddsc_sys::dds_durability_kind_DDS_DURABILITY_VOLATILE
                )
            },
            Durability::TransientLocal => unsafe {
                libddsc_sys::dds_qset_durability(
                    self.qos,
                    libddsc_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL
                )
            },
            Durability::Transient => unsafe {
                libddsc_sys::dds_qset_durability(
                    self.qos,
                    libddsc_sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT
                )
            },
            Durability::Persistent => unsafe {
                libddsc_sys::dds_qset_durability(
                    self.qos,
                    libddsc_sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT
                )
            },
        }
        self
    }

    pub fn reliability(&mut self, reliability: &Reliability, duration: &Duration) -> &mut Self {
        match reliability {
            Reliability::BestEffort => unsafe {
                libddsc_sys::dds_qset_reliability(
                    self.qos,
                    libddsc_sys::dds_reliability_kind_DDS_RELIABILITY_BEST_EFFORT,
                    duration.as_nanos() as i64,
                )
            },
            Reliability::Reliable => unsafe {
                libddsc_sys::dds_qset_reliability(
                    self.qos,
                    libddsc_sys::dds_reliability_kind_DDS_RELIABILITY_RELIABLE,
                    duration.as_nanos() as i64,
                )
            },
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
        let e = unsafe { libddsc_sys::dds_create_publisher(self.entity, qos.qos, std::ptr::null()) };
        Publisher { entity: e }
    }

    pub fn create_writer(&self, topic: &Topic, qos: &QoS) -> Writer {
        let e = unsafe { libddsc_sys::dds_create_writer(self.entity, topic.entity, qos.qos, std::ptr::null()) };
        Writer { entity: e }
    }

    pub fn create_topic(&self, name: &str, qos: &QoS) -> Topic {
        let e = unsafe { libddsc_sys::dds_create_topic(self.entity, std::ptr::null(), CString::new(name).unwrap().as_ptr(), std::ptr::null(), std::ptr::null()) };
        Topic { entity: e }
    }
}

pub struct Publisher {
    entity: libddsc_sys::dds_entity_t,
}

pub struct Writer {
    entity: libddsc_sys::dds_entity_t,
}

pub struct Topic {
    entity: libddsc_sys::dds_entity_t,
}
