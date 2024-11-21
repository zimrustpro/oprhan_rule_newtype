#[derive(Clone, Debug)]
struct File(String);

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_bytes = format!("{:?}", &self.0.as_bytes());
        write!(f, "{as_bytes}")
    }
}

fn main() {
    let file = File(String::from("I am file contents"));
    println!("{file:?}");
    println!("{file}");
}
