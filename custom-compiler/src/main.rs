

// It seems HIR is where to look in.
// https://github.com/rust-lang/rustc-guide/blob/master/src/hir.md
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc/hir/map/definitions/struct.Definitions.html

#![feature(rustc_private)]
extern crate rustc_interface;
extern crate rustc_driver;
extern crate rustc;

use rustc_interface::*;
use rustc_driver::Callbacks;
use rustc::session::config::*;
use rustc::session::*;
use rustc::session::search_paths::*;
//use std::Option::*;
use std::path::*;
use std::string::String;

struct Example1 {
    field1: String,
}

fn main() {
    let src_path = PathBuf::from("/Users/Dev/Workshop/eonil/rust-howto-examples/custom-compiler/src/main.rs");
    let sysroot_path = Path::new("/Users/Dev/.rustup/toolchains/nightly-x86_64-apple-darwin");
    let triple = "x86_64-apple-darwin";
    let mut search_path = SearchPath::from_sysroot_and_triple(sysroot_path, triple);
    let mut search_paths: Vec<SearchPath> = vec!(search_path);
    let mut sess_opts = config::Options::default();
    sess_opts.search_paths = search_paths;

    let conf = interface::Config {
        opts: sess_opts,
        crate_cfg: Default::default(),
        input: Input::File(src_path),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        diagnostic_output: DiagnosticOutput::Default,
        stderr: None,
        crate_name: None,
        lint_caps: Default::default(),
    };
    interface::run_compiler(conf, |compiler| {
        /*
        let q = match compiler.parse() {
            Ok(x) => x,
            Err(_) => return,
        };
        let x = q.take();
        println!("{:#?}", x);
        */
        let x = match compiler.lower_to_hir() {
            Ok(x) => x,
            Err(_) => return,
        };
        let z = x.take().0.steal();
        let k = z.krate();
        //println!("{:#?}", k);
        print_crate(k);
        ()
    });
}

struct ShimCalls;

impl Callbacks for ShimCalls {
    fn config(&mut self, config: &mut interface::Config) {
        config.opts.debugging_opts.continue_parse_after_error = true;
        config.opts.debugging_opts.save_analysis = true;
    }
}

// https://doc.rust-lang.org/nightly/nightly-rustc/rustc/hir/struct.Crate.html
// https://rust-lang.github.io/rustc-guide/hir.html
fn print_crate(k: &rustc::hir::Crate) {
    for (_,item) in &k.items {
        print_item(item);
    }
}
fn print_item(item: &rustc::hir::Item) {
    println!("item: hir_id = {}, name = {}, ", item.hir_id, item.ident.to_string());
    print_item_kind(&item.kind);
}
fn print_item_kind(item_kind: &rustc::hir::ItemKind) {
    use rustc::hir::ItemKind::*;
    match item_kind {
        Struct(ref vd, _) => {
            print_variant(vd);
        },
        _ => return,
    }    
}
fn print_variant(vd: &rustc::hir::VariantData) {
    use rustc::hir::VariantData::*;
    match vd {
        Struct(fields,_) => for field in fields {            
            print_struct_field(field);
        }
        _ => return,
    }
}
fn print_struct_field(field: &rustc::hir::StructField) {
    let name = field.ident.to_string();
    println!("field: name = {}", name);
    print_ty(&*field.ty);
}
fn print_ty(ty: &rustc::hir::Ty) {
    println!("ty: hir_id = {}", ty.hir_id);
    print_ty_kind(&ty.kind);
}
fn print_ty_kind(ty_kind: &rustc::hir::TyKind) {
    use rustc::hir::TyKind::*;
    println!("{:#?}", ty_kind);
    match ty_kind {
        Path(qpath) => {
            print_qpath(qpath);
        },
        _ => return,
    }
}
fn print_qpath(qpath: &rustc::hir::QPath) {
    use rustc::hir::QPath::*;
    match qpath {
        Resolved(ref maybe_ty, ref path) => {
            println!("{} {}", path, path.segments.len());
            return;
            match maybe_ty {
                None => return,
                Some(ty) => print_ty(&*ty), 
            }
        },
        TypeRelative(ref ty, ref path_segment) => {
            print_ty(&ty);
        },
    }
}

