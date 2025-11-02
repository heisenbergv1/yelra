# yelra

> A next-gen systems language built on Rust’s shoulders—without its ceilings.
> Our aim: **fearless low-level power** with **high-level ergonomics**, **determinism across async**, and **first-class effects**—all while staying **compile-time safe** and **predictably fast**.

---

## Why yelra?

We love Rust. We also run into recurring pain points across modern languages:

* **Async + ownership is still awkward.** You either leak lifetimes into futures or retreat to `Arc<Mutex<…>>`.
* **Effects and capability safety are bolted on.** Exceptions/logging/IO/cancellation are runtime conventions, not types.
* **State protocols are ad-hoc.** Typestate exists in libraries but isn’t ergonomic or checked deeply.
* **Units, time, and resource budgets** are not first-class; they live in comments until something breaks.
* **Build/packaging drift** erodes determinism; “works on my machine” is still a meme.

yelra is an experiment to close those gaps without sacrificing the properties that made Rust great.

---

## Design Pillars

1. **Zero-overhead abstractions**: what you don’t use, you don’t pay for—at runtime or in binary size.
2. **Ownership across time**: lifetimes that compose with async/await, tasks, and actors without `Arc` sprawl.
3. **Typed effects**: IO, logging, randomness, cancellation, and domain effects modeled at the type level.
4. **Protocol-level safety**: typestate and session types that are ergonomic and optimizable.
5. **Deterministic concurrency**: structural concurrency + cancellation that cannot be accidentally dropped.
6. **Practical interop**: frictionless FFI with Rust/C/C++, plus “shape-safe” interop with TypeScript and WASM.
7. **Reproducible toolchain**: hermetic builds by default; version-locked standard library and packages.

---

## What’s new (or fixes gaps elsewhere)

### 1) Temporal Ownership (TOwn)

**Problem:** Rust’s lifetimes capture *spatial* ownership well, but they don’t travel smoothly through async boundaries.

**yelra:** a **temporal ownership** model that:

* Treats `await` as a type-level suspension point tracked by the compiler.
* Allows borrowing across `await` only when **provably non-aliased** or **pinned by design**.
* Eliminates many `Arc<Mutex>` escapes; favors **linear capabilities** that move across tasks safely.

```yelra
async fn fetch<'t>(cap: Net<'t, Send>, url: Url) -> Bytes
    effects(Net)
{
    let mut conn = await cap.open(url);    // ownership travels across await
    await conn.send("GET / HTTP/1.1\r\n\r\n");
    return await conn.read_all();
}
```

### 2) Algebraic Effects with Capability Types

**Problem:** Exceptions/logging/IO are hidden global side effects.

**yelra:** **effect sets** are part of a function’s type; handlers are zero-cost where possible.

```yelra
fn parse_config(src: Bytes) -> Config effects(Decode, Env);

with effects {
  handle Env with { get(k) => env_table[k] }
  handle Decode with { json(b) => simdjson(b) }
} run {
  let cfg = parse_config(input);
}
```

* Callers can **restrict** or **swap** effects (e.g., test mode).
* No hidden IO: binaries compile without linking handlers you don’t use.

### 3) Ergonomic Typestate & Protocols

**Problem:** Correct API usage is often comment-driven.

**yelra:** **protocol blocks** compile into optimized state machines; pattern matching is state-safe.

```yelra
protocol File {
  state Closed;
  state Open;

  fn open(path: Path) -> Self in Closed -> Open;
  fn read(self: &mut Open, n: usize) -> Bytes;
  fn close(self: Open) -> Closed;
}

fn cat(p: Path) -> Bytes {
  let mut f = File::open(p);
  let b = f.read(4096);
  f.close();     // compile error if not closed on all branches (deterministic cleanup)
  b
}
```

### 4) Units of Measure & Time Safety

**Problem:** Milliseconds vs seconds vs “mystery ints” still cause outages.

**yelra:** **dimension types** and **timezones** in the core type system.

```yelra
let dt: Duration<ms> = 250.ms();
sleep(2.s());        // mismatched units won’t compile
let when: Zoned<UTC> = now();
```

### 5) Structural Concurrency + Cancellation

