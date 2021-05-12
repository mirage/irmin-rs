#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

fn irmin_type_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let name = &s.ast().ident;
    let is_record_like = s.variants().len() == 1;
    if is_record_like {
        let encode = s.each(|bi| quote!(count += #bi.encode_bin(dest)?;));

        let decode = s.variants().iter().map(|variant| {
            let construct = variant.construct(|_field, _idx| {
                //let t = &field.ty;
                quote!(irmin::Type::decode_bin(src)?)
            });

            quote! {
                #construct
            }
        });

        s.gen_impl(quote! {
            gen impl irmin::Type for @Self {
                fn encode_bin<W: std::io::Write>(&self, dest: &mut W) -> std::io::Result<usize> {
                    let mut count = 0;
                    match self {
                        #encode
                    }
                    Ok(count)
                }

                fn decode_bin<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                    Ok({
                        #(#decode),*
                    })
                }
            }
        })
        .into()
    } else {
        let mut bindings_index = 0;
        let encode = s.each_variant(|variant| {
            let bindings = variant.bindings();
            if bindings.len() == 0 {
                let q = quote! {
                    count += (#bindings_index as isize).encode_bin(dest)?;
                };
                bindings_index += 1;
                q
            } else {
                let b = &bindings[0];
                let q = quote! {
                    count += (#bindings_index as isize).encode_bin(dest)?;
                    count += #b.encode_bin(dest)?;
                };
                bindings_index += 1;
                q
            }
        });

        let decode = s
            .variants()
            .iter()
            .enumerate()
            .fold(quote!(), |acc, (n, variant)| {
                let construct = if variant.bindings().len() == 0 {
                    variant.construct(|field, _| {
                        quote!({
                            #field
                        })
                    })
                } else {
                    variant.construct(|_field, _idx| quote!({ irmin::Type::decode_bin(src)? }))
                };
                quote! {
                    #acc;

                    if #n == index as usize {
                        return Ok(#construct)
                    }
                }
            });

        s.gen_impl(quote! {
            gen impl irmin::Type for @Self {
                fn encode_bin<W: std::io::Write>(&self, dest: &mut W) -> std::io::Result<usize> {
                    use #name::*;

                    let mut count = 0;
                    match self {
                        #encode
                    }
                    Ok(count)
                }

                fn decode_bin<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                    use #name::*;

                    let index = isize::decode_bin(src)?;
                    #decode

                    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid variant"))
                }
            }
        })
        .into()
    }
}
decl_derive!([IrminType] => crate::irmin_type_derive);
