use ::JsResult;
use rt::{JsEnv, JsArgs, JsValue, JsFnMode};
use gc::*;

// TODO
pub fn console_log(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let string = try!(args.arg(env, 0).to_string(env)).to_string();
	
	println!("{}", string);
	
	Ok(env.new_undefined())
}