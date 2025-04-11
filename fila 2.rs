use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();
    fila.push_back(1);
    fila.push_back(2);
    fila.push_back(3);
    println!("Fila atual: {:?}", fila);

    if let Some(valor_removido) = fila.pop_front() {
        println!("Valor removido: {}", valor_removido);
    }

    println!("Fila após remoção: {:?}", fila);
}