**Problem:** Detached tasks leak resources; cancellation is best-effort.

**yelra:** `scope {}` spawns tasks that **must** complete or be handled before scope exit; cancellation is **typed**.

```yelra
scope |s| {
  let t1 = s.spawn(fetch(a, u1));
  let t2 = s.spawn(fetch(b, u2));
  let both = await try_join(t1, t2) on cancel { cleanup()? };
  return both;
}
```

### 6) Resource Budgets at the Type Level

**Problem:** Memory/CPU/IO budgets are runtime convention.

**yelra:** optional **budget types** propagate through APIs and are checked for exhaust paths.

```yelra
fn render(scene: &Scene, b: Budget<cpu: 8ms, mem: 32MB>) -> Frame;
```

### 7) Shape-Safe FFI & WASM

* **Rust-FFI**: `extern "yelra/rust"` ensures ABI-safe shapes verified at compile time.
* **WASM 2.0-ready**: `#[wasm(export)]` emits minimal shims; effects can be handler-injected by the host.
* **TypeScript bridge**: generate `.d.ts` with structural typing from yelra records and sum types.

---

## Syntax Snapshot

```yelra
module http

effect Net { open(Url) -> Conn; }

protocol Conn {
  state New; state Ready; state Closed;

  fn send(self: &mut Ready, Bytes) -> ();
  fn read_all(self: &mut Ready) -> Bytes;
  fn close(self: Ready) -> Closed;
}

record Config { endpoint: Url, timeout: Duration<ms> }

async fn get(cfg: &Config, path: Str) -> Bytes effects(Net) {
  let url = cfg.endpoint.join(path);
  with Net as net {
    let mut c = await net.open(url);
    await c.send(b"GET / HTTP/1.1\r\n\r\n");
    let body = await c.read_all();
    c.close();
    body
  }
}
```

---

## Toolchain

* **Compiler**: `ylc` (front-end in Rust; MIR-like mid-tier; LLVM/Cranelift back-ends).
* **Package manager**: `ylpm` (hermetic, content-addressed, lockfile mandatory).
* **Build system**: `ylb` (incremental, graph-aware, remote-cache-friendly).
* **Formatter/LSP**: `ylfmt`, `ylls` with semantic rename, effect/borrow hovers, protocol visualizer.
* **Test/Bench**: `yltest`, `ylbench` with flamegraph and effect coverage.

---

## Roadmap

### Phase 0.x — Foundations

* **0.1 Lexer/Parser**: concrete syntax, AST, source maps.
* **0.2 Type system MVP**: algebraic data types, traits, generics, trait impl coherence.
* **0.3 Borrow/Move core**: Rust-compatible ownership semantics; MIR lowerer.
* **0.4 Temporal Ownership**: lifetime analysis across `await` and task scopes.
* **0.5 Effects MVP**: effect sets, handler syntax, monomorphization rules.
* **0.6 Protocols/Typestate**: stateful types, exhaustiveness, optimizer passes.
* **0.7 Units & Time**: built-in dimensions, timezones, compile-time conversions.
* **0.8 Concurrency**: `scope`, `join/try_join`, typed cancellation, async runtime ABI.
* **0.9 Tooling**: `ylls`, `ylfmt`, diagnostics, IDE integration; `ylpm` lockfile.
* **0.10 Interop**: Rust FFI, C ABI, basic WASM export/import.
* **0.11 Stdlib (pre-1.0)**: collections, io, net (effect-based), time, task.
* **0.12 Reproducible builds**: hermetic compiler + std pinned by `toolchain.toml`.

### 1.0 — Stability Targets

* Language spec v1: temporal ownership, effects, protocols, units.
* Tier-1 platforms: Linux (x86_64, aarch64), Windows (x86_64), macOS (aarch64).
* WASM target stable; TS bindings generator 1.0.
* `unsafe` story clearly bounded and audited.

### Post-1.0 Explorations

* **Session types over the network** (protocols compiled to wire schemas).
* **Deterministic scheduling mode** for simulation/replay.
* **Hot-reload** of effect handlers for observability.
* **Lightweight green threads** with stack slicing where profitable.
* **Region-based memory** for arenas with escape analysis.

---

## Comparative Notes

