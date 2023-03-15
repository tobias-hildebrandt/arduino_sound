use ard_r_sound_base::{Length, Note, PitchClass, PitchOrRest};
use ard_r_sound_lib::{codegen::Optimized, parser::parse_abc_file};
use proc_macro2::TokenStream;
use quote::ToTokens;

struct Args {
    var_name: syn::Ident,
    filename: String,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        // the rest can be any token, they are all concatenated into a filename string
        let mut filename = String::new();
        for token in input.parse::<TokenStream>()? {
            filename.push_str(&token.to_string());
        }

        Ok(Args {
            var_name,
            filename,
        })
    }
}

struct NoteWrapper<'a>(&'a Note);
struct PitchOrRestWrapper<'a>(&'a PitchOrRest);
struct PitchClassWrapper<'a>(&'a PitchClass);
struct LengthWrapper<'a>(&'a Length);

impl<'a> ToTokens for NoteWrapper<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let pitch = PitchOrRestWrapper(&self.0.pitch);
        let length = LengthWrapper(&self.0.length);

        tokens.extend(quote::quote! {
            Note {
                pitch: #pitch,
                length: #length,
            }
        })
    }
}

impl<'a> ToTokens for PitchOrRestWrapper<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = match self.0 {
            PitchOrRest::Pitch { class, octave } => {
                let class = PitchClassWrapper(class);
                quote::quote! {
                    Pitch {class: #class, octave: #octave}
                }
            }
            PitchOrRest::Rest => quote::quote! {
                Rest
            },
        };

        tokens.extend(quote::quote! {
            PitchOrRest::#inner
        })
    }
}

impl<'a> ToTokens for PitchClassWrapper<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = match self.0 {
            PitchClass::A => quote::quote!(A),
            PitchClass::ASharpBFlat => quote::quote!(ASharpBFlat),
            PitchClass::B => quote::quote!(B),
            PitchClass::C => quote::quote!(C),
            PitchClass::CSharpDFlat => quote::quote!(CSharpDFlat),
            PitchClass::D => quote::quote!(D),
            PitchClass::DSharpEFlat => quote::quote!(DSharpEFlat),
            PitchClass::E => quote::quote!(E),
            PitchClass::F => quote::quote!(F),
            PitchClass::FSharpGFlat => quote::quote!(FSharpGFlat),
            PitchClass::G => quote::quote!(G),
            PitchClass::GSharpAFlat => quote::quote!(GSharpAFlat),
        };

        tokens.extend(quote::quote! {
            PitchClass::#inner
        })
    }
}

impl<'a> ToTokens for LengthWrapper<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = match self.0 {
            Length::Unit => quote::quote!(Unit),
            Length::Multiple(a) => quote::quote!(Multiple(#a)),
            Length::Division(a) => quote::quote!(Division(#a)),
        };

        tokens.extend(quote::quote! {
            Length::#inner
        })
    }
}

fn note_to_token(note: &Note) -> TokenStream {
    let wrapper = NoteWrapper(note);

    wrapper.into_token_stream()
}

fn notes_to_tokens(notes: Vec<&Note>) -> Vec<TokenStream> {
    notes
        .iter()
        .map(|note| note_to_token(note))
        .collect::<Vec<_>>()
}

#[proc_macro]
pub fn static_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as Args);

    // println!("file: {:?}", args.filename);

    let abc = parse_abc_file(std::path::Path::new(&args.filename)).unwrap();

    let optimized = Optimized::from(&abc);

    let uniques = optimized.uniques;
    let list = optimized.list;

    // println!("{:?}", abc.notes);

    let uniques_len = uniques.len();
    let list_len = list.len();

    let uniques = notes_to_tokens(uniques);

    let var_name = args.var_name;

    let quoted = quote::quote! {
        static #var_name: ard_r_sound_base::OptimizedStatic<#uniques_len, #list_len> = {
            use ard_r_sound_base::{OptimizedStatic, Note, PitchOrRest, PitchClass, Length};

            OptimizedStatic {
                uniques: [
                    #(#uniques),*
                ],
                list: [
                    #(#list),*
                ]
            }
        };
    };

    let output = quoted;

    // println!("output: {:?}", output.to_string());

    proc_macro::TokenStream::from(output)
}
