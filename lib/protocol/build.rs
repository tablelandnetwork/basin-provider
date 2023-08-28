fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("src")
        .file("schema/definitions.capnp")
        .file("schema/provider.capnp")
        .output_path("src")
        .default_parent_module(vec!["schema".to_string()])
        .run()
        .expect("protocol compiler command failed");
}
