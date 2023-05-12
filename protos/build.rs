use std::fs;

fn main() {
    let (protos, includes, our_dir) = v1();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(&our_dir)
        .compile(protos.as_slice(), includes.as_slice())
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", protos.join(","));
}

fn v1() -> (Vec<String>, Vec<String>, String) {
    (
        fs::read_dir("../protofiles/v1")
            .unwrap()
            .map(|f| f.unwrap().path().display().to_string())
            .collect::<Vec<String>>(),
        vec!["../protofiles/v1".to_owned()],
        String::from("./src/v1"),
    )
}
