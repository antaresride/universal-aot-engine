[Latest Release]: https://github.com/antaresride/universal-aot-engine/releases/latest
<div align="center">
  <picture>
    <img src="https://github.com/antaresride/universal-aot-engine/blob/main/src/assets/UniversalAOTEngineLogo.png" width="35%" />
  </picture>
  <div>
    <h1>Universal AOT Engine</h1>
    <i></i>
  </div>
  <br/> 
</div>

## Description
**Universal AOT Engine** - a VM optmized for smart contracts executions's payment.<br><br>
See official documentation here:
##  Features


## Technical Details

┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Source Lang 1  │     │  Source Lang 2  │     │  Source Lang 3  │
│  (Your DSL)     │     │  (Rust macros)  │     │  (Future: WASM) │
│                 │     │                 │     │                 │
│  enum Home {    │     │  enum! { Home   │     │  (imported)     │
│    House(u32),  │     │    House(u32),   │     │                 │
│    Apt(u32)     │     │    Apt(u32) }    │     │                 │
│  }              │     │                 │     │                 │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 ↓
                    ┌─────────────────────┐
                    │   UNIVERSAL CLIF    │
                    │   (Single format)   │
                    │                     │
                    │  function %0(...)   │
                    │    load.i32 ...      │
                    │    return ...        │
                    └──────────┬──────────┘
                               ↓
                    ┌─────────────────────┐
                    │  UNIVERSAL BACKEND  │
                    │  (x64, ARM64, etc.) │
                    │                     │
                    │  mov eax, [rdi]     │
                    │  ret                │
                    └─────────────────────┘
