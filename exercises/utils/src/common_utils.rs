pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}
