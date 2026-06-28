use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input, token::Token};

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let Data::Struct(s) = ast.data else {
        panic!("MyDefault derive macro must use in struct");
    };

    let mut field_ast = quote!();

    for (idx, f) in s.fields.iter().enumerate() {
        let (field_name, field_ty) = (&f.ident, &f.ty);

        if field_name.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {
                #field_idx:#field_ty::default(),
            });
        } else {
            field_ast.extend(quote! {
                #field_name : #field_ty::default(),
            });
        }
    }
    quote! {
        impl Default for #name{
            fn default() -> Self {
                Self {
                    # field_ast
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    // 1.解析为AST语法树
    let input = parse_macro_input!(input as DeriveInput);

    // 2. 获取名称
    let name = &input.ident;

    // 3. 获取结构体字段信息
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect::<Vec<_>>(),
            syn::Fields::Unit => {
                vec![]
            }
        },
        _ => {
            panic!("Describe 只能用于结构体");
        }
    };

    // 4. 生成用于描述的代码
    let fields_descriptions: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field
                .ident
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_else(|| "_".to_string());
            let field_type = &field.ty;
            quote! {
                println!(" - {}: {}",#field_name,stringify!(#field_type));
            }
        })
        .collect();

    // 5. 生成最终代码
    let expanded = quote! {
        impl #name {
            pub fn describe(&self){
                println!("==={}字段信息如下===",stringify!(#name));
                #(#fields_descriptions)*
                println!("===结束===")
            }
        }
    };

    // expanded.into()
    // 6.结束
    TokenStream::from(expanded)
}
