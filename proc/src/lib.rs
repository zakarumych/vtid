//! Volatile Type ID (Vtid) generator.
//!
//! # How it works
//! 1. Each time a crate is recompiled, a counter in a lock file is incremented to create a new version
//! 2. This version becomes the base ID for all types in that compilation unit
//! 3. When combined with type information, it creates unique IDs that:
//!    - Are consistent for the same type within one compilation
//!    - Automatically version bump on recompilation
//!    - Allow tracking type changes across hot reloads
//!    - Maintain thread-safety through file locking
//!
//! The versioning ensures that if a type's definition changes and the crate is recompiled,
//! the old and new versions of the type will have different IDs.

extern crate proc_macro;

use std::{
    env,
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use fs2::FileExt;
use proc_macro::TokenStream;

struct LockGuard<'a> {
    file: &'a mut File,
}

impl<'a> LockGuard<'a> {
    fn new(file: &'a mut File) -> std::io::Result<Self> {
        file.lock_exclusive()?;
        Ok(Self { file })
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn read_and_update_counter(file: &mut File) -> std::io::Result<u64> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let counter: u64 = content
        .trim()
        .parse()
        .unwrap_or_else(|_| get_unix_timestamp());
    let new_counter = counter + 1;

    file.seek(SeekFrom::Start(0))?;
    file.write_all(new_counter.to_string().as_bytes())?;
    file.sync_all()?;

    Ok(new_counter)
}

fn get_next_counter() -> u64 {
    let lock_path = env!("VTID_PROC_MACRO_LOCK_FILE_PATH");

    let path = PathBuf::from(&lock_path);
    if let Some(parent) = path.parent() {
        let _ = create_dir_all(parent);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Failed to open lock file");

    let _guard = LockGuard::new(&mut file).expect("Failed to lock file");
    read_and_update_counter(&mut *_guard.file).expect("Failed to read and update counter")
}

lazy_static::lazy_static! {
    static ref BASE_ID: u64 = get_next_counter();
}

#[proc_macro_derive(HasVtid)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::DeriveInput);

    let ident = &input.ident;

    let where_clause = input.generics.make_where_clause();
    where_clause.predicates.push(syn::parse_quote!(Self: 'static));

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let where_clause = if let Some(where_clause) = where_clause {
            let mut where_clause = where_clause.clone();
            where_clause.predicates.push(syn::parse_quote!(Self: 'static));
            Some(where_clause)
        } else {
            let where_clause: syn::WhereClause = syn::parse_quote!(where Self: 'static);
            Some(where_clause)
        };

    let base_id = *BASE_ID;

    let tokens = quote::quote! {
        impl #impl_generics ::vtid::HasVtid for #ident #ty_generics #where_clause {
            fn vtid() -> ::vtid::Vtid {
                ::vtid::private::vtid::<Self>(#base_id)
            }
        }
    };

    tokens.into()
}
