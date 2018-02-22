use libucl_sys::*;

use super::Object;

use utils;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Emitter {
    JSON,
    JSONCompact,
    Config,
    YAML,
	MSGPACK,
	MAX
}

impl Emitter {
    pub fn emit<T: AsRef<Object>>(&self, obj: T) -> Option<String> {
        let emit = unsafe { ucl_object_emit(obj.as_ref().obj, Into::into(*self)) };
        utils::to_str(emit)
    }
}

impl From<ucl_emitter> for Emitter {
    fn from(raw: ucl_emitter) -> Self {
        match raw {
            ucl_emitter::UCL_EMIT_JSON         => Emitter::JSON,
            ucl_emitter::UCL_EMIT_JSON_COMPACT => Emitter::JSONCompact,
            ucl_emitter::UCL_EMIT_CONFIG       => Emitter::Config,
            ucl_emitter::UCL_EMIT_YAML         => Emitter::YAML,
            ucl_emitter::UCL_EMIT_MSGPACK      => Emitter::MSGPACK,
            ucl_emitter::UCL_EMIT_MAX          => Emitter::MAX
        }
    }
}

impl Into<ucl_emitter> for Emitter {
    fn into(self) -> ucl_emitter {
        match self {
            Emitter::JSON        => ucl_emitter::UCL_EMIT_JSON,
            Emitter::JSONCompact => ucl_emitter::UCL_EMIT_JSON_COMPACT,
            Emitter::Config      => ucl_emitter::UCL_EMIT_CONFIG,
            Emitter::YAML        => ucl_emitter::UCL_EMIT_YAML,
            Emitter::MSGPACK     => ucl_emitter::UCL_EMIT_MSGPACK,
            Emitter::MAX         => ucl_emitter::UCL_EMIT_MAX
        }
    }
}
