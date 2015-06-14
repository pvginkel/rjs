#![allow(unused_variables)]

use ::{JsResult, JsError};
use rt::{JsEnv, JsArgs, JsValue, JsFnMode, JsItem, JsDescriptor, JsType};
use gc::*;
use syntax::Name;
use syntax::token::name;
use std::cmp::Ordering;
use std::u32;

macro_rules! local_try {
	( $expr:expr, $error:ident ) => {
		match $expr {
			Ok(ok) => ok,
			Err(error) => {
				$error = Some(error);
				return Ordering::Equal;
			}
		}
	}
}

// 15.4.1 The Array Constructor Called as a Function
// 15.4.2.1 new Array ( [ item0 [ , item1 [ , … ] ] ] )
// 15.4.2.2 new Array (len)
pub fn Array_constructor(env: &mut JsEnv, mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	if !mode.construct() {
		let target_args = args.args(env);
		return args.function(env).construct(env, target_args);
	}
	
	let mut array = env.create_array();
	
	if args.argc == 1 {
		let arg = args.arg(env, 0);
		if arg.ty() == JsType::Number {
			if let Some(len) = try!(arg.to_uint32_exact(env)) {
				let len = env.new_number(len as f64);
				try!(array.put(env, name::LENGTH, len, false));
			} else {
				return Err(JsError::new_range(env));
			}
		} else {
			try!(array.define_own_property(env, Name::from_index(0), JsDescriptor::new_simple_value(arg), false));
		}
	} else {
		for i in 0..args.argc {
			let arg = args.arg(env, i);
			try!(array.define_own_property(env, Name::from_index(i), JsDescriptor::new_simple_value(arg), false));
		}
	}
	
	Ok(array.as_value(env))
}

// 15.4.4.6 Array.prototype.pop ( )
pub fn Array_pop(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut array = try!(args.this(env).to_object(env));
	let len_val = try!(array.get(env, name::LENGTH));
	let len = try!(len_val.to_uint32(env));
	
	if len == 0 {
		let length = env.new_number(0f64);
		try!(array.put(env, name::LENGTH, length, true));
		
		Ok(env.new_undefined())
	} else {
		let index = len as usize - 1;
		let element = try!(array.get(env, Name::from_index(index)));
		try!(array.delete(env, Name::from_index(index), true));
		
		let length = env.new_number(index as f64);
		try!(array.put(env, name::LENGTH, length, true));
		
		Ok(element)
	}
}

// 15.4.4.7 Array.prototype.push ( [ item1 [ , item2 [ , … ] ] ] )
pub fn Array_push(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut array = try!(args.this(env).to_object(env));
	let len_val = try!(array.get(env, name::LENGTH));
	let mut index = try!(len_val.to_uint32(env)) as usize;
	
	for i in 0..args.argc {
		let arg = args.arg(env, i);
		
		let name = if index >= u32::MAX as usize {
			env.intern(&index.to_string())
		} else {
			Name::from_index(index)
		};
		
		try!(array.put(env, name, arg, true));
		
		index += 1;
	}
	
	let length = env.new_number(index as f64);
	try!(array.put(env, name::LENGTH, length, true));
	
	Ok(length)
}
}

// 15.4.4.9 Array.prototype.shift ( )
pub fn Array_shift(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut array = try!(args.this(env).to_object(env));
	let len_val = try!(array.get(env, name::LENGTH));
	let len = try!(len_val.to_uint32(env)) as usize;
	
	let result = if len == 0 {
		let length = env.new_number(0f64);
		try!(array.put(env, name::LENGTH, length, true));
		
		env.new_undefined()
	} else {
		let first = try!(array.get(env, Name::from_index(0)));
		
		for i in 1..len {
			let to = i - 1;
			let from_present = array.has_property(env, Name::from_index(i));
			
			if from_present {
				let from_val = try!(array.get(env, Name::from_index(i)));
				try!(array.put(env, Name::from_index(to), from_val, true));
			} else {
				try!(array.delete(env, Name::from_index(to), true));
			}
		}
		
		try!(array.delete(env, Name::from_index(len - 1), true));
		let length = env.new_number((len - 1) as f64);
		try!(array.put(env, name::LENGTH, length, true));
		
		first
	};
	
	Ok(result)
}

