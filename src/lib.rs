#[macro_use]
extern crate simple_error;
extern crate mruby_sys;

use std::error::Error;
use mruby_sys::*;

#[derive(Debug)]
pub struct MRuby{
    mrb_state: *mut mruby_sys::mrb_state
}


#[derive(Debug, Clone)]
pub enum MRubyValue {
    Fixnum(i64),
    String(String)
}


impl MRuby {
     pub fn new() -> Result<MRuby, ()>{
        unsafe{
            let mrb_state = mrb_open();
            if mrb_state.is_null(){
                return Err(())
            }
            Ok(MRuby{
                mrb_state : mrb_state
            })
        }       
    }

    pub fn load_string(&mut self, string: &str) -> Result<MRubyValue, Box<Error>>{
        unsafe{
            let c_string = std::ffi::CString::new(string)?;
            let mut result=  mrb_load_string(self.mrb_state, c_string.as_ptr());
            match result.tt {
                mruby_sys::mrb_vtype_MRB_TT_FIXNUM  => {
                     Ok(MRubyValue::Fixnum(result.value.i))
                },
                mruby_sys::mrb_vtype_MRB_TT_STRING  => {
                    let ptr = mruby_sys::mrb_string_value_cstr(self.mrb_state, &mut result);
                    let c_string = std::ffi::CStr::from_ptr(ptr as *const i8);
                    let string = c_string.to_str()?;
                    Ok(MRubyValue::String(string.to_string()))
                }
                _ => bail!("Can't convert type from Ruby to Rust")
            }
        }
    }
}

impl Drop for MRuby {
    fn drop(&mut self){
        unsafe{
            mrb_close(self.mrb_state)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_new_and_drop() {
        let _mruby = MRuby::new().expect("Can't make a new mruby");
    }

    #[test]
    fn can_add_1_and_1() -> Result<(), Box<Error>>{
        let mut mruby = MRuby::new().expect("Can't make a new mruby");
        let result =mruby.load_string("1 + 1")?;
        
        let val = match result {
            MRubyValue::Fixnum(i) => i,
            _ => -1
        };
        assert_eq!(2, val);
        Ok(())
    }

   #[test]
    fn can_get_a_string() -> Result<(), Box<Error>>{
        let mut mruby = MRuby::new().expect("Can't make a new mruby");
        let result = mruby.load_string("'hello'")?;
        
        let val = match result {
            MRubyValue::String(s) => s,
            _ => bail!("failed")
        };
        assert_eq!("hello", val);
        Ok(())
    }


    #[test]
    fn returns_error_if_unsupported_type() -> Result<(), Box<Error>>{
        let mut mruby = MRuby::new().expect("Can't make a new mruby");
        mruby.load_string("1.1").expect_err("Currently we only support ints");
        Ok(())
    }
}
