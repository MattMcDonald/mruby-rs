use mruby_sys::*;
use std::error::Error;

pub type ValueResult = Result<Value, Box<Error>>;

#[derive(Debug)]
pub struct State {
    mrb_state: *mut mruby_sys::mrb_state,
}

#[derive(Debug, Clone)]
pub enum Value {
    Fixnum(i64),
    String(String),
}

impl State {
    pub fn new() -> Result<State, ()> {
        unsafe {
            let mrb_state = mrb_open();
            if mrb_state.is_null() {
                return Err(());
            }
            Ok(State {
                mrb_state: mrb_state,
            })
        }
    }

    pub fn load_string(&mut self, string: &str) -> ValueResult {
        unsafe {
            let c_string = std::ffi::CString::new(string)?;
            let mut mrb_value = mrb_load_string(self.mrb_state, c_string.as_ptr());

            Ok(match mrb_value.tt {
                mruby_sys::mrb_vtype_MRB_TT_FIXNUM => Value::Fixnum(mrb_value.value.i),
                mruby_sys::mrb_vtype_MRB_TT_STRING => {
                    let ptr = mruby_sys::mrb_string_value_cstr(self.mrb_state, &mut mrb_value);
                    let c_string = std::ffi::CStr::from_ptr(ptr as *const i8);
                    let string = c_string.to_str()?;
                    Value::String(string.to_string())
                }
                _ => bail!("Can't convert type from Ruby to Rust"),
            })
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { mrb_close(self.mrb_state) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mrb() -> State {
        State::new().expect("Can't make a new mruby")
    }

    #[test]
    fn can_new_and_drop() {
        let _mrb = mrb();
    }

    #[test]
    fn can_get_a_fixnum() -> Result<(), Box<Error>> {
        let result = mrb().load_string("1 + 1")?;

        let val = match result {
            Value::Fixnum(i) => i,
            _ => bail!("failed"),
        };
        assert_eq!(2, val);
        Ok(())
    }

    #[test]
    fn can_get_a_string() -> Result<(), Box<Error>> {
        let result = mrb().load_string("'hello'")?;

        let val = match result {
            Value::String(s) => s,
            _ => bail!("failed"),
        };

        assert_eq!("hello", val);
        Ok(())
    }

    #[test]
    fn returns_error_if_unsupported_type() {
        mrb()
            .load_string("1.1")
            .expect_err("Currently we only support ints");
    }
}
