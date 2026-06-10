use crate::{sim_param_array::{FsParam, with_params}, sys};

pub fn fs_events_trigger_key_event(event_id: sys::FsEventId, value0: sys::UINT32, value1:sys::UINT32) {
    with_params(
        &[FsParam::Integer(value0), FsParam::Integer(value1)],
        |params_for_get| unsafe {
            sys::fsEventsTriggerKeyEvent(event_id, params_for_get);
        },
    );
} 