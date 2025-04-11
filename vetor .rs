fn main() {
    let vetor = vec![100, 200, 300];
    for (indice, valor) in vetor.iter().enumerate() {
        println!("Posição {}: {}", indice, valor);
    }
}
