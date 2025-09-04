use syn::{DeriveInput, GenericArgument, LitInt, PathArguments, Type, TypePath};

pub fn is_string_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(tp) => {
            let ident = &tp.path.segments.last().unwrap().ident;
            ident == "String"
        }
        syn::Type::Reference(r) => {
            if let syn::Type::Path(tp) = &*r.elem {
                tp.path.segments.last().unwrap().ident == "str"
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn is_numeric_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(typepath) => {
            let ident = &typepath.path.segments.last().unwrap().ident;
            matches!(
                ident.to_string().as_str(),
                "u8" | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "f32"
                    | "f64"
            )
        }
        _ => false,
    }
}

pub fn is_compatible(typename: &str, lit: &LitInt) -> Result<(), ()> {
    let suffix = lit.suffix(); // np. "u8", "i32", "f64", albo ""
    let type_prefix = &typename[0..1]; // "u", "i" albo "f"

    // brak sufiksu = traktujemy jak i32
    let lit_prefix = if suffix.is_empty() {
        "i"
    } else {
        &suffix[0..1]
    };

    if match type_prefix {
        "u" | "i" => lit_prefix == "i" || lit_prefix == "u",
        "f" => lit_prefix == "f",
        _ => false, // inne typy nas nie interesują
    } {
        Ok(())
    } else {
        Err(())
    }
}

pub fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(_) = &args.args[0] {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// let mut res = false;
// for attr in &input.attrs {
//     if attr.path().is_ident("derive") {
//         attr.parse_nested_meta(|meta| {
//             if meta.path.is_ident("Validate") {
//                 res = true;
//             }
//             Ok(())
//         })
//         .unwrap();
//     }
// }
// res

// for attr in &input.attrs {
//     if attr.path().is_ident("derive") {
//         // if let Meta::List(list) = &attr.meta {
//         //     println!("{:#?}", list);
//         // }

//         // if let Ok(list) = attr.parse_args_with(parser) {
//         //     for meta in list {
//         //         if let Meta::Path(path) = meta {
//         //             if path.is_ident("Validate") {
//         //                 return true;
//         //             }
//         //         }
//         //     }
//         // }
//     }
// }
pub fn is_validated(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("validate"))
}
