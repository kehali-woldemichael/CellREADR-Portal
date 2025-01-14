let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
                if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
                    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
                }
                return cachedUint8ArrayMemory0;
            }

function getStringFromWasm0(ptr, len) {
                ptr = ptr >>> 0;
                return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
            }

function isLikeNone(x) {
                return x === undefined || x === null;
            }

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
                if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
                    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
                }
                return cachedDataViewMemory0;
            }

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
                        ? function (arg, view) {
            return cachedTextEncoder.encodeInto(arg, view);
        }
                        : function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        });

function passStringToWasm0(arg, malloc, realloc) {
                
                if (realloc === undefined) {
                    const buf = cachedTextEncoder.encode(arg);
                    const ptr = malloc(buf.length, 1) >>> 0;
                    getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
                    WASM_VECTOR_LEN = buf.length;
                    return ptr;
                }

                let len = arg.length;
                let ptr = malloc(len, 1) >>> 0;

                const mem = getUint8ArrayMemory0();

                let offset = 0;

                for (; offset < len; offset++) {
                    const code = arg.charCodeAt(offset);
                    if (code > 0x7F) break;
                    mem[ptr + offset] = code;
                }
            
                if (offset !== len) {
                    if (offset !== 0) {
                        arg = arg.slice(offset);
                    }
                    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
                    const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
                    const ret = encodeString(arg, view);
                    
                    offset += ret.written;
                    ptr = realloc(ptr, len, offset, 1) >>> 0;
                }

                WASM_VECTOR_LEN = offset;
                return ptr;
            }

function debugString(val) {
                // primitive types
                const type = typeof val;
                if (type == 'number' || type == 'boolean' || val == null) {
                    return  `${val}`;
                }
                if (type == 'string') {
                    return `"${val}"`;
                }
                if (type == 'symbol') {
                    const description = val.description;
                    if (description == null) {
                        return 'Symbol';
                    } else {
                        return `Symbol(${description})`;
                    }
                }
                if (type == 'function') {
                    const name = val.name;
                    if (typeof name == 'string' && name.length > 0) {
                        return `Function(${name})`;
                    } else {
                        return 'Function';
                    }
                }
                // objects
                if (Array.isArray(val)) {
                    const length = val.length;
                    let debug = '[';
                    if (length > 0) {
                        debug += debugString(val[0]);
                    }
                    for(let i = 1; i < length; i++) {
                        debug += ', ' + debugString(val[i]);
                    }
                    debug += ']';
                    return debug;
                }
                // Test for built-in
                const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
                let className;
                if (builtInMatches.length > 1) {
                    className = builtInMatches[1];
                } else {
                    // Failed to match the standard '[object ClassName]'
                    return toString.call(val);
                }
                if (className == 'Object') {
                    // we're a user defined class or Object
                    // JSON.stringify avoids problems with cycles, and is generally much
                    // easier than looping through ownProperties of `val`.
                    try {
                        return 'Object(' + JSON.stringify(val) + ')';
                    } catch (_) {
                        return 'Object';
                    }
                }
                // errors
                if (val instanceof Error) {
                    return `${val.name}: ${val.message}\n${val.stack}`;
                }
                // TODO we could test for more things here, like `Set`s and `Map`s.
                return className;
            }

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
                ? { register: () => {}, unregister: () => {} }
                : new FinalizationRegistry(state => {
                    wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b)
                });

function makeClosure(arg0, arg1, dtor, f) {
                const state = { a: arg0, b: arg1, cnt: 1, dtor };
                const real = (...args) => {
                    // First up with a closure we increment the internal reference
                    // count. This ensures that the Rust closure environment won't
                    // be deallocated while we're invoking it.
                    state.cnt++;
                    try {
                        return f(state.a, state.b, ...args);
                    } finally {
                        if (--state.cnt === 0) {
                            wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b);
                            state.a = 0;
                            CLOSURE_DTORS.unregister(state);
                        }
                    }
                };
                real.original = state;
                CLOSURE_DTORS.register(real, state, state);
                return real;
            }
