extern crate gcc;

fn main () {
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
