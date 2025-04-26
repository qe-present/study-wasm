import * as wasm from './pkg/synchronous_instantiation.js'
console.log("123123")
self.onmessage=({data: bytes})=>{
    wasm.initSync({module: bytes})
    wasm.greet("synchronous_instantiation")
}
self.postMessage({type:"FETCH_WASM"})// 发生消息 获取wasm
