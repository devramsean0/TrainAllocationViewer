fn main() {
    cynic_codegen::register_schema("api")
        .from_sdl_file("schemas/api.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
