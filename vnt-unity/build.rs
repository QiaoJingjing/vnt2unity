fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/vnt.rs")
        .csharp_dll_name("vnt_unity")
        .generate_csharp_file("dotnet/NativeMethods.g.cs")
        .unwrap();
}