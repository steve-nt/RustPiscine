// The `<T>` right after the function name declares the generic type.
// We then use `T` as the type for the parameter `v` and the return type.
pub fn identity<T>(v: T) -> T {
    v
}