* **Rust**: we keep ownership/borrowing/perf; add **temporal ownership**, **typed effects**, and **ergonomic typestate**.
* **Haskell/OCaml**: we borrow algebraic effects and purity inspiration, but aim for **predictable performance** and **explicit resources**.
* **Zig**: we admire simplicity; yelra chooses **stronger static guarantees** (effects, units, protocols).
* **Go**: structured concurrency and ease are goals, but with **no hidden panics** and **typed cancellation**.
* **Kotlin/Swift**: result builders/DSL ergonomics inform protocols/effects syntax, without ARC overhead.

---

## Performance & Safety Expectations

* Effect handlers erase to direct calls where resolvable at compile time; otherwise devirtualize aggressively.
* Protocol automata lower to branchless state indices where patterns allow.
* Temporal ownership prevents most async-driven `Arc` cloning; fewer atomics, less contention.
* Units/time types are zero-sized at runtime—entirely compile-time enforced.

---

## Getting Started (pre-alpha)

```bash
# bootstrap toolchain (hermetic)
curl -fsSL https://get.yelra.dev | bash

ylpm init hello-yelra
cd hello-yelra

# format, build, run
ylfmt .
ylb build
ylb run
```

> Tooling stubs will compile a reference interpreter; the optimizing back-end lands during 0.8–0.10.

---

## Example: Effects in Tests

```yelra
test "parses config from env" {
  let input = b"{\"endpoint\":\"https://api\",\"timeout_ms\":250}";
  handle Env with { get(_) => None } run {
    handle Decode with { json(b) => pure_json(b) } run {
      let cfg = parse_config(input);
      assert_eq!(cfg.timeout, 250.ms());
    }
  }
}
```

---

## Contributing

We love precise experiments and sharp edges. To contribute:

1. Open an **RFC** issue describing a gap and a minimal design sketch.
2. Provide **motivating real-world cases** (benchmarks, bug classes, outage stories).
3. Keep the **zero-cost bar** in mind: if it adds runtime overhead, justify it with data.

**Areas to help:**

* Temporal ownership borrow-checker rules
* Effect handler lowering & optimization
* Protocol (typestate) syntax → MIR lowering
* Units/time dimension checker
* LSP diagnostics & quick-fixes
* Repro build sandboxing

---

## Governance & License

* **License:** Apache-2.0 (draft)
* **Decision making:** lightweight RFC process; compiler team triages monthly; breaking changes require two release deprecation windows post-1.0.

---

## FAQ (working notes)

* **Is this a Rust fork?** No. It’s a Rust-adjacent language with its own front-end and IR; we interoperate with Rust seamlessly.
* **Will it replace `Arc`/`Mutex`?** Not entirely. We reduce pressure to use them by making cross-time ownership first-class.
* **Are effects a performance tax?** Handlers erase statically when possible; otherwise we devirtualize. Benchmarks will gate features.
* **Why not macros?** We lean on **real types** (effects, protocols, units) to prevent “type erasure by macro.”

---

## Roadmap Board

* [ ] 0.1 Parser + AST
* [ ] 0.2 Type system MVP
* [ ] 0.3 Ownership core
* [ ] 0.4 Temporal ownership across async
* [ ] 0.5 Effects MVP + handlers
* [ ] 0.6 Protocols/Typestate
* [ ] 0.7 Units/Time
* [ ] 0.8 Concurrency runtime + scopes/cancel
* [ ] 0.9 Tooling (LSP/Fmt)
* [ ] 0.10 Interop (Rust/C/WASM/TS)
* [ ] 1.0 Spec + stabilization

---

# Comparative Landscape: yelra vs. Today’s Majors

> This section is standalone—drop it into the README as-is. It frames where yelra sits relative to C, C++, C#, Java, JavaScript/TypeScript, and Python, focusing on memory, concurrency, effects, protocols/typestate, units/time, interop, and build determinism.

---

## At-a-Glance Matrix

