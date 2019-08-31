use envopt::EnvOpt;

#[derive(EnvOpt)]
pub enum EnvOpts {
    #[envopt(name = "FOO")]
    Foo,
    #[envopt(name = "BAR", default = "default-bar")]
    Bar,
}

pub fn main() {
    EnvOpts::validate_or_exit();

    println!("FOO: {}", EnvOpts::Foo.value_or_exit());
    println!("BAR: {}", EnvOpts::Bar.value_or_exit());
}
