
fn main() {
    tonic_build::compile_protos("../proto/nuun.proto").unwrap();
}