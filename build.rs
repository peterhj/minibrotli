#[cfg(feature = "fresh")]
extern crate bindgen;
extern crate cc;

#[cfg(feature = "fresh")]
use std::env;
#[cfg(feature = "fresh")]
use std::fs;
#[cfg(feature = "fresh")]
use std::path::{PathBuf};

#[cfg(feature = "fresh")]
fn gen_bindings() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let gensrc_dir = manifest_dir.join("gensrc");
  fs::create_dir_all(&gensrc_dir).ok();
  fs::remove_file(gensrc_dir.join("_brotli.rs")).ok();
  bindgen::Builder::default()
    .clang_arg(format!("-I{}", manifest_dir.join("brotli/c/include").display()))
    .header("wrapped_brotli.h")
    .trust_clang_mangling(false)
    .whitelist_recursively(false)
    .whitelist_type("brotli_alloc_func")
    .whitelist_type("brotli_free_func")
    .whitelist_type("BrotliDecoderStateStruct")
    .whitelist_type("BrotliDecoderState")
    .whitelist_type("BrotliDecoderResult")
    .whitelist_type("BrotliDecoderErrorCode")
    .whitelist_type("BrotliDecoderParameter")
    .whitelist_function("BrotliDecoderSetParameter")
    .whitelist_function("BrotliDecoderCreateInstance")
    .whitelist_function("BrotliDecoderDestroyInstance")
    .whitelist_function("BrotliDecoderDecompress")
    .whitelist_function("BrotliDecoderDecompressStream")
    .whitelist_function("BrotliDecoderHasMoreOutput")
    .whitelist_function("BrotliDecoderTakeOutput")
    .whitelist_function("BrotliDecoderIsUsed")
    .whitelist_function("BrotliDecoderIsFinished")
    .whitelist_function("BrotliDecoderGetErrorCode")
    .whitelist_function("BrotliDecoderErrorString")
    .whitelist_function("BrotliDecoderVersion")
    .whitelist_var("BROTLI_DEFAULT_QUALITY")
    .whitelist_var("BROTLI_DEFAULT_WINDOW")
    .whitelist_var("BROTLI_MIN_QUALITY")
    .whitelist_var("BROTLI_MAX_QUALITY")
    .whitelist_var("BROTLI_MIN_WINDOW_BITS")
    .whitelist_var("BROTLI_MAX_WINDOW_BITS")
    .whitelist_type("BrotliEncoderMode")
    .whitelist_type("BrotliEncoderOperation")
    .whitelist_type("BrotliEncoderParameter")
    .whitelist_type("BrotliEncoderStateStruct")
    .whitelist_type("BrotliEncoderState")
    .whitelist_function("BrotliEncoderSetParameter")
    .whitelist_function("BrotliEncoderCreateInstance")
    .whitelist_function("BrotliEncoderDestroyInstance")
    .whitelist_function("BrotliEncoderMaxCompressedSize")
    .whitelist_function("BrotliEncoderCompress")
    .whitelist_function("BrotliEncoderCompressStream")
    .whitelist_function("BrotliEncoderIsFinished")
    .whitelist_function("BrotliEncoderHasMoreOutput")
    .whitelist_function("BrotliEncoderTakeOutput")
    .whitelist_function("BrotliEncoderVersion")
    .generate_comments(false)
    .prepend_enum_name(false)
    .rustfmt_bindings(true)
    .generate()
    .expect("bindgen failed to generate brotli bindings")
    .write_to_file(gensrc_dir.join("_brotli.rs"))
    .expect("bindgen failed to write brotli bindings");
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  let mut cc = cc::Build::new();
  if cfg!(feature = "fresh") {
    cc.warnings_into_errors(true);
  }
  cc.pic(true)
    .opt_level(3)
    .include("brotli/c/common")
    .include("brotli/c/include")
    .file("brotli/c/common/dictionary.c")
    .file("brotli/c/common/transform.c")
    .file("brotli/c/dec/bit_reader.c")
    .file("brotli/c/dec/decode.c")
    .file("brotli/c/dec/huffman.c")
    .file("brotli/c/dec/state.c")
    .file("brotli/c/enc/backward_references.c")
    .file("brotli/c/enc/backward_references_hq.c")
    .file("brotli/c/enc/bit_cost.c")
    .file("brotli/c/enc/block_splitter.c")
    .file("brotli/c/enc/brotli_bit_stream.c")
    .file("brotli/c/enc/cluster.c")
    .file("brotli/c/enc/compress_fragment.c")
    .file("brotli/c/enc/compress_fragment_two_pass.c")
    .file("brotli/c/enc/dictionary_hash.c")
    .file("brotli/c/enc/encode.c")
    .file("brotli/c/enc/encoder_dict.c")
    .file("brotli/c/enc/entropy_encode.c")
    .file("brotli/c/enc/histogram.c")
    .file("brotli/c/enc/literal_cost.c")
    .file("brotli/c/enc/memory.c")
    .file("brotli/c/enc/metablock.c")
    .file("brotli/c/enc/static_dict.c")
    .file("brotli/c/enc/utf8_util.c")
    .compile("libbrotli.a");

  #[cfg(feature = "fresh")]
  gen_bindings();
}
