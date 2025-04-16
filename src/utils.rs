/// Resolve the value of an optional value. Returns the value if it's Some, otherwise the default.
pub fn resolve_optional_val<T>(val: Option<T>, default: T) -> T {
    if let Some(param) = val {
        param
    } else {
        default
    }
}
