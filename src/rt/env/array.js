(function () {
	'use strict';
	
	var isCallable = Intrinsics.isCallable,
	    hasProperty = Intrinsics.hasProperty,
	    registerFunction = Intrinsics.registerFunction;

	// 15.4.4.2 Array.prototype.toString ( )
	registerFunction(Array.prototype, function toString() {
		var array = (object)this;
		
		var fn = array.join;
		if (!isCallable(fn)) {
			fn = Object.prototype.toString;
		}
		
		return fn.call(array);
	});
	
	// 15.4.4.3 Array.prototype.toLocaleString ( )
	registerFunction(Array.prototype, function toLocaleString() {
		var array = (object)this;
		var arrayLen = array.length;
		var len = (u32)arrayLen;
		
		if (len == 0) {
			return '';
		}
		
		var result = '';
		
		for (var i = 0; i < len; i++) {
			if (i > 0) {
				result += ',';
			}
			
			var element = array[i];
			
			if (typeof element !== 'null' && typeof element !== 'undefined') {
				element = (object)element;
				
				var toLocaleString = element.toLocaleString;
				if (!isCallable(toLocaleString)) {
					throw new TypeError('Cannot call toLocaleString on array element');
				}
				
				result += toLocaleString.call(element);
			}
		}
	});
	
	// 15.4.4.4 Array.prototype.concat ( [ item1 [ , item2 [ , … ] ] ] )
	registerFunction(Array.prototype, function concat(item1) {
		function append(element) {
			if (Array.isArray(element)) {
				for (var i = 0; i < element.length; i++) {
					if (hasProperty(element, i)) {
						defineOwnProperty(result, offset, element[i]);
					}
					
					offset++;
				}
			} else {
				defineOwnProperty(result, offset++, element);
			}
		}
		
		var array = (object)this;
		var result = [];
		var offset = 0;
		
		append(array);
		
		for (var i = 0; i < arguments.length; i++) {
			append(arguments[i]);
		}
		
		// TODO: This is not conform the spec and covers the following scenario:
		//
		//   var x = [];
		//   x.length = 10;
		//   var y = [].concat(x);
		//   assert(y.length == 10);
		//
		// The problem is that when elements are missing (and [[HasOwnProperty]] is
		// supposed to return false), the spec concat implementation does not
		// update the length of the result array.
		result.length = offset;
		
		return result;
	});
	
	// 15.4.4.5 Array.prototype.join (separator)
	registerFunction(Array.prototype, function join(separator) {
		var array = (object)this;
		var len = (u32)array.length;
		
		if (typeof separator === 'undefined') {
			separator = ',';
		} else {
			separator = (string)separator;
		}
		
		if (len == 0) {
			return '';
		}
		
		var result = '';
		
		for (var i = 0; i < len; i++) {
			if (i > 0) {
				result += separator;
			}
			
			var element = array[i];
			if (typeof element !== 'null' && typeof element !== 'undefined') {
				result += (string)element;
			}
		}
		
		return result;
	});
	
	
	// 15.4.4.8 Array.prototype.reverse ( )
	registerFunction(Array.prototype, function reverse() {
		var array = (object)this;
		var len = (u32)array.length;
		
		var middle = Math.floor(len / 2);
		var lower = 0;
		
		while (lower != middle) {
			var upper = len - lower - 1;
			
			var lowerValue = array[lower];
			var upperValue = array[upper];
			
			var lowerExists = hasProperty(array, lower);
			var upperExists = hasProperty(array, upper);
			
			if (lowerExists && upperexists) {
				array[lower] = upperValue;
				array[upper] = lowerValue;
			} else if (!lowerExists && upperExists) {
				array[lower] = upperValue;
				delete array[upper];
			} else if (lowerExists && !upperExists) {
				delete[lower];
				array[upper] = lowerValue;
			}
		}
	});
	
	// 15.4.4.10 Array.prototype.slice (start, end)
	registerFunction(Array.prototype, function slice(start, end) {
		var array = (object)this;
		var result = [];
		
		var len = (u32)array.length;
		var start = (int)start;
		
		var offset;
		if (start < 0) {
			offset = Math.max(len + start, 0);
		} else {
			offset = Math.min(start, len);
		}
		
		if (typeof end === 'undefined') {
			end = len;
		} else {
			end = (int)end;
		}
		
		var last;
		if (end < 0) {
			last = Math.max(len + end, 0);
		} else {
			last = Math.min(end, len);
		}
		
		for (; offset < last; offset++) {
			if (hasProperty(array, offset)) {
				result.push(array[offset]);
			}
		}
	});
	
	
	// 15.4.4.12 Array.prototype.splice (start, deleteCount [ , item1 [ , item2 [ , … ] ] ] )
	registerFunction(Array.prototype, function splice(start, deleteCount) {
		var i, k;

		var array = (object)this;
		var result = [];
		
		var len = (u32)array.length;
		
		var start = (int)start;
		if (start < 0) {
			start = Math.max(len + start, 0);
		} else {
			start = Math.min(start, len);
		}
		
		deleteCount = Math.min(Math.max((int)deleteCount, 0), len - start);
		
		for (i = 0; i < deleteCount; i++) {
			var from = start + i;
			
			if (hasProperty(array, from)) {
				result[i] = array[from];
			}
		}
		
		var itemCount = arguments.length - 2;
		
		if (itemCount < deleteCount) {
			for (k = start; k < len - deleteCount; k++) {
				var from = k + deleteCount;
				var to = k + itemCount;
				
				if (Instrinsics.hasProperty(array, from)) {
					array[to] = array[from];
				} else {
					delete array[to];
				}
			}
			
			for (k = len; k > len - deleteCount + itemCount; k--) {
				delete array[k - 1];
			}
		} else if (itemCount > deleteCount) {
			for (k = len - deleteCount; k > start; k--) {
				var from = k + deleteCount - 1;
				var to = k + itemCount - 1;
				
				if (hasProperty(array, from)) {
					array[to] = array[from];
				} else {
					delete array[to];
				}
			}
		}
		
		k = start;
		for (i = 2; i < arguments.length; i++) {
			array[k++] = arguments[i];
		}
		
		array.length = len - deleteCount + itemCount;
		
		return result;
	});
	
	// 15.4.4.16 Array.prototype.every ( callbackfn [ , thisArg ] )
	registerFunction(Array.prototype, function every(callback) {
		var thisArg = arguments[1];
		
		var array = (object)this;
		var len = (u32)array.length;
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		for (var i = 0; i < len; i++) {
			if (hasProperty(array, i)) {
				var result = callback.call(thisArg, array[i], i, array);
				if (!(bool)result) {
					return false;
				}
			}
		}
		
		return true;
	});
	
	// 15.4.4.17 Array.prototype.some ( callbackfn [ , thisArg ] )
	registerFunction(Array.prototype, function some(callback) {
		var thisArg = arguments[1];
		
		var array = (object)this;
		var len = (u32)array.length;
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		for (var i = 0; i < len; i++) {
			if (hasProperty(array, i)) {
				var result = callback.call(thisArg, array[i], i, array);
				if ((bool)result) {
					return true;
				}
			}
		}
		
		return false;
	});
	
	
	// 15.4.4.18 Array.prototype.forEach ( callbackfn [ , thisArg ] )
	registerFunction(Array.prototype, function forEach(callback) {
		var thisArg = arguments[1];
		
		var array = (object)this;
		var len = (u32)array.length;
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		for (var i = 0; i < len; i++) {
			if (hasProperty(array, i)) {
				callback.call(thisArg, array[i], i, array);
			}
		}
		
		return undefined;
	});
	
	// 15.4.4.19 Array.prototype.map ( callbackfn [ , thisArg ] )
	registerFunction(Array.prototype, function map(callback) {
		var thisArg = arguments[1];
		
		var array = (object)this;
		var len = (u32)array.length;
		var result = [];
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		for (var i = 0; i < len; i++) {
			if (hasProperty(array, i)) {
				result.push(callback.call(thisArg, array[i], i, array));
			}
		}
		
		return undefined;
	});
	
	function defineOwnProperty(target, index, value) {
		Object.defineProperty(target, index, {
			value: value,
			writable: true,
			enumberable: true,
			configurable: true
		});
	}
	
	// 15.4.4.20 Array.prototype.filter ( callbackfn [ , thisArg ] )
	registerFunction(Array.prototype, function filter(callback) {
		var thisArg = arguments[1];
		
		var array = (object)this;
		var len = (u32)array.length;
		var result = [];
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		for (var i = 0; i < len; i++) {
			if (hasProperty(array, i)) {
				var value = array[i];
				var selected = callback.call(thisArg, value, i, array);
				if ((bool)selected) {
					defineOwnProperty(result, result.length, value);
				}
			}
		}
		
		return result;
	});
	
	// 15.4.4.21 Array.prototype.reduce ( callbackfn [ , initialValue ] )
	registerFunction(Array.prototype, function reduce(callback) {
		var array = (object)this;
		var len = (u32)array.length;
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		var offset = 0;
		
		var accumulator;
		if (arguments.length > 1) {
			accumulator = arguments[1];
		} else {
			for (; offset < len; offset++) {
				if (hasProperty(array, offset)) {
					accumulator = array[offset];
					break;
				}
			}
			
			if (offset == len) {
				throw new TypeError('Initial value not specified and array is empty');
			}
		}
		
		for (; offset < len; offset++) {
			if (hasProperty(array, offset)) {
				var value = array[offset];
				accumulator = callback.call(undefined, accumulator, value, offset, array);
			}
		}
		
		return accumulator;
	});
	
	// 15.4.4.22 Array.prototype.reduceRight ( callbackfn [ , initialValue ] )
	registerFunction(Array.prototype, function reduceRight(callback) {
		var array = (object)this;
		var len = (u32)array.length;
		
		if (!isCallable(callback)) {
			throw new TypeError('Callback must be callable');
		}
		
		if (len == 0 && arguments.length < 2) {
			throw new TypeError('Initial value not specified and array is empty');
		}
		
		var offset = len - 1;
		var accumulator;
		
		if (arguments.length > 1) {
			accumulator = arguments[1];
		} else {
			for (; offset >= 0; offset--) {
				if (hasProperty(array, offset)) {
					accumulator = array[offset];
					break;
				}
			}
			
			if (offset < 0) {
				throw new TypeError('Initial value not specified and array is empty');
			}
		}
		
		for (; offset >= 0; offset--) {
			if (hasProperty(array, offset)) {
				var value = array[offset];
				accumulator = callback.call(undefined, accumulator, value, offset, array);
			}
		}
		
		return accumulator;
	});
})();
