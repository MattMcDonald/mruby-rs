extern crate mruby_sys;

use mruby_sys::*;

pub struct MRuby{
    mrb_state: *mut mruby_sys::mrb_state
}

impl MRuby {
     pub fn new() -> MRuby{
        unsafe{
            let mrb_state = mrb_open();
            
            MRuby{
                mrb_state : mrb_state
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
    fn can_make_and_drop() {
        let _mruby = MRuby::new();
    }
}
