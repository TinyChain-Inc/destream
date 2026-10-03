/// One structural event in a pull-based encoding stream.
///
/// Recursive value owners emit these steps using an explicit frame stack;
/// codecs supply byte framing and consume one leaf stream at a time. Events
/// describe traversal but do not themselves make recursive code stack-safe.
pub enum Event<T> {
    SeqStart(Option<usize>),
    MapStart(Option<usize>),
    /// A complete leaf, delegated to its ordinary [`super::IntoStream`] implementation.
    /// Emit recursive containers as events instead of hiding them in this variant.
    Value(T),
    End,
}
