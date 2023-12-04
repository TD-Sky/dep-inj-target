use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};
use syn::{Fields, ItemStruct};

#[proc_macro_attribute]
pub fn dep_inj_target(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    if let Err(e) = check_target(&item) {
        let error = e.to_compile_error();
        return quote! {
            #item
            #error
        }
        .into();
    }

    let target_struct = target_struct(&item);
    let target_impl_ref_casting = target_impl_ref_casting(&target_struct);
    let target_impl_new = target_impl_new(&target_struct);

    quote! {
        #target_struct
        #target_impl_ref_casting
        #target_impl_new
    }
    .into()
}

fn check_target(item: &ItemStruct) -> syn::Result<()> {
    if !matches!(item.fields, Fields::Unit) {
        return Err(syn::Error::new_spanned(
            item,
            "#[dep_inj_target] requires a unit struct",
        ));
    };

    Ok(())
}

fn target_struct(item: &ItemStruct) -> ItemStruct {
    let vis = &item.vis;
    let ident = &item.ident;
    parse_quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        #[repr(transparent)]
        #vis struct #ident<__Deps__: ?Sized> {
            deps: __Deps__,
        }
    }
}

fn target_impl_ref_casting(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;
    quote! {
        impl<__Deps__: ?Sized> #ident<__Deps__> {
            #[inline]
            pub fn inj_ref(deps: &__Deps__) -> &Self {
                 unsafe { &*(deps as *const __Deps__ as *const Self) }
            }

            #[inline]
            pub fn prj_ref(&self) -> &__Deps__ {
                unsafe { &*(self as *const Self as *const __Deps__) }
            }

            #[inline]
            pub fn inj_ref_mut(deps: &mut __Deps__) -> &mut Self {
                unsafe { &mut*(deps as *mut __Deps__ as *mut Self) }
            }

            #[inline]
            pub fn prj_ref_mut(&mut self) -> &mut __Deps__ {
                unsafe { &mut*(self as *mut Self as *mut __Deps__) }
            }

            #[inline]
            pub fn inj_box(deps: Box<__Deps__>) -> Box<Self> {
                unsafe { Box::from_raw(Box::into_raw(deps) as *mut Self) }
            }

            #[inline]
            pub fn prj_box(self: Box<Self>) -> Box<__Deps__> {
                unsafe { Box::from_raw(Box::into_raw(self) as *mut __Deps__) }
            }

            #[inline]
            pub fn inj_rc(deps: ::std::rc::Rc<__Deps__>) -> ::std::rc::Rc<Self> {
                unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(deps) as *const Self)}
            }

            #[inline]
            pub fn prj_rc(self: ::std::rc::Rc<Self>) -> ::std::rc::Rc<__Deps__> {
                unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(self) as *const __Deps__) }
            }

            #[inline]
            pub fn inj_arc(deps: ::std::sync::Arc<__Deps__>) -> ::std::sync::Arc<Self> {
                unsafe { ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(deps) as *const Self)}
            }

            #[inline]
            pub fn prj_arc(self: ::std::sync::Arc<Self>) -> ::std::sync::Arc<__Deps__> {
                unsafe { ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(self) as *const __Deps__) }
            }

            #[inline]
            pub fn inj_pin_ref(deps: ::core::pin::Pin<&__Deps__>) -> ::core::pin::Pin<&Self> {
                 unsafe {
                    ::core::pin::Pin::new_unchecked(
                        &*(::core::pin::Pin::into_inner_unchecked(deps) as *const __Deps__ as *const Self)
                    )
                }
            }

            #[inline]
            pub fn prj_pin_ref(self: ::core::pin::Pin<&Self>) -> ::core::pin::Pin<&__Deps__> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        &*(::core::pin::Pin::into_inner_unchecked(self) as *const Self as *const __Deps__)
                    )
                }
            }

            #[inline]
            pub fn inj_pin_ref_mut(deps: ::core::pin::Pin<&mut __Deps__>) -> ::core::pin::Pin<&mut Self> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        &mut*(::core::pin::Pin::into_inner_unchecked(deps) as *mut __Deps__ as *mut Self)
                    )
                }
            }

            #[inline]
            pub fn prj_pin_ref_mut(self: ::core::pin::Pin<&mut Self>) -> ::core::pin::Pin<&mut __Deps__> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        &mut*(::core::pin::Pin::into_inner_unchecked(self) as *mut Self as *mut __Deps__)
                    )
                }
            }

            #[inline]
            pub fn inj_pin_box(deps: ::core::pin::Pin<Box<__Deps__>>) -> ::core::pin::Pin<Box<Self>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        Box::from_raw(Box::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *mut Self)
                    )
                }
            }

            #[inline]
            pub fn prj_pin_box(self: ::core::pin::Pin<Box<Self>>) -> ::core::pin::Pin<Box<__Deps__>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        Box::from_raw(Box::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *mut __Deps__)
                    )
                }
            }

            #[inline]
            pub fn inj_pin_rc(deps: ::core::pin::Pin<::std::rc::Rc<__Deps__>>) -> ::core::pin::Pin<::std::rc::Rc<Self>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *const Self)
                    )
                }
            }

            #[inline]
            pub fn prj_pin_rc(self: ::core::pin::Pin<::std::rc::Rc<Self>>) -> ::core::pin::Pin<::std::rc::Rc<__Deps__>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *const __Deps__)
                    )
                }
            }

            #[inline]
            pub fn inj_pin_arc(deps: ::core::pin::Pin<::std::sync::Arc<__Deps__>>) -> ::core::pin::Pin<::std::sync::Arc<Self>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *const Self)
                    )
                }
            }

            #[inline]
            pub fn prj_pin_arc(self: ::core::pin::Pin<::std::sync::Arc<Self>>) -> ::core::pin::Pin<::std::sync::Arc<__Deps__>> {
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *const __Deps__)
                    )
                }
            }
        }
    }
}

fn target_impl_new(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;
    quote! {
        impl<__Deps__> #ident<__Deps__> {
            #[inline]
            pub fn inj(deps: __Deps__) -> Self {
                Self { deps }
            }

            #[inline]
            pub fn prj(self) -> __Deps__ {
                self.deps
            }
        }
    }
}