// 15.4.4.10 Array.prototype.slice (start, end)
pub fn Array_slice(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let array = try!(args.this(env).to_object(env));
	let mut result = env.create_array();
	let len_val = try!(array.get(env, name::LENGTH));
	let len = try!(len_val.to_uint32(env)) as usize;
	
	let relative_start = try!(args.arg(env, 0).to_integer(env));
	let mut k = if relative_start < 0f64 {
		let k = len as f64 + relative_start;
		if k < 0f64 { 0 } else { k as usize }
	} else {
		if relative_start < len as f64 { relative_start as usize } else { len }
	};
	
	let end = args.arg(env, 1);
	let relative_end = if end.is_undefined() {
		len as f64
	} else {
		try!(end.to_integer(env))
	};
	
	let final_ = if relative_end < 0f64 {
		let final_ = len as f64 + relative_end;
		if final_ > 0f64 { final_ as usize } else { 0 }
	} else {
		if relative_end < len as f64 { relative_end as usize } else { len }
	};
	
	let mut n = 0;
	
	while k < final_ {
		let k_present = array.has_property(env, Name::from_index(k));
		if k_present {
			let k_value = try!(array.get(env, Name::from_index(k)));
			try!(result.define_own_property(env, Name::from_index(n), JsDescriptor::new_simple_value(k_value), false));
		}
		
		k += 1;
		n += 1;
	}
	
	Ok(result.as_value(env))
}

