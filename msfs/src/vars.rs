//! Bindings to the  vars api for msfs2024

use crate::sys;
use crate::sim_param_array::{with_params, FsParam};

pub trait SimVarF64 {
    fn to(self) -> f64;
    fn from(v: f64) -> Self;
}

impl SimVarF64 for f64 {
    fn to(self) -> f64 {
        self
    }

    fn from(v: f64) -> Self {
        v
    }
}

impl SimVarF64 for bool {
    fn to(self) -> f64 {
        if self { 1.0 } else { 0.0 }
    }

    fn from(v: f64) -> Self {
        v != 0.0
    }
}

impl SimVarF64 for u8 {
    fn to(self) -> f64 {
        self as f64
    }

    fn from(v: f64) -> Self {
        v as Self
    }
}

pub struct AircraftVariableApi {simvar: sys::FsSimVarId , units: sys::FsUnitId, index: u32, name: String}

impl AircraftVariableApi {
    pub fn from(name: &str, units: &str, index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let name_cstr = std::ffi::CString::new(name).unwrap();
        let units_cstr = std::ffi::CString::new(units).unwrap();
        let var = unsafe { let result = sys::fsVarsGetAVarId(name_cstr.as_ptr());
            if result == -1 {
                println!("Error getting aircraft var id for {} with error {}", name, result);
            }
            result
         };
        let unit = unsafe { let result = sys::fsVarsGetUnitId(units_cstr.as_ptr());
            if result == -1 {
                println!("Error getting unit id for {} with error {}", units, result);
            }
            result
         }; 
      
        Ok(Self {
            simvar: var,
            units: unit,
            index: index,
            name: name.to_string()
        })

    }

    pub fn get<T: SimVarF64>(&self) -> T {
        let mut v = 0.0;

        with_params(&[FsParam::Integer(self.index)], |params_for_get| unsafe {
            sys::fsVarsAVarGet(self.simvar, self.units, params_for_get, &mut v, sys::FS_OBJECT_ID_USER_AIRCRAFT);
        });



        T::from(v)
    }

     pub fn set(&self, value: impl SimVarF64) {

        let v: f64 = value.to();

        with_params(&[FsParam::Integer(self.index)], |params_for_set| unsafe { 
            let retval = sys::fsVarsAVarSet(self.simvar, self.units, params_for_set, v, sys::FS_OBJECT_ID_USER_AIRCRAFT);

            if retval != 0 {
                println!("Error setting aircraft var: {:?} for {:?} : {:?}, value {:?}", retval, self.name, self.index, v);
            }
        });

        
    } 
}