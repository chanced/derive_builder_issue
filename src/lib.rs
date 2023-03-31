#[derive(derive_builder::Builder)]
struct Example {
    #[builder(try_setter, setter(strip_option))]
    value: Option<String>,
}
