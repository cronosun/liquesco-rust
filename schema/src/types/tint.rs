use liquesco_common::ine_range::IneRange;
use liquesco_common::int_memory::IntMemory;

/// Common trait for integer types (signed and unsigned).
pub trait TInt<T> {

    fn range(&self) -> &IneRange<T>;

    /// Information about memory representation.
    fn memory(&self) -> IntMemory;
}