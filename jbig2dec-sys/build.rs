fn main() {
    cc::Build::new()
        .include(".")
        .include("jbig2dec/")
        .files(vec![
            "jbig2dec/jbig2_arith.c",
            "jbig2dec/jbig2_arith_int.c",
            "jbig2dec/jbig2_arith_iaid.c",
            "jbig2dec/jbig2_huffman.c",
            "jbig2dec/jbig2_hufftab.c",
            "jbig2dec/jbig2_segment.c",
            "jbig2dec/jbig2_page.c",
            "jbig2dec/jbig2_symbol_dict.c",
            "jbig2dec/jbig2_text.c",
            "jbig2dec/jbig2_halftone.c",
            "jbig2dec/jbig2_generic.c",
            "jbig2dec/jbig2_refinement.c",
            "jbig2dec/jbig2_mmr.c",
            "jbig2dec/jbig2_image.c",
            "jbig2dec/jbig2.c",
        ])
        .warnings(false)
        .extra_warnings(false)
        .compile("libjbig2dec.a");
    println!("cargo:rustc-link-lib=static=jbig2dec");
}