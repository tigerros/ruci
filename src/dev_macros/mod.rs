dry_mods::mods! {
    mod pub use define_message, from_str_parts, message_from_impl, impl_message;
}

#[cfg(test)]
mod assert_message_str;
#[cfg(test)]
#[allow(unused_imports)] // rustc is hallucinating
pub use assert_message_str::*;