// 15.4.4.11 Array.prototype.sort (comparefn)
// TODO: This is not a correct implementation!
pub fn Array_sort(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut obj = try!(args.this(env).to_object(env));
	let len_val = try!(obj.get(env, name::LENGTH));
	let len = try!(len_val.to_uint32(env)) as usize;
	
	let mut values = Vec::new();
	
	for i in 0..len {
		if obj.has_property(env, Name::from_index(i)) {
			values.push(Some(try!(obj.get(env, Name::from_index(i)))));
		} else {
			values.push(None)
		}
	}
	
	let compare_fn = {
		let arg = args.arg(env, 0);
		if arg.is_undefined() { None } else { Some(arg) }
	};
	
	let mut error = None;
	let this = env.new_undefined();
	let zero = env.new_number(0f64);
	
	values.sort_by(|x, y| {
		// Fast escape if we're in error mode.
		if error.is_some() {
			return Ordering::Equal;
		}
		
		if x.is_none() && y.is_none() {
			Ordering::Equal
		} else if x.is_none() {
			Ordering::Greater
		} else if y.is_none() {
			Ordering::Less
		} else {
			let x = x.unwrap();
			let y = y.unwrap();
			
			if x.is_undefined() && y.is_undefined() {
				Ordering::Equal
			} else if x.is_undefined() {
				Ordering::Greater
			} else if y.is_undefined() {
				Ordering::Less
			} else if let Some(compare_fn) = compare_fn {
				if !compare_fn.is_callable(env) {
					error = Some(JsError::new_type(env, ::errors::TYPE_NOT_A_FUNCTION));
					Ordering::Equal
				} else {
					let result = local_try!(compare_fn.call(env, this, vec![x, y], false), error);
					
					if local_try!(env.compare_lt(result, zero), error) {
						Ordering::Less
					} else if local_try!(env.compare_gt(result, zero), error) {
						Ordering::Greater
					} else {
						Ordering::Equal
					}
				}
			} else {
				let x_string = local_try!(x.to_string(env), error);
				let y_string = local_try!(y.to_string(env), error);
				let x_string = x_string.as_value(env);
				let y_string = y_string.as_value(env);
				
				if local_try!(env.compare_lt(x_string, y_string), error) {
					Ordering::Less
				} else if local_try!(env.compare_gt(x_string, y_string), error) {
					Ordering::Greater
				} else {
					Ordering::Equal
				}
			}
		}
	});
	
	if let Some(error) = error {
		return Err(error);
	}
	
	for i in 0..values.len() as usize {
		if let Some(value) = values[i] {
			try!(obj.put(env, Name::from_index(i), value, true));
		} else {
			try!(obj.delete(env, Name::from_index(i), true));
		}
	}
	
	Ok(obj)
}

	let len = try!(len_val.to_uint32(env)) as isize;
		try!(args.arg(env, 0).to_integer(env)) as isize
	let actual_start = if relative_start < 0 {
		cmp::max(relative_start + len, 0) as usize
		if actual_start < 0f64 { 0 } else { actual_start as usize }
		cmp::min(relative_start, len) as usize
		try!(args.arg(env, 1).to_integer(env)) as isize
	let delete_count = if delete_count < 0f64 { 0f64 } else { delete_count };
	let actual_delete_count = if delete_count < (len - actual_start) as f64 {
		delete_count as usize
		0
// 15.4.4.13 Array.prototype.unshift ( [ item1 [ , item2 [ , … ] ] ] )
pub fn Array_unshift(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let mut array = try!(args.this(env).to_object(env));
	let len_val = try!(array.get(env, name::LENGTH));
	let len = try!(len_val.to_uint32(env)) as usize;
	
	let arg_count = args.argc;
	let mut k = len;
	while k > 0 {
		let from = Name::from_index(k - 1);
		let to = Name::from_index(k + arg_count - 1);
		let from_present = array.has_property(env, from);
		if from_present {
			let from_value = try!(array.get(env, from));
			try!(array.put(env, to, from_value, true));
		} else {
			try!(array.delete(env, to, true));
		}
		
		k -= 1;
	}
	
	for i in 0..args.argc {
		let arg = args.arg(env, i);
		try!(array.put(env, Name::from_index(i), arg, true));
	}
	
	let length = env.new_number((len + arg_count) as f64);
	try!(array.put(env, name::LENGTH, length, true));
	
	Ok(length)
}

// 15.4.4.14 Array.prototype.indexOf ( searchElement [ , fromIndex ] )
pub fn Array_indexOf(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let array = try!(args.this(env).to_object(env));
	let len_value = try!(array.get(env, name::LENGTH));
	let len = try!(len_value.to_uint32(env)) as usize;
	
	let search_element = args.arg(env, 0);
	
	let result = if len == 0 {
		-1
	} else {
		let n = if args.argc > 1 {
			try!(args.arg(env, 1).to_integer(env))
		} else {
			0f64
		};
		
		if n >= len as f64 {
			-1
		} else {
			let k = if n >= 0f64 {
				n as usize
			} else {
				let k = len as f64 - n.abs();
				if k < 0f64 { 0 } else { k as usize }
			};
			
			for i in k..len {
				let present = array.has_property(env, Name::from_index(i));
				if present {
					let element = try!(array.get(env, Name::from_index(i)));
					if env.strict_eq(search_element, element) {
						return Ok(env.new_number(i as f64));
					}
				}
			}
			
			-1
		}
	};
	
	Ok(env.new_number(result as f64))
}

// 15.4.4.15 Array.prototype.lastIndexOf ( searchElement [ , fromIndex ] )
pub fn Array_lastIndexOf(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let array = try!(args.this(env).to_object(env));
	let len_value = try!(array.get(env, name::LENGTH));
	let len = try!(len_value.to_uint32(env)) as usize;
	
	let result = if len == 0 {
		-1
	} else {
		let n = if args.argc > 1 {
			try!(args.arg(env, 1).to_integer(env))
		} else {
			(len - 1) as f64
		};
		
		let mut k = if n >= 0f64 {
			if n > (len - 1) as f64 { len - 1 } else { n as usize }
		} else {
			let n = n.abs();
			if n > len as f64 {
				return Ok(env.new_number(-1f64));
			} else {
				len - n as usize
			}
		};
		
		let search_element = args.arg(env, 0);
		
		loop {
			let k_present = array.has_property(env, Name::from_index(k));
			if k_present {
				let element_k = try!(array.get(env, Name::from_index(k)));
				if env.strict_eq(search_element, element_k) {
					return Ok(env.new_number(k as f64));
				}
			}
			
			if k == 0 {
				break
			}
			
			k -= 1;
		}
		
		-1
	};
	
	Ok(env.new_number(result as f64))
}

	let this_arg = args.arg(env, 0);
	let this_arg = args.arg(env, 0);
		if k_present {
			let mapped_value = try!(callback_fn.call(
			));
		} else {
			env.new_undefined()
		};
			
	let this_arg = args.arg(env, 0);
	let accumulator = if args.argc > 1 {
			let accumulator = try!(callback_fn.call(
				vec![k_value, k, array],
	let accumulator = if args.argc > 1 {
		while accumulator.is_none() && k >= len {
	while k >= len {
			let accumulator = try!(callback_fn.call(
				vec![k_value, k, array],
// 15.4.3.2 Array.isArray ( arg )
pub fn Array_isArray(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let arg = args.arg(env, 0);
	
	let result = if arg.ty() != JsType::Object {
		false
	} else {
		if arg.class(env) == Some(name::ARRAY_CLASS) {
			true
		} else if arg.get_ptr() == env.array_prototype.as_ptr() {
			// TODO: This is not conform the specs (the specs state that only the [[Class]]
			// should be checked) but the ECMA test suite tests for this and Chrome
			// passes this test.
			true
		} else {
			false
		}
	};
	
	Ok(env.new_bool(result))
}