function __wbg_adapter_50(arg0, arg1, arg2) {
const ptr0 = passStringToWasm0(arg2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len0 = WASM_VECTOR_LEN;
wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hec5d38ec36b9ce9f(arg0, arg1, ptr0, len0);
}

function makeMutClosure(arg0, arg1, dtor, f) {
                const state = { a: arg0, b: arg1, cnt: 1, dtor };
                const real = (...args) => {
                    // First up with a closure we increment the internal reference
                    // count. This ensures that the Rust closure environment won't
                    // be deallocated while we're invoking it.
                    state.cnt++;
                    const a = state.a;
                    state.a = 0;
                    try {
                        return f(a, state.b, ...args);
                    } finally {
                        if (--state.cnt === 0) {
                            wasm.__wbindgen_export_3.get(state.dtor)(a, state.b);
                            CLOSURE_DTORS.unregister(state);
                        } else {
                            state.a = a;
                        }
                    }
                };
                real.original = state;
                CLOSURE_DTORS.register(real, state, state);
                return real;
            }
function __wbg_adapter_53(arg0, arg1, arg2, arg3) {
wasm.closure326_externref_shim(arg0, arg1, arg2, arg3);
}

function __wbg_adapter_56(arg0, arg1, arg2) {
wasm.closure1035_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_59(arg0, arg1, arg2) {
wasm.closure1479_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_62(arg0, arg1) {
wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h45941fafa122e0a7(arg0, arg1);
}

function __wbg_adapter_65(arg0, arg1, arg2) {
wasm.closure1596_externref_shim(arg0, arg1, arg2);
}

export function hydrate() {
wasm.hydrate();
}

function getFromExternrefTable0(idx) { return wasm.__wbindgen_export_2.get(idx); }

function getCachedStringFromWasm0(ptr, len) {
                if (ptr === 0) {
                    return getFromExternrefTable0(len);
                } else {
                    return getStringFromWasm0(ptr, len);
                }
            }

function addToExternrefTable0(obj) {
                    const idx = wasm.__externref_table_alloc();
                    wasm.__wbindgen_export_2.set(idx, obj);
                    return idx;
                }

function handleError(f, args) {
                        try {
                            return f.apply(this, args);
                        } catch (e) {
                            const idx = addToExternrefTable0(e);
                            wasm.__wbindgen_exn_store(idx);
                        }
                    }

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
function __wbg_adapter_465(arg0, arg1, arg2, arg3) {
wasm.closure1634_externref_shim(arg0, arg1, arg2, arg3);
}

const __wbindgen_enum_BinaryType = ["blob", "arraybuffer"];

const __wbindgen_enum_NotificationDirection = ["auto", "ltr", "rtl"];

const __wbindgen_enum_NotificationPermission = ["default", "denied", "granted"];

const __wbindgen_enum_PermissionState = ["granted", "denied", "prompt"];

const __wbindgen_enum_ReadableStreamReaderMode = ["byob"];

const __wbindgen_enum_ReadableStreamType = ["bytes"];

const __wbindgen_enum_ReferrerPolicy = ["", "no-referrer", "no-referrer-when-downgrade", "origin", "origin-when-cross-origin", "unsafe-url", "same-origin", "strict-origin", "strict-origin-when-cross-origin"];

const __wbindgen_enum_RequestCache = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const __wbindgen_enum_RequestRedirect = ["follow", "error", "manual"];

const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];

const __wbindgen_enum_ResponseType = ["basic", "cors", "default", "error", "opaque", "opaqueredirect"];

const __wbindgen_enum_ScrollBehavior = ["auto", "instant", "smooth"];

const __wbindgen_enum_ShadowRootMode = ["open", "closed"];

const __wbindgen_enum_VideoFacingModeEnum = ["user", "environment", "left", "right"];

const __wbindgen_enum_VisibilityState = ["hidden", "visible"];

const IntoUnderlyingByteSourceFinalization = (typeof FinalizationRegistry === 'undefined')
                ? { register: () => {}, unregister: () => {} }
                : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingbytesource_free(ptr >>> 0, 1));

export class IntoUnderlyingByteSource {

            __destroy_into_raw() {
                const ptr = this.__wbg_ptr;
                this.__wbg_ptr = 0;
                IntoUnderlyingByteSourceFinalization.unregister(this);
                return ptr;
            }

            free() {
                const ptr = this.__destroy_into_raw();
                wasm.__wbg_intounderlyingbytesource_free(ptr, 0);
            }
            /**
 * @returns {string}
 */
get type() {
const ret = wasm.intounderlyingbytesource_type(this.__wbg_ptr);
var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
return v1;
}
/**
 * @returns {number}
 */
get autoAllocateChunkSize() {
const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
return ret >>> 0;
}
/**
 * @param {ReadableByteStreamController} controller
 */
start(controller) {
wasm.intounderlyingbytesource_start(this.__wbg_ptr, controller);
}
/**
 * @param {ReadableByteStreamController} controller
 * @returns {Promise<any>}
 */
pull(controller) {
const ret = wasm.intounderlyingbytesource_pull(this.__wbg_ptr, controller);
return ret;
}
cancel() {
const ptr = this.__destroy_into_raw();
wasm.intounderlyingbytesource_cancel(ptr);
}
}

const IntoUnderlyingSinkFinalization = (typeof FinalizationRegistry === 'undefined')
                ? { register: () => {}, unregister: () => {} }
                : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsink_free(ptr >>> 0, 1));

export class IntoUnderlyingSink {

            __destroy_into_raw() {
                const ptr = this.__wbg_ptr;
                this.__wbg_ptr = 0;
                IntoUnderlyingSinkFinalization.unregister(this);
                return ptr;
            }

            free() {
                const ptr = this.__destroy_into_raw();
                wasm.__wbg_intounderlyingsink_free(ptr, 0);
            }
            /**
 * @param {any} chunk
 * @returns {Promise<any>}
 */
write(chunk) {
const ret = wasm.intounderlyingsink_write(this.__wbg_ptr, chunk);
return ret;
}
/**
 * @returns {Promise<any>}
 */
close() {
const ptr = this.__destroy_into_raw();
const ret = wasm.intounderlyingsink_close(ptr);
return ret;
}
/**
 * @param {any} reason
 * @returns {Promise<any>}
 */
abort(reason) {
const ptr = this.__destroy_into_raw();
const ret = wasm.intounderlyingsink_abort(ptr, reason);
return ret;
}
}

const IntoUnderlyingSourceFinalization = (typeof FinalizationRegistry === 'undefined')
                ? { register: () => {}, unregister: () => {} }
                : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsource_free(ptr >>> 0, 1));

export class IntoUnderlyingSource {

            __destroy_into_raw() {
                const ptr = this.__wbg_ptr;
                this.__wbg_ptr = 0;
                IntoUnderlyingSourceFinalization.unregister(this);
                return ptr;
            }

            free() {
                const ptr = this.__destroy_into_raw();
                wasm.__wbg_intounderlyingsource_free(ptr, 0);
            }
            /**
 * @param {ReadableStreamDefaultController} controller
 * @returns {Promise<any>}
 */
pull(controller) {
const ret = wasm.intounderlyingsource_pull(this.__wbg_ptr, controller);
return ret;
}
cancel() {
const ptr = this.__destroy_into_raw();
wasm.intounderlyingsource_cancel(ptr);
}
}

