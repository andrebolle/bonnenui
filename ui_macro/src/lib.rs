use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, FnArg, ItemFn, Pat};
use heck::ToPascalCase;

#[proc_macro_attribute]
pub fn generate_ui(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    
    // Capitalize the function name (e.g., 'add' -> 'AddDialog')
    let struct_name_str = format!("{}Dialog", fn_name.to_string().to_pascal_case());
    let wrapper_name = format_ident!("{}", struct_name_str);
    let name_str = fn_name.to_string();

    let args = input.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(ident) = &*pat_type.pat { 
                Some(&ident.ident) 
            } else { None }
        } else { None }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        #input

        #[derive(Default, Clone)]
        pub struct #wrapper_name { 
            #(#args: i32),* }

        impl common::Command for #wrapper_name {
            fn name(&self) -> &str { 
                #name_str 
            }
            
            fn show(&mut self, ui: &mut eframe::egui::Ui) -> Option<i32> {
                ui.heading(stringify!(#fn_name));
                
                #(
                    ui.horizontal(|ui| {
                        ui.label(stringify!(#args));
                        ui.add(eframe::egui::DragValue::new(&mut self.#args));
                    });
                )*
                
                if ui.button("Execute").clicked() {
                    return Some(#fn_name(#(self.#args),*));
                }
                None
            }
        }
    };
    expanded.into()
}