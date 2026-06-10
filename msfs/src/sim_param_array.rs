use crate::sys;

#[derive(Copy, Clone, Debug)]
pub(crate) enum FsParam<'a> {
    Integer(u32),
    #[allow(dead_code)]
    String(&'a std::ffi::CStr),
    #[allow(dead_code)]
    Crc(sys::FsCRC),
}

impl FsParam<'_> {
    fn to_variant(self) -> sys::FsVarParamVariant {
        match self {
            Self::Integer(v) => sys::FsVarParamVariant {
                type_: sys::eFsVarParamType_FsVarParamTypeInteger,
                __bindgen_anon_1: sys::FsVarParamVariant__bindgen_ty_1 { intValue: v },
            },
            Self::String(v) => sys::FsVarParamVariant {
                type_: sys::eFsVarParamType_FsVarParamTypeString,
                __bindgen_anon_1: sys::FsVarParamVariant__bindgen_ty_1 {
                    stringValue: v.as_ptr(),
                },
            },
            Self::Crc(v) => sys::FsVarParamVariant {
                type_: sys::eFsVarParamType_FsVarParamTypeCRC,
                __bindgen_anon_1: sys::FsVarParamVariant__bindgen_ty_1 { CRCValue: v },
            },
        }
    }
}

pub(crate) fn with_params<R>(params: &[FsParam<'_>], f: impl FnOnce(sys::FsVarParamArray) -> R) -> R {
    let mut variants: Vec<sys::FsVarParamVariant> =
        params.iter().copied().map(FsParam::to_variant).collect();

    let array = if variants.is_empty() {
        std::ptr::null_mut()
    } else {
        variants.as_mut_ptr()
    };

    let ffi_params = sys::FsVarParamArray {
        size: variants.len() as u32,
        array,
    };

    f(ffi_params)
}