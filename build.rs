// Use this in build.rs
fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .includes(&["src"])
        // Inputs must reside in some of include paths.
        .inputs(&["src/RichMsg.proto"])
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();
}
