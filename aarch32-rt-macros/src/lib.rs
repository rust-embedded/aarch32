//! Macros for the aarch32-rt library
//!
//! Provides `#[entry]`, `#[exception(...)]` and `#[irq]` attribute macros.
//!
//! Do not use this crate directly.
//!
//! Based on <https://github.com/rust-embedded/cortex-m/tree/c-m-rt-v0.7.5/cortex-m-rt/macros>.

extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse, parse_macro_input, spanned::Spanned, AttrStyle, Attribute, Ident, ItemFn, ReturnType,
    Type, Visibility,
};

/// Creates an `unsafe` program entry point (i.e. a `kmain` function).
///
/// It's `unsafe` because you are not supposed to call it - it should only be
/// called from the start-up code once initialisation is complete.
///
/// When placed on a function like:
///
/// ```rust ignore
/// #[entry]
/// fn foo() -> ! {
///     panic!("On no")
/// }
/// ```
///
/// You get something like:
///
/// ```rust
/// #[doc(hidden)]
/// #[export_name = "kmain"]
/// pub unsafe extern "C" fn __aarch32_rt_kmain() -> ! {
///     foo()
/// }
///
/// fn foo() -> ! {
///     panic!("On no")
/// }
/// ```
///
/// The symbol `kmain` is what the assembly code in aarch32-rt start-up code
/// will jump to, and the `extern "C"` makes it sound to call from assembly.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function signature.
    //
    // it should be `fn foo() -> !` or `unsafe fn foo() -> !`
    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // This is the name that other Rust code needs to use to call this function -
    // we make it long an complicated because no-one is supposed to call this function.
    // The `__aarch32_rt` prefix re-inforces this.
    //
    // However, this is not the symbol that the linker sees - we override that to be
    // `kmain` because that's what the start-up assembly code is looking for. As you
    // cannot call that symbol without using `extern "C" { }`, it should be sufficiently
    // well hidden.
    let tramp_ident = Ident::new("__aarch32_rt_kmain", Span::call_site());
    let block = f.block;

    if let Err(error) = check_attr_whitelist(&f.attrs, VectorKind::Entry) {
        return error;
    }

    let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

    quote!(
        #(#cfgs)*
        #(#attrs)*
        #[doc(hidden)]
        #[export_name = "kmain"]
        pub unsafe extern "C" fn #tramp_ident() -> ! {
            #block
        }
    )
    .into()
}

/// The set of exceptions we can handle.
#[derive(Debug, PartialEq)]
enum Exception {
    Undefined,
    SupervisorCall,
    HypervisorCall,
    PrefetchAbort,
    DataAbort,
    Irq,
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::Undefined => write!(f, "Undefined"),
            Exception::SupervisorCall => write!(f, "SupervisorCall"),
            Exception::HypervisorCall => write!(f, "HypervisorCall"),
            Exception::PrefetchAbort => write!(f, "PrefetchAbort"),
            Exception::DataAbort => write!(f, "DataAbort"),
            Exception::Irq => write!(f, "Irq"),
        }
    }
}

/// Creates an `unsafe` exception handler.
///
/// It's `unsafe` because you are not supposed to call it - it should only be
/// called from assembly routines registered in the interrupt vector table.
///
/// When placed on a function like:
///
/// ```rust ignore
/// #[exception(Undefined)]
/// fn foo(addr: usize) -> ! {
///     panic!("On no")
/// }
/// ```
///
/// You get something like:
///
/// ```rust
/// #[doc(hidden)]
/// #[export_name = "_undefined_handler"]
/// pub unsafe extern "C" fn __aarch32_rt_undefined_handler(addr: usize) -> ! {
///     foo(addr)
/// }
///
/// fn foo(addr: usize) -> ! {
///     panic!("On no")
/// }
/// ```
///
/// The supported arguments are:
///
/// * Undefined (creates `_undefined_handler`)
/// * SupervisorCall (creates `_svc_handler`)
/// * HypervisorCall (creates `_hvc_handler`)
/// * PrefetchAbort (creates `_prefetch_abort_handler`)
/// * DataAbort (creates `_data_abort_handler`)
/// * Irq (creates `_irq_handler`) - although people should prefer `#[irq]`.
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    handle_vector(args, input, VectorKind::Exception)
}

