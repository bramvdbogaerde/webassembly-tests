# Converting WAST to WAT

<div align="center">
⚠️ This repository is not working yet, it contains ideas but not working code. ⚠️
</div>

## Notes

I could not quickly find a tool that converts WAST (e.g. from the [reference Wasm test suite][reference-wasm-test-suite]) to WAT, while preserving the assertions as part of the WASM instructions.

When using the reference WASM interpreter, [WebAssembly/spec/interpreter][reference-wasm-interpreter] to convert a WAST file to a WAT file (as is performed [in this script][reference-wasm-usage]), the assertions seem to be lost.

The [wat (parser) library][bytecode-aliance-wat] from the bytecode aliance does not parse WAST files, however the [wast (parser) library] from the bytecode aliance does!

---

[reference-wasm-interpreter]: https://github.com/WebAssembly/spec/tree/228549757b301c698c5588ec0d58b201b5777c92/interpreter
[reference-wasm-test-suite]: https://github.com/WebAssembly/spec/tree/228549757b301c698c5588ec0d58b201b5777c92/test/core
[reference-wasm-usage]: https://github.com/danleh/wasabi/blob/fe12347f3557ca1db64b33ce9a83026143fa2e3f/test-inputs/wasm-spec-tests/build.sh
[bytecode-aliance-wat]: https://github.com/bytecodealliance/wasm-tools/tree/acb410e70ea57ffe93853f37d49e366acb3b4962/crates/wat
[bytecode-aliance-wast]: https://github.com/bytecodealliance/wasm-tools/tree/acb410e70ea57ffe93853f37d49e366acb3b4962/crates/wast