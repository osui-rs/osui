fn main() {
    cc::Build::new()
        .file("src/app.c").compile("app");
}