async function __wbg_load(module, imports) {
                    if (typeof Response === 'function' && module instanceof Response) {
                        if (typeof WebAssembly.instantiateStreaming === 'function') {
                            try {
                                return await WebAssembly.instantiateStreaming(module, imports);

                            } catch (e) {
                                if (module.headers.get('Content-Type') != 'application/wasm') {
                                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                                } else {
                                    throw e;
                                }
                            }
                        }

                        const bytes = await module.arrayBuffer();
                        return await WebAssembly.instantiate(bytes, imports);

                    } else {
                        const instance = await WebAssembly.instantiate(module, imports);

                        if (instance instanceof WebAssembly.Instance) {
                            return { instance, module };

                        } else {
                            return instance;
                        }
                    }
                }

                function __wbg_get_imports() {
                    const imports = {};
                    imports.wbg = {};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
const ret = arg0 === undefined;
return ret;
};
imports.wbg.__wbindgen_in = function(arg0, arg1) {
const ret = arg0 in arg1;
return ret;
};
imports.wbg.__wbindgen_is_null = function(arg0) {
const ret = arg0 === null;
return ret;
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
const ret = getStringFromWasm0(arg0, arg1);
return ret;
};
imports.wbg.__wbindgen_is_array = function(arg0) {
const ret = Array.isArray(arg0);
return ret;
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
const obj = arg0.original;
if (obj.cnt-- == 1) {
obj.a = 0;
return true;
}
const ret = false;
return ret;
};
imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
console.error(v0);
};
imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
const ret = new Error();
return ret;
};
imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
const ret = arg1.stack;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_is_object = function(arg0) {
const val = arg0;
const ret = typeof(val) === 'object' && val !== null;
return ret;
};
imports.wbg.__wbindgen_is_string = function(arg0) {
const ret = typeof(arg0) === 'string';
return ret;
};
imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
const ret = arg0.crypto;
return ret;
};
imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
const ret = arg0.msCrypto;
return ret;
};
imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
arg0.getRandomValues(arg1);
}, arguments) };
imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
arg0.randomFillSync(arg1);
}, arguments) };
imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
const ret = module.require;
return ret;
}, arguments) };
imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
const ret = arg0.process;
return ret;
};
imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
const ret = arg0.versions;
return ret;
};
imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
const ret = arg0.node;
return ret;
};
imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
const obj = arg1;
const ret = typeof(obj) === 'number' ? obj : undefined;
getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};
imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
const obj = arg1;
const ret = typeof(obj) === 'string' ? obj : undefined;
var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_boolean_get = function(arg0) {
const v = arg0;
const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
return ret;
};
imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
const ret = arg0 === arg1;
return ret;
};
imports.wbg.__wbindgen_is_falsy = function(arg0) {
const ret = !arg0;
return ret;
};
imports.wbg.__wbg_fetch_eeae7120f2a07ede = typeof fetch == 'function' ? fetch : notDefined('fetch');
imports.wbg.__wbindgen_is_function = function(arg0) {
const ret = typeof(arg0) === 'function';
return ret;
};
imports.wbg.__wbg_queueMicrotask_c5419c06eab41e73 = typeof queueMicrotask == 'function' ? queueMicrotask : notDefined('queueMicrotask');
imports.wbg.__wbg_queueMicrotask_848aa4969108a57e = function(arg0) {
const ret = arg0.queueMicrotask;
return ret;
};
imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
const ret = new Error(getStringFromWasm0(arg0, arg1));
return ret;
};
imports.wbg.__wbindgen_as_number = function(arg0) {
const ret = +arg0;
return ret;
};
imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
const ret = arg0 == arg1;
return ret;
};
imports.wbg.__wbg_getwithrefkey_edc2c8960f0f1191 = function(arg0, arg1) {
const ret = arg0[arg1];
return ret;
};
imports.wbg.__wbg_instanceof_Window_5012736c80a01584 = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof Window;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_document_8554450897a855b9 = function(arg0) {
const ret = arg0.document;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_location_af118da6c50d4c3f = function(arg0) {
const ret = arg0.location;
return ret;
};
imports.wbg.__wbg_history_489e13d0b625263c = function() { return handleError(function (arg0) {
const ret = arg0.history;
return ret;
}, arguments) };
imports.wbg.__wbg_navigator_6210380287bf8581 = function(arg0) {
const ret = arg0.navigator;
return ret;
};
imports.wbg.__wbg_getComputedStyle_ba4609b39055f674 = function() { return handleError(function (arg0, arg1) {
const ret = arg0.getComputedStyle(arg1);
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };
imports.wbg.__wbg_scrollTo_19dc1dbbc8c19fa8 = function(arg0, arg1, arg2) {
arg0.scrollTo(arg1, arg2);
};
imports.wbg.__wbg_requestAnimationFrame_b4b782250b9c2c88 = function() { return handleError(function (arg0, arg1) {
const ret = arg0.requestAnimationFrame(arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_clearTimeout_25cdc2ed88b3c0b2 = function(arg0, arg1) {
arg0.clearTimeout(arg1);
};
imports.wbg.__wbg_setTimeout_73b734ca971c19f4 = function() { return handleError(function (arg0, arg1, arg2) {
const ret = arg0.setTimeout(arg1, arg2);
return ret;
}, arguments) };
imports.wbg.__wbg_body_b3bb488e8e54bf4b = function(arg0) {
const ret = arg0.body;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_head_6c6317d70f23ff16 = function(arg0) {
const ret = arg0.head;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_createComment_7a1d9856e50567bb = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.createComment(v0);
return ret;
};
imports.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55 = function(arg0) {
const ret = arg0.createDocumentFragment();
return ret;
};
imports.wbg.__wbg_createElement_5921e9eb06b9ec89 = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.createElement(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_createTextNode_8bce33cf33bf8f6e = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.createTextNode(v0);
return ret;
};
imports.wbg.__wbg_createTreeWalker_5b69acaa3481b1f7 = function() { return handleError(function (arg0, arg1, arg2) {
const ret = arg0.createTreeWalker(arg1, arg2 >>> 0);
return ret;
}, arguments) };
imports.wbg.__wbg_getElementById_f56c8e6a15a6926d = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.getElementById(v0);
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_querySelector_e21c39150aa72078 = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.querySelector(v0);
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };
imports.wbg.__wbg_querySelectorAll_52447cbab6df8bae = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.querySelectorAll(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_writeText_20fb3f7393d34052 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.writeText(v0);
return ret;
};
imports.wbg.__wbg_instanceof_Response_e91b7eb7c611a9ae = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof Response;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_url_1bf85c8abeb8c92d = function(arg0, arg1) {
const ret = arg1.url;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_status_ae8de515694c5c7c = function(arg0) {
const ret = arg0.status;
return ret;
};
imports.wbg.__wbg_headers_5e283e8345689121 = function(arg0) {
const ret = arg0.headers;
return ret;
};
imports.wbg.__wbg_text_a94b91ea8700357a = function() { return handleError(function (arg0) {
const ret = arg0.text();
return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof ShadowRoot;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_host_fdfe1258b06fe937 = function(arg0) {
const ret = arg0.host;
return ret;
};
imports.wbg.__wbg_namespaceURI_d27c7f3638dd2926 = function(arg0, arg1) {
const ret = arg1.namespaceURI;
var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_scrollTop_69eca9764f0e696c = function(arg0) {
const ret = arg0.scrollTop;
return ret;
};
imports.wbg.__wbg_scrollLeft_d57a937188efebd2 = function(arg0) {
const ret = arg0.scrollLeft;
return ret;
};
imports.wbg.__wbg_scrollWidth_a2bdee78f09b4be7 = function(arg0) {
const ret = arg0.scrollWidth;
return ret;
};
imports.wbg.__wbg_scrollHeight_dd26e5c88313153f = function(arg0) {
const ret = arg0.scrollHeight;
return ret;
};
imports.wbg.__wbg_clientWidth_e73e836a6257fbe5 = function(arg0) {
const ret = arg0.clientWidth;
return ret;
};
imports.wbg.__wbg_clientHeight_9325ebe2318ce8af = function(arg0) {
const ret = arg0.clientHeight;
return ret;
};
imports.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.innerHTML = v0;
};
imports.wbg.__wbg_outerHTML_463e1461b3d153ca = function(arg0, arg1) {
const ret = arg1.outerHTML;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_children_0725d024369e02df = function(arg0) {
const ret = arg0.children;
return ret;
};
imports.wbg.__wbg_getAttribute_e867e037f066c410 = function(arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
const ret = arg1.getAttribute(v0);
var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len2 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
};
imports.wbg.__wbg_getBoundingClientRect_35fc4d8fa10e0463 = function(arg0) {
const ret = arg0.getBoundingClientRect();
return ret;
};
imports.wbg.__wbg_hasAttribute_a17d49194d050f19 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.hasAttribute(v0);
return ret;
};
imports.wbg.__wbg_removeAttribute_c80e298b60689065 = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_scrollIntoView_4b805e2532108e71 = function(arg0) {
arg0.scrollIntoView();
};
imports.wbg.__wbg_scrollTo_128d0568f61f681d = function(arg0, arg1) {
arg0.scrollTo(arg1);
};
imports.wbg.__wbg_setAttribute_d5540a19be09f8dc = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
var v1 = getCachedStringFromWasm0(arg3, arg4);
arg0.setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_before_ac3792b457802cbf = function() { return handleError(function (arg0, arg1) {
arg0.before(arg1);
}, arguments) };
imports.wbg.__wbg_remove_5b68b70c39041e2a = function(arg0) {
arg0.remove();
};
imports.wbg.__wbg_append_c473cdcda58a1a9e = function() { return handleError(function (arg0, arg1, arg2) {
arg0.append(arg1, arg2);
}, arguments) };
imports.wbg.__wbg_getwithindex_272e38cf4b97b8aa = function(arg0, arg1) {
const ret = arg0[arg1 >>> 0];
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_ctrlKey_957c6c31b62b4550 = function(arg0) {
const ret = arg0.ctrlKey;
return ret;
};
imports.wbg.__wbg_shiftKey_8c0f9a5ca3ff8f93 = function(arg0) {
const ret = arg0.shiftKey;
return ret;
};
imports.wbg.__wbg_altKey_d3fbce7596aac8cf = function(arg0) {
const ret = arg0.altKey;
return ret;
};
imports.wbg.__wbg_metaKey_be0158b14b1cef4a = function(arg0) {
const ret = arg0.metaKey;
return ret;
};
imports.wbg.__wbg_button_460cdec9f2512a91 = function(arg0) {
const ret = arg0.button;
return ret;
};
imports.wbg.__wbg_nodeName_cbf72a21681425d9 = function(arg0, arg1) {
const ret = arg1.nodeName;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_parentNode_3e06cf96d7693d57 = function(arg0) {
const ret = arg0.parentNode;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_childNodes_031aa96d5e3d21b0 = function(arg0) {
const ret = arg0.childNodes;
return ret;
};
imports.wbg.__wbg_previousSibling_076df2178284ef97 = function(arg0) {
const ret = arg0.previousSibling;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_nextSibling_f6396d6fd0b97830 = function(arg0) {
const ret = arg0.nextSibling;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_textContent_a049d1ce093c3d21 = function(arg0, arg1) {
const ret = arg1.textContent;
var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_settextContent_cd38ea7d4e0f7260 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.textContent = v0;
};
imports.wbg.__wbg_appendChild_ac45d1abddf1b89b = function() { return handleError(function (arg0, arg1) {
const ret = arg0.appendChild(arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_cloneNode_629a1b180e91c467 = function() { return handleError(function (arg0) {
const ret = arg0.cloneNode();
return ret;
}, arguments) };
imports.wbg.__wbg_removeChild_139b30d19f579e41 = function() { return handleError(function (arg0, arg1) {
const ret = arg0.removeChild(arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_new_95093d1a71aba61d = function() { return handleError(function () {
const ret = new Range();
return ret;
}, arguments) };
imports.wbg.__wbg_deleteContents_45ba9b733b3b97ea = function() { return handleError(function (arg0) {
arg0.deleteContents();
}, arguments) };
imports.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41 = function() { return handleError(function (arg0, arg1) {
arg0.setEndBefore(arg1);
}, arguments) };
imports.wbg.__wbg_setStartAfter_a96a0162a15cf5a8 = function() { return handleError(function (arg0, arg1) {
arg0.setStartAfter(arg1);
}, arguments) };
imports.wbg.__wbg_byobRequest_b32c77640da946ac = function(arg0) {
const ret = arg0.byobRequest;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_close_aca7442e6619206b = function() { return handleError(function (arg0) {
arg0.close();
}, arguments) };
imports.wbg.__wbg_scrollTop_92b55c10c137abcb = function(arg0) {
const ret = arg0.scrollTop;
return ret;
};
imports.wbg.__wbg_dataset_b0dd1a3513142b54 = function(arg0) {
const ret = arg0.dataset;
return ret;
};
imports.wbg.__wbg_style_e06c9e03355741e9 = function(arg0) {
const ret = arg0.style;
return ret;
};
imports.wbg.__wbg_setbehavior_af86e7f6f648de50 = function(arg0, arg1) {
arg0.behavior = __wbindgen_enum_ScrollBehavior[arg1];
};
imports.wbg.__wbg_setleft_da60b2c0a6ccbc56 = function(arg0, arg1) {
arg0.left = arg1;
};
imports.wbg.__wbg_settop_a891a94ff069007a = function(arg0, arg1) {
arg0.top = arg1;
};
imports.wbg.__wbg_getPropertyValue_b0f0858c3b5f17dd = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
const ret = arg1.getPropertyValue(v0);
const ptr2 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len2 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
}, arguments) };
imports.wbg.__wbg_removeProperty_cfd836a4f7e5e86e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
const ret = arg1.removeProperty(v0);
const ptr2 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len2 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
}, arguments) };
imports.wbg.__wbg_setProperty_ff389e5a7fb9910e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
var v1 = getCachedStringFromWasm0(arg3, arg4);
arg0.setProperty(v0, v1);
}, arguments) };
imports.wbg.__wbg_close_cef2400b120c9c73 = function() { return handleError(function (arg0) {
arg0.close();
}, arguments) };
imports.wbg.__wbg_enqueue_6f3d433b5e457aea = function() { return handleError(function (arg0, arg1) {
arg0.enqueue(arg1);
}, arguments) };
imports.wbg.__wbg_setcapture_4818ebe9ef88b2f6 = function(arg0, arg1) {
arg0.capture = arg1 !== 0;
};
imports.wbg.__wbg_setonce_06b35a72a3fafc15 = function(arg0, arg1) {
arg0.once = arg1 !== 0;
};
imports.wbg.__wbg_setpassive_70ce6704aec553f6 = function(arg0, arg1) {
arg0.passive = arg1 !== 0;
};
imports.wbg.__wbg_instanceof_HtmlAnchorElement_7a88f0b97085fa30 = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof HTMLAnchorElement;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_target_fed794e9a6ed73fe = function(arg0, arg1) {
const ret = arg1.target;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_href_31456ceb26f92368 = function(arg0, arg1) {
const ret = arg1.href;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_new_ac9dbf743c2383ee = function() { return handleError(function () {
const ret = new URLSearchParams();
return ret;
}, arguments) };
imports.wbg.__wbg_clipboard_0d7b5c390c14b0e6 = function(arg0) {
const ret = arg0.clipboard;
return ret;
};
imports.wbg.__wbg_setbody_734cb3d7ee8e6e96 = function(arg0, arg1) {
arg0.body = arg1;
};
imports.wbg.__wbg_setheaders_be10a5ab566fd06f = function(arg0, arg1) {
arg0.headers = arg1;
};
imports.wbg.__wbg_setmethod_dc68a742c2db5c6a = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.method = v0;
};
imports.wbg.__wbg_new_42acb42ec2ace97c = function() { return handleError(function (arg0) {
const ret = new ResizeObserver(arg0);
return ret;
}, arguments) };
imports.wbg.__wbg_disconnect_1dbf7e19d9590abd = function(arg0) {
arg0.disconnect();
};
imports.wbg.__wbg_observe_60f3631b2f7c6d8b = function(arg0, arg1, arg2) {
arg0.observe(arg1, arg2);
};
imports.wbg.__wbg_setdata_27c6828c5a5a5ce4 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.data = v0;
};
imports.wbg.__wbg_before_34932b0128b3e8f6 = function() { return handleError(function (arg0, arg1) {
arg0.before(arg1);
}, arguments) };
imports.wbg.__wbg_remove_81fdfdc9088ee033 = function(arg0) {
arg0.remove();
};
imports.wbg.__wbg_target_b7cb1739bee70928 = function(arg0) {
const ret = arg0.target;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_defaultPrevented_9e2309e82258aee7 = function(arg0) {
const ret = arg0.defaultPrevented;
return ret;
};
imports.wbg.__wbg_cancelBubble_0374b329f66f59b5 = function(arg0) {
const ret = arg0.cancelBubble;
return ret;
};
imports.wbg.__wbg_composedPath_d1052062308beae5 = function(arg0) {
const ret = arg0.composedPath();
return ret;
};
imports.wbg.__wbg_preventDefault_c55d86c27b2dfa6e = function(arg0) {
arg0.preventDefault();
};
imports.wbg.__wbg_url_87b699cb6519ba34 = function(arg0, arg1) {
const ret = arg1.url;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_newwithstr_31920be5b8b6d221 = function() { return handleError(function (arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = new Request(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_newwithstrandinit_a31c69e4cc337183 = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = new Request(v0, arg2);
return ret;
}, arguments) };
imports.wbg.__wbg_get_9a60fce5bf7da364 = function(arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
const ret = arg1[v0];
var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len2 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
};
imports.wbg.__wbg_contentRect_c1a9045c459744d9 = function(arg0) {
const ret = arg0.contentRect;
return ret;
};
imports.wbg.__wbg_borderBoxSize_692fda7b4e3b97f1 = function(arg0) {
const ret = arg0.borderBoxSize;
return ret;
};
imports.wbg.__wbg_contentBoxSize_a2d93ded171ea1de = function(arg0) {
const ret = arg0.contentBoxSize;
return ret;
};
imports.wbg.__wbg_devicePixelContentBoxSize_8d531ca6a4331b28 = function(arg0) {
const ret = arg0.devicePixelContentBoxSize;
return ret;
};
imports.wbg.__wbg_nextNode_9a6a06f51331eace = function() { return handleError(function (arg0) {
const ret = arg0.nextNode();
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };
imports.wbg.__wbg_error_09480e4aadca50ad = typeof console.error == 'function' ? console.error : notDefined('console.error');
imports.wbg.__wbg_warn_2b3adb99ce26c314 = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');
imports.wbg.__wbg_sethref_9d76f6c9356e9638 = function() { return handleError(function (arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.href = v0;
}, arguments) };
imports.wbg.__wbg_origin_648082c4831a5be8 = function() { return handleError(function (arg0, arg1) {
const ret = arg1.origin;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_pathname_6e6871539b48a0e5 = function() { return handleError(function (arg0, arg1) {
const ret = arg1.pathname;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_search_20c15d493b8602c5 = function() { return handleError(function (arg0, arg1) {
const ret = arg1.search;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_hash_313d7fdf42f6e7d3 = function() { return handleError(function (arg0, arg1) {
const ret = arg1.hash;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_view_2a901bda0727aeb3 = function(arg0) {
const ret = arg0.view;
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_respond_a799bab31a44f2d7 = function() { return handleError(function (arg0, arg1) {
arg0.respond(arg1 >>> 0);
}, arguments) };
imports.wbg.__wbg_state_b863826253700666 = function() { return handleError(function (arg0) {
const ret = arg0.state;
return ret;
}, arguments) };
imports.wbg.__wbg_pushState_fc8b2d0c45854901 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
var v1 = getCachedStringFromWasm0(arg4, arg5);
arg0.pushState(arg1, v0, v1);
}, arguments) };
imports.wbg.__wbg_replaceState_c3213575ed65bac2 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
var v1 = getCachedStringFromWasm0(arg4, arg5);
arg0.replaceState(arg1, v0, v1);
}, arguments) };
imports.wbg.__wbg_length_4919f4a62b9b1e94 = function(arg0) {
const ret = arg0.length;
return ret;
};
imports.wbg.__wbg_item_984ea38de8896660 = function(arg0, arg1) {
const ret = arg0.item(arg1 >>> 0);
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_new_e27c93803e1acc42 = function() { return handleError(function () {
const ret = new Headers();
return ret;
}, arguments) };
imports.wbg.__wbg_get_5069e42d7c221f57 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg2, arg3);
const ret = arg1.get(v0);
var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
var len2 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
}, arguments) };
imports.wbg.__wbg_set_b3c7c6d2e5e783d6 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
var v1 = getCachedStringFromWasm0(arg3, arg4);
arg0.set(v0, v1);
}, arguments) };
imports.wbg.__wbg_checked_1502c0098821afb3 = function(arg0) {
const ret = arg0.checked;
return ret;
};
imports.wbg.__wbg_value_d4a95e7a0d390578 = function(arg0, arg1) {
const ret = arg1.value;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_inlineSize_322ab111c2b5c9e3 = function(arg0) {
const ret = arg0.inlineSize;
return ret;
};
imports.wbg.__wbg_blockSize_981c4dfa6e1263a8 = function(arg0) {
const ret = arg0.blockSize;
return ret;
};
imports.wbg.__wbg_append_d510a297e3ba948e = function() { return handleError(function (arg0, arg1) {
arg0.append(arg1);
}, arguments) };
imports.wbg.__wbg_append_22299f1011b1f9d7 = function() { return handleError(function (arg0, arg1, arg2) {
arg0.append(arg1, arg2);
}, arguments) };
imports.wbg.__wbg_width_e7964a50b174d035 = function(arg0) {
const ret = arg0.width;
return ret;
};
imports.wbg.__wbg_height_cd5c897b4d3fabe3 = function(arg0) {
const ret = arg0.height;
return ret;
};
imports.wbg.__wbg_top_322638693276a225 = function(arg0) {
const ret = arg0.top;
return ret;
};
imports.wbg.__wbg_addEventListener_e167f012cbedfa4e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.addEventListener(v0, arg3);
}, arguments) };
imports.wbg.__wbg_addEventListener_14b036ff7cb8747c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.addEventListener(v0, arg3, arg4);
}, arguments) };
imports.wbg.__wbg_removeEventListener_b6cef5ad085bea8f = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.removeEventListener(v0, arg3);
}, arguments) };
imports.wbg.__wbg_removeEventListener_f19508a45d20bda3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.removeEventListener(v0, arg3, arg4);
}, arguments) };
imports.wbg.__wbg_setbox_0d838a2d268b7fac = function(arg0, arg1) {
arg0.box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
};
imports.wbg.__wbg_href_f1d20018a97415a0 = function(arg0, arg1) {
const ret = arg1.href;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_origin_b1cdab9cfa04b734 = function(arg0, arg1) {
const ret = arg1.origin;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_pathname_adec1eb7f76356a8 = function(arg0, arg1) {
const ret = arg1.pathname;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_search_f384756d8e27fd66 = function(arg0, arg1) {
const ret = arg1.search;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_setsearch_b9d03a586b9a2fa4 = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
arg0.search = v0;
};
imports.wbg.__wbg_searchParams_8b40e0942f870b44 = function(arg0) {
const ret = arg0.searchParams;
return ret;
};
imports.wbg.__wbg_hash_50828fbc16613897 = function(arg0, arg1) {
const ret = arg1.hash;
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_new_33db4be5d9963ec1 = function() { return handleError(function (arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = new URL(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_newwithbase_ba5e3790a41efd02 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
var v1 = getCachedStringFromWasm0(arg2, arg3);
const ret = new URL(v0, v1);
return ret;
}, arguments) };
imports.wbg.__wbg_decodeURI_135fe2e8f0684ba4 = function() { return handleError(function (arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = decodeURI(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_decodeURIComponent_b5b4c94b85a4ec75 = function() { return handleError(function (arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = decodeURIComponent(v0);
return ret;
}, arguments) };
imports.wbg.__wbg_get_5419cf6b954aa11d = function(arg0, arg1) {
const ret = arg0[arg1 >>> 0];
return ret;
};
imports.wbg.__wbg_isArray_6f3b47f09adb61b5 = function(arg0) {
const ret = Array.isArray(arg0);
return ret;
};
imports.wbg.__wbg_length_f217bbbf7e8e4df4 = function(arg0) {
const ret = arg0.length;
return ret;
};
imports.wbg.__wbg_instanceof_ArrayBuffer_74945570b4a62ec7 = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof ArrayBuffer;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_instanceof_Error_a0af335a62107964 = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof Error;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_new_70a2f23d1565c04c = function(arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = new Error(v0);
return ret;
};
imports.wbg.__wbg_message_00eebca8fa4dd7db = function(arg0) {
const ret = arg0.message;
return ret;
};
imports.wbg.__wbg_name_aa32a0ae51232604 = function(arg0) {
const ret = arg0.name;
return ret;
};
imports.wbg.__wbg_toString_4b677455b9167e31 = function(arg0) {
const ret = arg0.toString();
return ret;
};
imports.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43 = function(arg0, arg1) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
const ret = new Function(v0);
return ret;
};
imports.wbg.__wbg_call_a9ef466721e824f2 = function() { return handleError(function (arg0, arg1) {
const ret = arg0.call(arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_call_3bfa248576352471 = function() { return handleError(function (arg0, arg1, arg2) {
const ret = arg0.call(arg1, arg2);
return ret;
}, arguments) };
imports.wbg.__wbg_next_b06e115d1b01e10b = function() { return handleError(function (arg0) {
const ret = arg0.next();
return ret;
}, arguments) };
imports.wbg.__wbg_next_13b477da1eaa3897 = function(arg0) {
const ret = arg0.next;
return ret;
};
imports.wbg.__wbg_done_983b5ffcaec8c583 = function(arg0) {
const ret = arg0.done;
return ret;
};
imports.wbg.__wbg_value_2ab8a198c834c26a = function(arg0) {
const ret = arg0.value;
return ret;
};
imports.wbg.__wbg_isSafeInteger_b9dff570f01a9100 = function(arg0) {
const ret = Number.isSafeInteger(arg0);
return ret;
};
imports.wbg.__wbg_now_70af4fe37a792251 = function() {
const ret = Date.now();
return ret;
};
imports.wbg.__wbg_entries_c02034de337d3ee2 = function(arg0) {
const ret = Object.entries(arg0);
return ret;
};
imports.wbg.__wbg_is_4b64bc96710d6a0f = function(arg0, arg1) {
const ret = Object.is(arg0, arg1);
return ret;
};
imports.wbg.__wbg_new_e69b5f66fda8f13c = function() {
const ret = new Object();
return ret;
};
imports.wbg.__wbg_toString_aea130fe68d19e1a = function(arg0) {
const ret = arg0.toString();
return ret;
};
imports.wbg.__wbg_exec_c872ad5c15e456ad = function(arg0, arg1, arg2) {
var v0 = getCachedStringFromWasm0(arg1, arg2);
const ret = arg0.exec(v0);
return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_new_800498ec872f75d4 = function(arg0, arg1, arg2, arg3) {
var v0 = getCachedStringFromWasm0(arg0, arg1);
var v1 = getCachedStringFromWasm0(arg2, arg3);
const ret = new RegExp(v0, v1);
return ret;
};
imports.wbg.__wbg_iterator_695d699a44d6234c = function() {
const ret = Symbol.iterator;
return ret;
};
imports.wbg.__wbg_new_1073970097e5a420 = function(arg0, arg1) {
try {
var state0 = {a: arg0, b: arg1};
var cb0 = (arg0, arg1) => {
const a = state0.a;
state0.a = 0;
try {
return __wbg_adapter_465(a, state0.b, arg0, arg1);
} finally {
state0.a = a;
}
};
const ret = new Promise(cb0);
return ret;
} finally {
state0.a = state0.b = 0;
}
};
imports.wbg.__wbg_resolve_0aad7c1484731c99 = function(arg0) {
const ret = Promise.resolve(arg0);
return ret;
};
imports.wbg.__wbg_then_748f75edfb032440 = function(arg0, arg1) {
const ret = arg0.then(arg1);
return ret;
};
imports.wbg.__wbg_then_4866a7d9f55d8f3e = function(arg0, arg1, arg2) {
const ret = arg0.then(arg1, arg2);
return ret;
};
imports.wbg.__wbg_globalThis_05c129bf37fcf1be = function() { return handleError(function () {
const ret = globalThis.globalThis;
return ret;
}, arguments) };
imports.wbg.__wbg_self_bf91bf94d9e04084 = function() { return handleError(function () {
const ret = self.self;
return ret;
}, arguments) };
imports.wbg.__wbg_window_52dd9f07d03fd5f8 = function() { return handleError(function () {
const ret = window.window;
return ret;
}, arguments) };
imports.wbg.__wbg_global_3eca19bb09e9c484 = function() { return handleError(function () {
const ret = global.global;
return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_Uint8Array_df0761410414ef36 = function(arg0) {
let result;
                    try {
                        result = arg0 instanceof Uint8Array;
                    } catch (_) {
                        result = false;
                    }
                    const ret = result;
return ret;
};
imports.wbg.__wbg_new_fec2611eb9180f95 = function(arg0) {
const ret = new Uint8Array(arg0);
return ret;
};
imports.wbg.__wbg_newwithlength_76462a666eca145f = function(arg0) {
const ret = new Uint8Array(arg0 >>> 0);
return ret;
};
imports.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730 = function(arg0, arg1, arg2) {
const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
return ret;
};
imports.wbg.__wbg_buffer_95102df5554646dc = function(arg0) {
const ret = arg0.buffer;
return ret;
};
imports.wbg.__wbg_subarray_975a06f9dbd16995 = function(arg0, arg1, arg2) {
const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
return ret;
};
imports.wbg.__wbg_length_9254c4bd3b9f23c4 = function(arg0) {
const ret = arg0.length;
return ret;
};
imports.wbg.__wbg_byteLength_5d623ba3d92a3a9c = function(arg0) {
const ret = arg0.byteLength;
return ret;
};
imports.wbg.__wbg_byteOffset_ec0928143c619cd7 = function(arg0) {
const ret = arg0.byteOffset;
return ret;
};
imports.wbg.__wbg_set_ec2fcf81bc573fd9 = function(arg0, arg1, arg2) {
arg0.set(arg1, arg2 >>> 0);
};
imports.wbg.__wbg_get_ef828680c64da212 = function() { return handleError(function (arg0, arg1) {
const ret = Reflect.get(arg0, arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_has_bd717f25f195f23d = function() { return handleError(function (arg0, arg1) {
const ret = Reflect.has(arg0, arg1);
return ret;
}, arguments) };
imports.wbg.__wbg_set_e864d25d9b399c9f = function() { return handleError(function (arg0, arg1, arg2) {
const ret = Reflect.set(arg0, arg1, arg2);
return ret;
}, arguments) };
imports.wbg.__wbg_buffer_ccaed51a635d8a2d = function(arg0) {
const ret = arg0.buffer;
return ret;
};
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
const ret = debugString(arg1);
const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
const len1 = WASM_VECTOR_LEN;
getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_rethrow = function(arg0) {
throw arg0;
};
imports.wbg.__wbindgen_memory = function() {
const ret = wasm.memory;
return ret;
};
imports.wbg.__wbindgen_closure_wrapper3554 = function(arg0, arg1, arg2) {
const ret = makeClosure(arg0, arg1, 325, __wbg_adapter_50);
return ret;
};
imports.wbg.__wbindgen_closure_wrapper3556 = function(arg0, arg1, arg2) {
const ret = makeMutClosure(arg0, arg1, 327, __wbg_adapter_53);
return ret;
};
imports.wbg.__wbindgen_closure_wrapper9993 = function(arg0, arg1, arg2) {
const ret = makeMutClosure(arg0, arg1, 1036, __wbg_adapter_56);
return ret;
};
imports.wbg.__wbindgen_closure_wrapper16308 = function(arg0, arg1, arg2) {
const ret = makeMutClosure(arg0, arg1, 1480, __wbg_adapter_59);
return ret;
};
imports.wbg.__wbindgen_closure_wrapper17081 = function(arg0, arg1, arg2) {
const ret = makeMutClosure(arg0, arg1, 1557, __wbg_adapter_62);
return ret;
};
imports.wbg.__wbindgen_closure_wrapper18602 = function(arg0, arg1, arg2) {
const ret = makeMutClosure(arg0, arg1, 1597, __wbg_adapter_65);
return ret;
};
imports.wbg.__wbindgen_init_externref_table = function() {
const table = wasm.__wbindgen_export_2;
const offset = table.grow(4);
table.set(0, undefined);
table.set(offset + 0, undefined);
table.set(offset + 1, null);
table.set(offset + 2, true);
table.set(offset + 3, false);
;
};

                    return imports;
                }

                function __wbg_init_memory(imports, memory) {
                    
                }

                function __wbg_finalize_init(instance, module) {
                    wasm = instance.exports;
                    __wbg_init.__wbindgen_wasm_module = module;
                    cachedDataViewMemory0 = null;
cachedUint8ArrayMemory0 = null;

                    
                    wasm.__wbindgen_start();
                    return wasm;
                }

                function initSync(module) {
                    if (wasm !== undefined) return wasm;

                    
                    if (typeof module !== 'undefined') {
                        if (Object.getPrototypeOf(module) === Object.prototype) {
                            ({module} = module)
                        } else {
                            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
                        }
                    }

                    const imports = __wbg_get_imports();

                    __wbg_init_memory(imports);

                    if (!(module instanceof WebAssembly.Module)) {
                        module = new WebAssembly.Module(module);
                    }

                    const instance = new WebAssembly.Instance(module, imports);

                    return __wbg_finalize_init(instance, module);
                }

                async function __wbg_init(module_or_path) {
                    if (wasm !== undefined) return wasm;

                    
                    if (typeof module_or_path !== 'undefined') {
                        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                            ({module_or_path} = module_or_path)
                        } else {
                            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
                        }
                    }

                    
                    const imports = __wbg_get_imports();

                    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
                        module_or_path = fetch(module_or_path);
                    }

                    __wbg_init_memory(imports);

                    const { instance, module } = await __wbg_load(await module_or_path, imports);

                    return __wbg_finalize_init(instance, module);
                }
            
export { initSync };
export default __wbg_init;
