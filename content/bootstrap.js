// Copyright 2018-2025 the Deno authors. MIT license.
/**
 * This module provides the JavaScript interface atop calls to the Rust ops.
 */

// Minimal example, just passes arguments through to Rust:
export function callRust(stringValue) {
  const { op_call_rust } = Deno.core.ops;
  op_call_rust(stringValue);
}

// export const console=new class Console{
//   log(...msg){
//     for(const m of msg){
//       Deno.core.print(`${m} `);
//     }
//     Deno.core.print('\n');
//   }
//   warn(a0){ return this.log(a0) }
//   error(a0){ return this.log(a0) }
// }

export const http=new class Http{
  close(str){ return Deno.core.ops.http_close(str); }
  write(str){ return Deno.core.ops.http_close(str); }
  setHeader(name,value){ return Deno.core.ops.http_set_header(name,value); }
}

