use std::collections::HashMap;

fn main() {
    let mut idades = HashMap::new();
    idades.insert("Alice", 25);
    idades.insert("Bob", 30);
    idades.insert("Carol", 22);

    for (nome, idade) in &idades {
        println!("{} tem {} anos", nome, idade);
    }
}
