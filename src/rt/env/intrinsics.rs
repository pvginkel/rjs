use ::{JsResult, JsError};
use rt::{JsEnv, JsArgs, JsValue, JsFnMode, JsItem, JsDescriptor, JsType, JsFunction};
use gc::*;
use syntax::token::name;

pub fn Intrinsics_isCallable(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let result = args.arg(env, 0).is_callable(env);
	
	Ok(env.new_bool(result))
}

pub fn Intrinsics_hasProperty(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let object = args.arg(env, 0);
	let property = try!(args.arg(env, 1).to_string(env)).to_string();
	let property = env.intern(&property);
	
	let result = object.has_property(env, property);
	
	Ok(env.new_bool(result))
}

pub fn Intrinsics_registerFunction(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut target = args.arg(env, 0);
	let function = args.arg(env, 1);
	
	if function.ty() != JsType::Object || target.ty() != JsType::Object {
		JsError::new_type(env, ::errors::TYPE_INVALID);
	} else {
		let mut object = function.unwrap_object(env);
		
		let name = match object.function() {
			Some(JsFunction::Ir(function_ref)) => {
				let function = env.ir.get_function(function_ref);
				if let Some(name) = function.name {
					name
				} else {
					return Err(JsError::new_type(env, ::errors::TYPE_FUNCTION_HAS_NO_NAME))
				}
			}
			_ => return Err(JsError::new_type(env, ::errors::TYPE_NOT_A_FUNCTION))
		};

		object.set_can_construct(env, false);
		object.delete_unchecked(env, name::PROTOTYPE);
		
		
		try!(target.define_own_property(env, name, JsDescriptor::new_value(function, true, false, true), false));
	}
	
	Ok(env.new_undefined())
}
