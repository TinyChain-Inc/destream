# destream
Rust library for asynchronous stream (de)serialization.

Use `FromStream`, `ToStream`, and `IntoStream` for ordinary typed values;
fixed-schema types can implement visitors and delegate their fields to these traits.

For data whose nesting is determined by input, use **iterative traversal with an
explicit frame stack**. This avoids call-stack exhaustion through uncontrolled
recursion ([CWE-674](https://cwe.mitre.org/data/definitions/674.html)). Depth and
allocation limits remain necessary to bound heap usage and work. Recursive value
owners must also make destruction and other structural operations stack-safe.

- **Decoding uses a container cursor:** `peek_kind`, `open_container`, and
  `finish_child` let the owner drive its frame stack and delegate typed leaves
  through `FromStream`. Complete each child before advancing its cursor; discard
  the decoder after failed or interrupted traversal.
- **Encoding uses a pull-based structural event stream:** `encode_events`
  consumes sequence/map starts, leaves, and container ends. For example,
  `[[1], 2]` emits `SeqStart, SeqStart, Value(1), End, Value(2), End` (size hints
  omitted). `Event::Value` delegates a leaf's ordinary encoding; recursive
  containers must be emitted as events. The codec retains one active leaf stream,
  accepting borrowed input without requiring `Unpin` or `'static`.

Events describe traversal; stack safety comes from the iterative implementation.
Arbitrary recursive visitors or leaf encoders do not become stack-safe automatically.

Each codec owns root/container validation, byte framing, and leaf encoding.
`inspect_any` supplies structural observations
without constructing decoded payloads; value owners supply allocation policy.