/// Creates an `unsafe` interrupt handler.
///
/// It's `unsafe` because you are not supposed to call it - it should only be
/// called from assembly routines registered in the interrupt vector table.
///
/// When placed on a function like:
///
/// ```rust ignore
/// #[irq]
/// fn foo(addr: usize) -> ! {
///     panic!("On no")
/// }
/// ```
///
/// You get something like:
///
/// ```rust
/// #[doc(hidden)]
/// #[export_name = "_irq_handler"]
/// pub unsafe extern "C" fn __aarch32_rt_irq_handler(addr: usize) -> ! {
///     foo(addr)
/// }
///
/// fn foo(addr: usize) -> ! {
///     panic!("On no")
/// }
/// ```
///
/// This is preferred over `#[exception(Irq)` because most people
/// probably won't consider interrupts to be a form of exception.
#[proc_macro_attribute]
pub fn irq(args: TokenStream, input: TokenStream) -> TokenStream {
    handle_vector(args, input, VectorKind::Interrupt)
}

/// Note if we got `#[entry]`, `#[exception(...)]` or `#[irq]`
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum VectorKind {
    /// Corresponds to `#[entry]`
    Entry,
    /// Corresponds to `#[exception(...)]`
    Exception,
    /// Corresponds to `#[irq]`
    Interrupt,
}

/// A common routine for handling exception or interrupt functions
fn handle_vector(args: TokenStream, input: TokenStream, kind: VectorKind) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if let Err(error) = check_attr_whitelist(&f.attrs, kind) {
        return error;
    }

    let returns_never = match f.sig.output {
        ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        _ => false,
    };

    let exception = match kind {
        VectorKind::Entry => {
            panic!("Don't handle #[entry] with `handle_vector`!");
        }
        VectorKind::Exception => {
            let mut args_iter = args.into_iter();
            let Some(TokenTree::Ident(exception_name)) = args_iter.next() else {
                return parse::Error::new(
                    Span::call_site(),
                    "This attribute requires the name of the exception as the first argument",
                )
                .to_compile_error()
                .into();
            };
            if args_iter.next().is_some() {
                return parse::Error::new(
                    Span::call_site(),
                    "This attribute accepts only one argument",
                )
                .to_compile_error()
                .into();
            }
            match exception_name.to_string().as_str() {
                "Undefined" => {
                    if !returns_never && f.sig.unsafety.is_none() {
                        return parse::Error::new(
                            exception_name.span().into(),
                            "Undefined handlers that don't return ! must be unsafe",
                        )
                        .to_compile_error()
                        .into();
                    }
                    Exception::Undefined
                }
                "SupervisorCall" => Exception::SupervisorCall,
                "HypervisorCall" => Exception::HypervisorCall,
                "PrefetchAbort" => {
                    if !returns_never && f.sig.unsafety.is_none() {
                        return parse::Error::new(
                            exception_name.span().into(),
                            "PrefetchAbort handlers that don't return ! must be unsafe",
                        )
                        .to_compile_error()
                        .into();
                    }
                    Exception::PrefetchAbort
                }
                "DataAbort" => {
                    if !returns_never && f.sig.unsafety.is_none() {
                        return parse::Error::new(
                            exception_name.span().into(),
                            "DataAbort handlers that don't return ! must be unsafe",
                        )
                        .to_compile_error()
                        .into();
                    }
                    Exception::DataAbort
                }
                "Irq" => Exception::Irq,
                _ => {
                    return parse::Error::new(
                        exception_name.span().into(),
                        "This is not a valid exception name",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        VectorKind::Interrupt => Exception::Irq,
    };

    let func_name = f.sig.ident.clone();
    let block = f.block.clone();
    let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

    let handler = match exception {
        // extern "C" fn _undefined_handler(addr: usize) -> !;
        // unsafe extern "C" fn _undefined_handler(addr: usize) -> usize;
        Exception::Undefined => {
            let tramp_ident = Ident::new("__aarch32_rt_undefined_handler", Span::call_site());
            if returns_never {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_undefined_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> ! {
                        #block
                    }

                )
            } else {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_undefined_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> usize {
                        #block
                    }

                )
            }
        }
        // extern "C" fn _prefetch_abort_handler(addr: usize) -> !;
        // unsafe extern "C" fn _prefetch_abort_handler(addr: usize) -> usize;
        Exception::PrefetchAbort => {
            let tramp_ident = Ident::new("__aarch32_rt_prefetch_abort_handler", Span::call_site());
            if returns_never {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_prefetch_abort_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> ! {
                        #block
                    }
                )
            } else {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_prefetch_abort_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> usize {
                        #block
                    }

                )
            }
        }
        // extern "C" fn _data_abort_handler(addr: usize) -> !;
        // unsafe extern "C" fn _data_abort_handler(addr: usize) -> usize;
        Exception::DataAbort => {
            let tramp_ident = Ident::new("__aarch32_rt_data_abort_handler", Span::call_site());
            if returns_never {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_data_abort_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> ! {
                        #block
                    }

                )
            } else {
                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[export_name = "_data_abort_handler"]
                    pub unsafe extern "C" fn #tramp_ident(addr: usize) -> usize {
                        #block
                    }
                )
            }
        }
        // extern "C" fn _svc_handler(arg: u32, args: &Frame) -> u32
        Exception::SupervisorCall => {
            let tramp_ident = Ident::new("__aarch32_rt_svc_handler", Span::call_site());
            quote!(
                #(#cfgs)*
                #(#attrs)*
                #[doc(hidden)]
                #[export_name = "_svc_handler"]
                pub unsafe extern "C" fn #tramp_ident(arg: u32, frame: &aarch32_rt::Frame) -> u32 {
                    #f

                    #func_name(arg, frame)
                }
            )
        }
        // extern "C" fn _hvc_handler(arg: u32, args: &Frame) -> u32;
        Exception::HypervisorCall => {
            let tramp_ident = Ident::new("__aarch32_rt_hvc_handler", Span::call_site());
            quote!(
                #(#cfgs)*
                #(#attrs)*
                #[doc(hidden)]
                #[export_name = "_hvc_handler"]
                pub unsafe extern "C" fn #tramp_ident(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
                    #f

                    #func_name(hsr, frame)
                }
            )
        }
        // extern "C" fn _irq_handler(addr: usize);
        Exception::Irq => {
            let tramp_ident = Ident::new("__aarch32_rt_irq_handler", Span::call_site());
            quote!(
                #(#cfgs)*
                #(#attrs)*
                #[doc(hidden)]
                #[export_name = "_irq_handler"]
                pub unsafe extern "C" fn #tramp_ident() {
                    #block
                }
            )
        }
    };

    quote!(
        #handler
    )
    .into()
}

