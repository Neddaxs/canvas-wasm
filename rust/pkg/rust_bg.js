import * as wasm from './rust_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
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
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h48bdeb0d3ad2d441(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hecd24e6413118652(arg0, arg1, arg2);
}

function __wbg_adapter_30(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h48bdeb0d3ad2d441(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_33(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h48bdeb0d3ad2d441(arg0, arg1, addHeapObject(arg2));
}

/**
* @param {number} num1
* @param {number} num2
* @returns {number}
*/
export function add(num1, num2) {
    var ret = wasm.add(num1, num2);
    return ret;
}

/**
* @param {string} canvas_id
*/
export function snake(canvas_id) {
    var ptr0 = passStringToWasm0(canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.snake(ptr0, len0);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
*/
export class TannerClient {

    static __wrap(ptr) {
        const obj = Object.create(TannerClient.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tannerclient_free(ptr);
    }
    /**
    * @returns {TannerClient}
    */
    static new() {
        var ret = wasm.tannerclient_new();
        return TannerClient.__wrap(ret);
    }
    /**
    * @param {number} _time
    * @param {number} _height
    * @param {number} _width
    */
    update(_time, _height, _width) {
        wasm.tannerclient_update(this.ptr, _time, _height, _width);
    }
    /**
    */
    render() {
        wasm.tannerclient_render(this.ptr);
    }
}

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_error_d127905bc2cdd021(arg0, arg1) {
    console.error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_cb_drop(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};

export function __wbg_log_4c5cd786a2b55b17(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_is_falsy(arg0) {
    var ret = !getObject(arg0);
    return ret;
};

export function __wbindgen_string_new(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_new_693216e109162396() {
    var ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_error_09919627ac0992f5(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export function __wbg_randomFillSync_64cc7d048f228ca8() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };

export function __wbg_getRandomValues_98117e9a7e993920() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

export function __wbg_process_2f24d6544ea7b200(arg0) {
    var ret = getObject(arg0).process;
    return addHeapObject(ret);
};

export function __wbindgen_is_object(arg0) {
    const val = getObject(arg0);
    var ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg_versions_6164651e75405d4a(arg0) {
    var ret = getObject(arg0).versions;
    return addHeapObject(ret);
};

export function __wbg_node_4b517d861cbcb3bc(arg0) {
    var ret = getObject(arg0).node;
    return addHeapObject(ret);
};

export function __wbindgen_is_string(arg0) {
    var ret = typeof(getObject(arg0)) === 'string';
    return ret;
};

export function __wbg_modulerequire_3440a4bcf44437db() { return handleError(function (arg0, arg1) {
    var ret = module.require(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_crypto_98fc271021c7d2ad(arg0) {
    var ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export function __wbg_msCrypto_a2cdb043d2bfe57f(arg0) {
    var ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

export function __wbg_instanceof_WebGl2RenderingContext_56ad96bfac3f5531(arg0) {
    var ret = getObject(arg0) instanceof WebGL2RenderingContext;
    return ret;
};

export function __wbg_bufferData_bba22fbe5dd1f1d6(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
};

export function __wbg_attachShader_7faccaa7b5ac28a6(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
};

export function __wbg_bindBuffer_4ece833dd10cac2f(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
};

export function __wbg_blendFunc_b254bb91838df1dd(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
};

export function __wbg_clear_4ce66c813d66e77d(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
};

export function __wbg_clearColor_71f96fd72a7646a6(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
};

export function __wbg_clearDepth_998e0b481aed1b74(arg0, arg1) {
    getObject(arg0).clearDepth(arg1);
};

export function __wbg_compileShader_dd66d66a5a6481f3(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
};

export function __wbg_createBuffer_5c5caa16032a81b7(arg0) {
    var ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_createProgram_32d01a55e144b9fc(arg0) {
    var ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_createShader_6e8eed55567fe1a6(arg0, arg1) {
    var ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_drawArrays_5db2f4e6291f7fb2(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
};

export function __wbg_enable_766e546395da5a5d(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
};

export function __wbg_enableVertexAttribArray_91da8d3cbe0c2bbd(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
};

export function __wbg_getAttribLocation_5d304d390c7273f5(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return ret;
};

export function __wbg_getProgramInfoLog_18c849a5fa54e7b1(arg0, arg1, arg2) {
    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_getProgramParameter_80edd3cfbcf7cf1d(arg0, arg1, arg2) {
    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_getShaderInfoLog_ba1de20c14b6fb63(arg0, arg1, arg2) {
    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_getShaderParameter_264d9ab5c13ece4d(arg0, arg1, arg2) {
    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_linkProgram_b84796e37364e5c9(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
};

export function __wbg_shaderSource_18f45f93c05a8311(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
};

export function __wbg_useProgram_c2fdf4a953d1128a(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
};

export function __wbg_validateProgram_2b095acdaada7683(arg0, arg1) {
    getObject(arg0).validateProgram(getObject(arg1));
};

export function __wbg_vertexAttribPointer_76d558694fe81cd7(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
};

export function __wbg_instanceof_Window_c4b70662a0d2c5ec(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

export function __wbg_document_1c64944725c0d81d(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_requestAnimationFrame_71638ca922068239() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
}, arguments) };

export function __wbg_preventDefault_9866c9fd51eecfb6(arg0) {
    getObject(arg0).preventDefault();
};

export function __wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e(arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

export function __wbg_setwidth_c1a7061891b71f25(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

export function __wbg_setheight_88894b05710ff752(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

export function __wbg_getContext_f701d0231ae22393() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

export function __wbg_addEventListener_52721772cc0a7f30() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
}, arguments) };

export function __wbg_instanceof_HtmlDivElement_abfbf03c1045fc4b(arg0) {
    var ret = getObject(arg0) instanceof HTMLDivElement;
    return ret;
};

export function __wbg_createElement_86c152812a141a62() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_getElementById_f3e94458ce77f0d0(arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export function __wbg_setAttribute_1b533bf07966de55() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_offsetWidth_69cd6669725b154f(arg0) {
    var ret = getObject(arg0).offsetWidth;
    return ret;
};

export function __wbg_offsetHeight_8da312843e7777ab(arg0) {
    var ret = getObject(arg0).offsetHeight;
    return ret;
};

export function __wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae(arg0) {
    var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    return ret;
};

export function __wbg_setfillStyle_528a6a267c863ae7(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
};

export function __wbg_setfont_884816cc1b46ae3f(arg0, arg1, arg2) {
    getObject(arg0).font = getStringFromWasm0(arg1, arg2);
};

export function __wbg_fillRect_10e42dc7a5e8cccd(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
};

export function __wbg_strokeText_ac933a55116c976f() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).strokeText(getStringFromWasm0(arg1, arg2), arg3, arg4);
}, arguments) };

export function __wbg_clientX_97ff0f5c7b19e687(arg0) {
    var ret = getObject(arg0).clientX;
    return ret;
};

export function __wbg_clientY_cacd4a7e44b9719b(arg0) {
    var ret = getObject(arg0).clientY;
    return ret;
};

export function __wbg_key_10dcaa4bb6d5449f(arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_appendChild_d318db34c4559916() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_newnoargs_be86524d73f67598(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_888d259a5fefc347() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_object_clone_ref(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbg_self_c6fbdfc2918d5e58() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_baec038b5ab35c54() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_3f735a5746d41fbd() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_1bc0b39582740e95() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_undefined(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbg_buffer_397eaa4d72ee94dd(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export function __wbg_new_a7ce447f15ff496f(arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_set_969ad0a60e51d320(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbg_length_1eb8fc608a0d4cdb(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};

export function __wbg_new_8b45a9becdb89691(arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_newwithlength_929232475839a482(arg0) {
    var ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_subarray_8b658422a224f479(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_subarray_9e3273b330900f8c(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_instanceof_Memory_625744f21df3a5ec(arg0) {
    var ret = getObject(arg0) instanceof WebAssembly.Memory;
    return ret;
};

export function __wbindgen_debug_string(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper109(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 23, __wbg_adapter_24);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper111(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 23, __wbg_adapter_27);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper113(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 23, __wbg_adapter_30);
    return addHeapObject(ret);
};

export function __wbindgen_closure_wrapper115(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 23, __wbg_adapter_33);
    return addHeapObject(ret);
};