| Axis                  | **C**               | **C++**                         | **C#**                             | **Java**                          | **JS/TS**                   | **Python**                      | **yelra**                                       |
| --------------------- | ------------------- | ------------------------------- | ---------------------------------- | --------------------------------- | --------------------------- | ------------------------------- | ----------------------------------------------- |
| Memory & Ownership    | Manual malloc/free  | RAII + smart ptrs; UB landmines | GC (configurable spans/stackalloc) | GC                                | GC                          | GC                              | Ownership + **temporal** lifetimes              |
| Concurrency Model     | pthreads            | std::thread, atomics            | async/await, TPL                   | virtual threads (Loom), executors | async/await + event loop    | asyncio/threads                 | **Structured** concurrency + typed cancellation |
| Effects / Error Model | errno, return codes | exceptions (costly), status     | exceptions + `IAsyncEnumerable`    | checked/unchecked exceptions      | exceptions, dynamic         | exceptions, dynamic             | **Algebraic effects** + capability types        |
| Protocol/Typestate    | manual discipline   | libraries/templates             | state machines by convention       | by convention                     | by convention               | by convention                   | **First-class protocols/typestate**             |
| Units/Time Safety     | integers            | libraries                       | libraries                          | libraries                         | libraries                   | libraries                       | **Dimension types** + timezones                 |
| Interop               | ABI king            | ABI king (fragile templates)    | P/Invoke, source gen               | JNI, Panama                       | WASM/FFI via host           | C-ABI, C-extensions             | Rust/C ABI, TS/WASM with **shape-safe** checks  |
| Deterministic Builds  | possible, painful   | possible, painful               | NuGet + SDK pinning                | Maven/Gradle pinning              | npm lockfiles (noisy trees) | venv/poetry/lockfiles           | **Hermetic toolchain** + mandatory lockfile     |
| Performance Envelope  | Peak + footguns     | Peak + complexity               | High with GC                       | High with GC                      | Medium (JIT/engines)        | Low–Medium (C extensions boost) | **Zero-cost abstractions**; AOT                 |

---

## Language-by-Language Notes

### C

**What it nails:** Minimal runtime, predictable ABI, tiny binaries, ubiquitous FFI.
**Where it bites:** Manual memory, aliasing hazards, no generics, error handling via return codes/`errno`, concurrency is low-level and footgun-rich.
**yelra stance:** We keep C’s predictability but replace footguns with **ownership + borrow checking** and **typed effects** (no hidden IO). Concurrency is **structured** with cancellable scopes, and protocol state machines are typed. FFI remains straightforward for systems work; we generate C headers and keep ABIs boring.

**Migration hints:** Ports of leaf libraries (parsers, codecs) become slim yelra modules. Effects allow us to stub IO during tests instead of #ifdef mazes.

---

### C++

**What it nails:** Performance, RAII, value semantics, template metaprogramming, huge ecosystem.
**Where it bites:** UB traps, template complexity, exceptions with nontrivial cost semantics, fragmented build systems, evolving coroutines story, protocol safety by convention.
**yelra stance:** Value semantics and zero-overhead are non-negotiable, but we replace exception machinery with **algebraic effects** and **result types** that erase to direct calls. **Temporal ownership** cleans up coroutine/async lifetime puzzles without spraying `shared_ptr`. **Protocols** make stateful APIs (files, sockets, GPU contexts) statically correct.

**Migration hints:** Think “RAII → ownership + effects”; “variant/expected → sum types/results”; “coroutines → async with temporal lifetimes.” Build determinism is a default, not a team sport.

---

### C#

**What it nails:** Productive tooling, async/await ergonomics, ecosystem, modern language features, P/Invoke interop, source generators.
**Where it bites:** GC pauses (mitigable but present), hidden allocations in async paths, exceptions as control flow, typestate by convention, concurrency cancellation often ad-hoc tokens.
**yelra stance:** We embrace the good async ergonomics but make **cancellation typed and scoped**, forbid hidden heap churn by default, and surface IO/logging/randomness as **capability effects**. Memory is explicit; no GC surprises in hot paths. Protocol/typestate is first-class to prevent misuse of services/clients.

**Migration hints:** Controllers/handlers map well: `async Task<ActionResult>` → `async fn(...) -> Result<T> effects(...)`. Replace DI singletons with capability injection (effects). Interop with .NET lives via C ABI or WASI services.

---

### Java

