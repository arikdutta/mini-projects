use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn secured_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let struct_name = fn_name.to_string().to_case(Case::Pascal);
    let struct_ident = format_ident!("{}", struct_name);

    let args_str = args.to_string();
    let state_param = if args_str.contains("state_param") {
        args_str
            .split("state_param = \"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .map(|s| format_ident!("{}", s))
    } else {
        None
    };

    let validation_code = if let Some(param_name) = state_param {
        quote! {
            let auth_session = crate::common::app_state::get_session()
                .map_err(|e| leptos::prelude::ServerFnError::new(&e.to_string()))?;
            let user = auth_session
                .current_user
                .ok_or_else(|| leptos::prelude::ServerFnError::new("Failed to retrieve user session"))?;
            <#struct_ident as crate::domain::auth::access_controller::AccessController>::check_permission_with_state(&user, #param_name)
                .await?;
        }
    } else {
        quote! {
            let auth_session = crate::common::app_state::get_session()
                .map_err(|e| leptos::prelude::ServerFnError::new(&e.to_string()))?;
            let user = auth_session
                .current_user
                .ok_or_else(|| leptos::prelude::ServerFnError::new("Failed to retrieve user session"))?;
            <#struct_ident as crate::domain::auth::access_controller::AccessController>::check_permission(&user)?;
        }
    };

    let filtered_args = if args_str.contains("state_param") {
        TokenStream::new()
    } else {
        args
    };

    let original_body = input_fn.block.clone();
    *input_fn.block = syn::parse_quote! {
        {
            #validation_code
            #original_body
        }
    };

    let filtered_args = proc_macro2::TokenStream::from(filtered_args);
    let output = quote! {
        #[::leptos::server(#filtered_args)]
        #input_fn

        // Enforce bound
        // This const ensures that the actual T type implements AccessController.
        // Otherwise this is dead code.
        // The const should not have a name as it is reused in each instance of the macro.
        const _: () = {
            fn dummy_function_to_enforce_t_implements_access_controller<T: crate::domain::auth::access_controller::AccessController>() {}

            fn check_that_the_actual_t_is_an_access_controller() {
                dummy_function_to_enforce_t_implements_access_controller::<#struct_ident>();
            }
        };
    };
    output.into()
}
