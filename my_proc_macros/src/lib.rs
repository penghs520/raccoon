extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote; // quote 宏用于生成 Rust 代码
use syn::{parse_macro_input, DeriveInput, Type}; // syn 库用于解析和处理 Rust 代码

#[proc_macro_derive(Getter)] // 定义一个过程宏，名为 Getter
pub fn getter_derive(input: TokenStream) -> TokenStream {
    // 将输入的 TokenStream 解析为 DeriveInput 类型
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体的标识符（名称）
    let name = &input.ident;

    // 获取结构体的泛型参数（包括生命周期参数）
    let generics = &input.generics;

    // 将泛型参数拆分为用于实现的部分、用于类型的部分和可能的约束
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 获取结构体的字段
    let fields = if let syn::Data::Struct(ref data) = input.data {
        // 确保结构体的字段是命名字段（即具有字段名）
        if let syn::Fields::Named(ref fields) = data.fields {
            &fields.named
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    };

    // 为每个字段生成 getter 方法
    let getters = fields.iter().map(|f| {
        let ident = &f.ident; // 获取字段的标识符（名称）
        let ty = &f.ty; // 获取字段的类型
        match ty {
            // 如果字段类型是引用类型
            Type::Reference(_) => quote! {
                pub fn #ident(&self) -> #ty {
                    self.#ident
                }
            },
            // 如果字段类型不是引用类型
            _ => quote! {
                pub fn #ident(&self) -> &#ty {
                    &self.#ident
                }
            },
        }
    });

    // 生成最终的实现代码
    let expanded = quote! {
        // 实现块，包含泛型参数和可能的约束
        impl #impl_generics #name #ty_generics #where_clause {
            //将每个生成的 getter 方法插入到实现块中
            //这是quote! 宏中的一种特殊语法，#( ... )*: 重复插值模式，展开括号内的内容，对每个元素应用括号内的模式，并用空格分隔。
            //#(#getters)* 表示将一个迭代器中的每个元素展开并插入到生成的代码中。
            #(#getters)*
        }
    };

    // 将生成的代码转换回 TokenStream 并返回
    TokenStream::from(expanded)
}
