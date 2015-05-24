#[macro_use]
extern crate lazy_static;

use gc::Root;
use rt::{JsEnv, JsValue, JsString, JsItem};
use std::fmt;
use syntax::Name;
use syntax::token::name;

#[macro_use]
pub mod debug;
pub mod syntax;
pub mod ir;
pub mod util;
#[macro_use]
pub mod gc;
pub mod rt;
mod errors;

pub enum JsError {
	Io(std::io::Error),
	Lex(String),
	Parse(String),
	Runtime(Root<JsValue>)
}

impl JsError {
	fn new_error(env: &mut JsEnv, name: Name, message: Option<&str>, file_name: Option<&str>, line_number: Option<usize>) -> JsResult<Root<JsValue>> {
		// If construction of the error fails, we simply propagate the error itself.
		
		let _scope = env.heap().new_local_scope();
		
		let class = try!(env.global().as_local(env).get(env, name));
		
		let mut args = Vec::new();
		
		args.push(match message {
			Some(message) => JsString::from_str(env, message).as_value(env),
			None => JsValue::new_undefined().as_local(env)
		});
		args.push(match file_name {
			Some(file_name) => JsString::from_str(env, file_name).as_value(env),
			None => JsValue::new_undefined().as_local(env)
		});
		args.push(match line_number {
			Some(line_number) => JsValue::new_number(line_number as f64).as_local(env),
			None => JsValue::new_undefined().as_local(env)
		});
		
		let obj = try!(class.construct(env, args));
		
		Ok(Root::from_local(env.heap(), obj))
	}
	
	pub fn new_runtime(env: &mut JsEnv, name: Name, message: Option<&str>, file_name: Option<&str>, line_number: Option<usize>) -> JsError {
		match Self::new_error(env, name, message, file_name, line_number) {
			Ok(error) => JsError::Runtime(error),
			Err(error) => error
		}
	}
	
	pub fn new_type(env: &mut JsEnv, message: &str) -> JsError {
		Self::new_runtime(env, name::TYPE_ERROR_CLASS, Some(message), None, None)
	}
	
	pub fn new_range(env: &mut JsEnv) -> JsError {
		Self::new_runtime(env, name::RANGE_ERROR_CLASS, None, None, None)
	}
	
	pub fn new_reference(env: &mut JsEnv) -> JsError {
		Self::new_runtime(env, name::REFERENCE_ERROR_CLASS, None, None, None)
	}
	
	pub fn as_runtime(&self, env: &mut JsEnv) -> Root<JsValue> {
		match *self {
			JsError::Lex(ref message) | JsError::Parse(ref message) => {
				match Self::new_error(env, name::SYNTAX_ERROR_CLASS, Some(&message), None, None) {
					Ok(error) => error,
					Err(error) => error.as_runtime(env)
				}
			}
			JsError::Runtime(ref error) => error.clone(),
			ref error @ _ => {
				// TODO: This could be nicer.
				let error = JsString::from_str(env, &format!("{:?}", error)).as_value(env);
				Root::from_local(env.heap(), error)
			}
		}
	}
}

impl fmt::Debug for JsError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(write!(formatter, "JsError {{ "));
		match *self {
			JsError::Io(ref err) => try!(err.fmt(formatter)),
			JsError::Lex(ref message) => try!(write!(formatter, "Lex {{ {} }}", message)),
			JsError::Parse(ref message) => try!(write!(formatter, "Parse {{ {} }}", message)),
			JsError::Runtime(..) => try!(write!(formatter, "Runtime {{ .. }}"))
		}
		write!(formatter, " }}")
	}
}

pub type JsResult<T> = Result<T, JsError>;