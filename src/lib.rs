#[macro_use]
extern crate simple_error;
extern crate mruby_sys;

use std::error::Error;
use mruby_sys::*;

pub struct MRuby{
    mrb_state: *mut mruby_sys::mrb_state
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

    pub fn load_string(&mut self, string: &str) -> Result<i64, Box<Error>>{
        unsafe{
            let c_string = std::ffi::CString::new(string)?;
            let result=  mrb_load_string(self.mrb_state, c_string.as_ptr());
            match result.tt {
                mruby_sys::mrb_vtype_MRB_TT_FIXNUM  => {
                    return Ok(result.value.i)
                }
                _ => bail!("Only fixnums are supported right now")
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
        let result = mruby.load_string("1 + 1")?;
        assert_eq!(2, result);
        Ok(())
    }


    #[test]
    fn returns_error_if_unsupported_type() -> Result<(), Box<Error>>{
        let mut mruby = MRuby::new().expect("Can't make a new mruby");
        mruby.load_string("'2'").expect_err("Currently we only support ints");
        mruby.load_string("1.1").expect_err("Currently we only support ints");
        Ok(())
    }
}