**What it nails:** Stability, portability, mature tooling, now with **virtual threads** (Project Loom).
**Where it bites:** Checked vs unchecked exceptions confusion, GC tuning requirements, reflection-heavy frameworks, units/time safety via libraries only, protocol safety by convention.
**yelra stance:** We adopt Loom’s spirit—**cheap concurrency**—but keep it **structured** and **deterministic** where requested. Effects replace sprawling exception hierarchies. Units/time are types, not comments. Builds are hermetic out of the box.

**Migration hints:** Replace annotation-driven DI with explicit capability parameters; swap exception pyramids for `Result<T, E>` or effect handlers. Use our TS/WASM bridge for JVM/JS interop at the edges.

---

### JavaScript / TypeScript

**What it nails:** Developer velocity, async I/O ergonomics, huge ecosystem, WASM friendliness, ubiquitous distribution.
**Where it bites:** Type erasure at runtime, error handling via exceptions, performance cliffs, units/time by convention, concurrency limited to event loops/workers, non-deterministic build graphs.
**yelra stance:** We aim for **TS-like ergonomics** with **static guarantees** and **AOT performance**. Effects model the same IO/DOM-like powers but are **capability-scoped**, testable, and optimizable. Our **TS bridge** emits `.d.ts` from yelra types so frontends keep their shape safety without runtime baggage.

**Migration hints:** Keep the UI in TS/React/Vue; compile core logic to WASM with yelra, exporting typed functions. Effects allow us to stub networking/storage in browser tests cleanly.

---

### Python

**What it nails:** Expressivity, batteries-included, scientific ecosystem, fantastic prototyping.
**Where it bites:** Single-threaded CPU performance (GIL), runtime type errors, async pitfalls with resource ownership, units/time by libraries only, packaging repeatability is fragile without discipline.
**yelra stance:** We keep Python’s **expressive feel** in APIs (records, pattern matching, sum types) but compile to **predictable native code**. Effects capture filesystem/network/random dependencies explicitly for reproducible research and robust services. Units/time are enforced at compile time.

**Migration hints:** Keep notebooks for exploration; push hot paths and reliability-critical cores to yelra via FFI or WASM. Use effect handlers to emulate environment dependencies during tests.

---

## Where yelra Intentionally Differs

* **Effects Instead of Exceptions:** We make side-effects explicit and swappable. This turns “global powers” (IO, randomness, logging) into **values** that compilers can eliminate or route—no surprise cross-cuts.
* **Temporal Ownership:** Async boundaries are first-class in the type system, so lifetimes cross `await` sanely without defaulting to atomics or GC.
* **Protocol/Typestate as Syntax:** State machines are part of types. APIs declare their legal transitions; misuse won’t compile. Optimizer lowers them to efficient automata.
* **Units/Time Are Built-in:** Dimensions and zones exist at the type level; conversions are compile-time checked and zero-cost at runtime.
* **Determinism by Default:** Toolchain pinning and hermetic builds are standard; CI and local dev produce bit-for-bit compatible artifacts where the platform allows.

---

## Interop & Migration Story

* **Rust/C/C++:** Header generation and **shape-safe** verification guard ABIs. Call into existing kernels, drivers, codecs without drama.
* **WASM + TypeScript:** Export yelra modules to WASM with generated `.d.ts` and thin JS glue; effects can be provided by the host (browser, Node, Deno).
* **Service Meshes:** Protocol definitions double as wire schemas. We can compile protocol types to IDLs (e.g., Protobuf/Cap’n Proto) while preserving typestate invariants.

---

## When to Choose What

* **Choose C/C++** for exotic hardware, committee-blessed ABIs, or when every byte and cycle must be hand-tuned and team culture absorbs UB risk.
* **Choose C#/Java** for enterprise platforms with deep framework gravity and GC-friendly latency budgets.
* **Choose JS/TS** for UI and glue across the web’s runtime.
* **Choose Python** for research, scripting, ML prototyping.
* **Choose yelra** when we want **Rust-class performance** with **async-aware ownership**, **typed effects**, **protocol safety**, and **deterministic builds**—especially for services, SDKs, game/engine subsystems, and WASM cores that must be fast, testable, and dependency-tame.

---
