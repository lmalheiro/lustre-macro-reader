use proc_macro::{self, *};
use lustre_lib::{object::{Object, RefObject}, reader::{Reader, tokenizer::Tokenizer}};
use std::io::prelude::*;
use std::io::Cursor;

fn ref_object_to_parsable(refobj: &RefObject) -> String {
    if let Some(obj) = refobj.as_ref() {
        format!("Arc::new(Some({}))", object_to_parsable(obj))
    } else {
        "Arc::new(None)".to_string()
    }

}

fn object_to_parsable(obj: &Object) -> String {
    match obj {
        Object::Cons(car,cdr) => {
            format!("Object::Cons({},{})", ref_object_to_parsable(car), ref_object_to_parsable(cdr))
        },
        Object::Integer(value) => format!("Object::Integer({})", value ),
        Object::IString(value) => format!("Object::IString(String::from(\"{}\"))", value ),
        Object::Lambda(_,_) => unimplemented!(),
        Object::Operator(_,_) => unimplemented!(),
        Object::Symbol(value) => format!("Object::Symbol(String::from(\"{}\"))", value ),  
    }.to_string()
}

#[proc_macro]
pub fn lustre(sexp: TokenStream) -> TokenStream {
    let cursor = Cursor::new(sexp.to_string());
    let tokenizer = Tokenizer::new(cursor.bytes());
    let mut reader = Reader::new(tokenizer);
    ref_object_to_parsable(&reader.read().unwrap()).parse().unwrap()
}
