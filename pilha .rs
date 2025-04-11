fn main() {
    let mut pilha: Vec<i32> = Vec::new();
    pilha.push(10);
    pilha.push(20);
    pilha.push(30);
    println!("Pilha atual: {:?}", pilha);

    if let Some(valor_removido) = pilha.pop() {
        println!("Valor removido: {}", valor_removido);
    }

    println!("Pilha após remoção: {:?}", pilha);
}
