# destream
Rust library for asynchronous stream (de)serialization

## Default wire representations

`destream` itself defines only an abstract stream encoding API. Concrete codecs may encode types
in different ways, but the default `ToStream`/`FromStream` impls included with `destream` follow
these conventions:

- `i128` / `u128`: encoded as strings (decoded from either a string or an in-range integer token)
- `Duration`: encoded as a 2-tuple `(secs: u64, nanos: u32)` where `nanos < 1_000_000_000`
- `Ipv4Addr` / `Ipv6Addr` / `IpAddr` / `SocketAddr`: encoded as strings using their standard
  textual representation

## Codec authors

- If your format has only 64-bit numeric tokens, prefer decoding `i128`/`u128` via `Decoder::decode_any`
  so callers can supply either:
  - a numeric token (when the value fits in `i64`/`u64`), or
  - a string token (for the full 128-bit range).
- When emitting `Duration`, always emit normalized `nanos` (`< 1_000_000_000`) to preserve a single
  canonical representation across codecs.