/// Given a list of attributes, split them into `cfg` and non-`cfg`.
///
/// Returns `(cfgs, non_cfgs)`.
fn extract_cfgs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

/// Check whether any disallowed attributes have been applied to our entry/exception function.
fn check_attr_whitelist(attrs: &[Attribute], caller: VectorKind) -> Result<(), TokenStream> {
    let whitelist = &[
        "doc",
        "link_section",
        "cfg",
        "allow",
        "warn",
        "deny",
        "forbid",
        "cold",
        "naked",
        "expect",
    ];

    'o: for attr in attrs {
        // Also check whatever is wrapped inside `unsafe(...)`
        if attr.path().is_ident("unsafe") {
            let mut whitelisted = false;
            let _ = attr.parse_nested_meta(|meta| {
                for val in whitelist {
                    if meta.path.is_ident(val) {
                        whitelisted = true;
                    }
                }
                Ok(())
            });
            if whitelisted {
                continue 'o;
            }
        }
        for val in whitelist {
            if eq(attr, val) {
                continue 'o;
            }
        }

        let err_str = match caller {
            VectorKind::Entry => "this attribute is not allowed on an aarch32-rt entry point",
            VectorKind::Exception => {
                "this attribute is not allowed on an exception handler controlled by aarch32-rt"
            }
            VectorKind::Interrupt => {
                "this attribute is not allowed on an interrupt handler controlled by aarch32-rt"
            }
        };

        return Err(parse::Error::new(attr.span(), err_str)
            .to_compile_error()
            .into());
    }

    Ok(())
}

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &Attribute, name: &str) -> bool {
    attr.style == AttrStyle::Outer && attr.path().is_ident(name)
}
