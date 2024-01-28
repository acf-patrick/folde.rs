pub trait Transpile {
    fn transpile(&mut self) -> std::io::Result<String>;
}