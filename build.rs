use std::path::Path;

use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Schema {
    name: String,
    version: u64,
    kind: SchemaKind,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaKind {
    Event,
    StateUpdate,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    name: String,
    physical: PhysicalType,
    logical: Option<LogicalType>,
}

#[derive(Debug, Deserialize)]
pub enum PhysicalType {
    #[serde(rename = "i64")]
    I64,
    #[serde(rename = "bool")]
    Bool,
}

impl PhysicalType {
    pub fn rust_type_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::I64 => quote!(i64),
            Self::Bool => quote!(bool),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum LogicalType {
    Timestamp,
    Fp { scale: u32 },
    Side,
}

impl LogicalType {
    pub fn rust_type_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Timestamp => quote!(crate::primitives::Timestamp),
            Self::Fp { scale } => {
                quote!(crate::primitives::FpInt<#scale>)
            }
            Self::Side => quote!(crate::primitives::Side),
        }
    }
}

fn gen_wire(schema: &Schema) -> proc_macro2::TokenStream {
    let name = format_ident!("Wire{}", schema.name);

    let fields = schema.fields.iter().map(|f| {
        let field_name = format_ident!("{}", f.name);
        let type_ = f.physical.rust_type_tokens();

        quote! {
            pub #field_name: #type_
        }
    });

    quote! {
        pub struct #name {
            #(#fields,)*
        }
    }
}

fn gen_canonical(schema: &Schema) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", schema.name);

    let fields = schema.fields.iter().map(|f| {
        let field_name = format_ident!("{}", f.name);
        let type_ = if let Some(logical) = &f.logical {
            logical.rust_type_tokens()
        } else {
            f.physical.rust_type_tokens()
        };

        quote! {
            pub #field_name: #type_
        }
    });

    quote! {
        pub struct #name {
            #(#fields,)*
        }
    }
}

fn write_tokens(path: impl AsRef<Path>, tokens: proc_macro2::TokenStream) -> anyhow::Result<()> {
    let syntax_tree: syn::File = syn::parse2(tokens)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    std::fs::write(path, formatted)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    //println!("cargo::rerun-if-changed=schemas/");
    let names = ["trade", "lob_update"];

    for name in names {
        let text = std::fs::read_to_string(format!("schemas/{}.toml", name))?;
        let schema: Schema = toml::from_str(&text)?;

        let wire = gen_wire(&schema);
        let canon = gen_canonical(&schema);

        //let out_dir = std::env::var("OUT_DIR")?;
        let out_dir = Path::new("gen_schemas");
        let out_path = Path::new(&out_dir).join(format!("{}_schema.rs", name));

        write_tokens(
            out_path,
            quote! {
                #wire
                #canon
            },
        )?;
    }

    Ok(())
}
