use gc::{Array, Local};
use rt::{JsEnv, JsValue, JsItem, JsDescriptor, JsHandle, GC_STRING, GC_U16};
use rt::utf;
use syntax::Name;
use syntax::token::name;

// Modifications to this struct must be synchronized with the GC walker.
pub struct JsString {
    chars: Array<u16>
}

impl JsString {
    pub fn new_local<'s>(scope: &'s LocalScope, size: usize) -> Local<'s, JsString> {
        let mut result = scope.alloc_local::<JsString>(GC_STRING);
        
        unsafe {
            result.chars = scope.alloc_array(GC_U16, size);
        }
        
        result
    }
    
    pub fn from_str<'s>(scope: &'s LocalScope, string: &str) -> Local<'s, JsString> {
        let chars = utf::utf32_to_utf16(
            &string.chars().map(|c| c as u32).collect::<Vec<_>>()[..],
            false
        );
        
        let mut result = Self::new_local(scope, chars.len());
        
        {
            let result_chars = &mut *result.chars;
            
            for i in 0..chars.len() {
                result_chars[i] = chars[i];
            }
        }
        
        result
    }
    
    pub fn from_u16<'s>(scope: &'s LocalScope, chars: &[u16]) -> Local<'s, JsString> {
        // TODO #84: Most of the calls to this function take the chars from the GC
        // heap. Because of this we create a copy of chars. However, this must
        // be changed so that this extra copy is unnecessary.
        
        let mut copy = Vec::with_capacity(chars.len());
        for i in 0..chars.len() {
            copy.push(chars[i]);
        }
        
        let result = JsString::new_local(scope, copy.len());
        
        let mut result_chars = result.chars;
        
        for i in 0..copy.len() {
            result_chars[i] = copy[i];
        }
        
        result
    }
    
    pub fn chars(&self) -> &[u16] {
        &*self.chars
    }
    
    pub fn concat<'s>(scope: &'s LocalScope, strings: &[Local<'s, JsString>]) -> Local<'s, JsString> {
        let mut len = 0;
        for string in strings {
            len += string.chars().len();
        }
        
        let mut result = Self::new_local(scope, len);
        
        {
            let chars = &mut *result.chars;
            let mut offset = 0;
            
            for string in strings {
                let string_chars = string.chars();
                for i in 0..string_chars.len() {
                    chars[offset] = string_chars[i];
                    offset += 1;
                }
            }
        }
        
        result
    }
    
    pub fn equals<'s>(x: Local<'s, JsString>, y: Local<'s, JsString>) -> bool {
        let x_chars = &*x.chars;
        let y_chars = &*y.chars;
        
        if x_chars.len() != y_chars.len() {
            false
        } else {
            for i in 0..x_chars.len() {
                if x_chars[i] != y_chars[i] {
                    return false
                }
            }
            
            true
        }
    }
    
    pub fn to_string(&self) -> String {
        ::rt::utf::utf16_to_string(&*self.chars)
    }
}

impl<'a> JsItem for Local<'a, JsString> {
    fn as_value<'s>(&self, env: &JsEnv, scope: &'s LocalScope) -> Local<'s, JsValue> {
        env.new_string(*self, scope)
    }
    
    fn has_prototype(&self, _: &JsEnv) -> bool {
        true
    }
    
    fn prototype(&self, env: &JsEnv) -> Option<Local<JsValue>> {
        Some(env.handle(JsHandle::String).as_value(env))
    }
    
    // 15.5.5.2 [[GetOwnProperty]] ( P )
    // 15.5.5.1 length
    fn get_own_property(&self, env: &JsEnv, property: Name) -> Option<JsDescriptor> {
        if property == name::LENGTH {
            let value = env.new_number(self.chars.len() as f64);
            return Some(JsDescriptor::new_value(value, false, false, false));
        }
        
        if let Some(index) = property.index() {
            let chars = self.chars;
            if index < chars.len() {
                let char = chars[index];
                let mut string = JsString::new_local(env, 1);
                string.chars[0] = char;
                return Some(JsDescriptor::new_value(string.as_value(env), false, true, false));
            }
        }
        
        None
    }
}
