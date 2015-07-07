use rt::{JsEnv, JsValue, JsObject, JsItem, GC_SCOPE, GC_VALUE};
use gc::*;

// Modifications to this struct must be synchronized with the GC walker.
pub struct JsScope {
    items: Array<JsValue>
}

impl JsScope {
    pub fn new_local_thin<'s>(scope: &'s LocalScope, size: usize, parent: Option<Local<'s, JsScope>>) -> Local<'s, JsScope> {
        let mut result = scope.alloc_local::<JsScope>(GC_SCOPE);
        
        unsafe {
            result.items = scope.alloc_array(GC_VALUE, size + 1);
        }
        
        if let Some(parent) = parent {
            result.raw_set(0, parent.as_value(scope));
        }
        
        result
    }
    
    pub fn new_local_thick<'s>(scope: &'s LocalScope, scope_object: Local<'s, JsObject>, parent: Option<Local<'s, JsScope>>, arguments: bool) -> Local<'s, JsScope> {
        let mut result = scope.alloc_local::<JsScope>(GC_SCOPE);
        
        let size = 2 + if arguments { 1 } else { 0 };
        
        unsafe {
            result.items = scope.alloc_array(GC_VALUE, size);
        }
        
        if let Some(parent) = parent {
            result.raw_set(0, parent.as_value(scope));
        }
        result.raw_set(1, scope_object.as_value(scope));
        
        result
    }
}

impl<'a> Local<'a, JsScope> {
    pub fn as_value(&self, env: &JsEnv, scope: &'s LocalScope) -> Local<'s, JsValue> {
        env.new_scope(scope, *self)
    }
    
    pub fn parent<'s>(&self, scope: &'s LocalScope) -> Option<Local<'s, JsScope>> {
        let parent = self.raw_get(scope, 0);
        
        if parent.is_undefined() { None } else { Some(parent.unwrap_scope(env)) }
    }
    
    pub fn scope_object<'s>(&self, scope: &'s LocalScope) -> Local<'s, JsObject> {
        self.raw_get(scope, 1).unwrap_object(scope)
    }
    
    pub fn arguments<'s>(&self, scope: &'s LocalScope) -> Option<Local<'s, JsValue>> {
        if self.items.len() == 2 {
            None
        } else {
            Some(self.raw_get(scope, 2))
        }
    }
    
    pub fn set_arguments<'s>(&mut self, arguments: Local<'s, JsValue>) {
        if self.items.len() == 2 {
            panic!("scope does not have a slot to store arguments");
        }
        
        self.raw_set(2, arguments);
    }
    
    pub fn len(&self) -> usize {
        self.items.len() - 1
    }
    
    pub fn get<'s>(&self, scope: &'s LocalScope, index: usize) -> Local<'s, JsValue> {
        self.raw_get(scope, index + 1)
    }
    
    pub fn set<'s>(&mut self, index: usize, value: Local<'s, JsValue>) {
        self.raw_set(index + 1, value)
    }
    
    fn raw_get<'s>(&self, env: &JsEnv, scope: &'s LocalScope, index: usize) -> Local<'s, JsValue> {
        let mut local = env.new_value(scope);
        
        *local = self.items[index];
        
        local
    }
    
    fn raw_set<'s>(&mut self, index: usize, value: Local<'s, JsValue>) {
        self.items[index] = *value;
    }
}
