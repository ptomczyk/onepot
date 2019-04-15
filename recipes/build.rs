fn main() {
    prost_build::compile_protos(&["src/recipes.proto"],
                                &["src/"]).unwrap();
}