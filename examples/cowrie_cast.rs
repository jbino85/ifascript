use ifascript::IfaVM;

fn main() {
    let mut vm = IfaVM::with_intent("Ẹbọ: I seek clarity and truth");

    vm.execute(vec!["Èjì Ogbè", "CastCowries", "Ìwòrì Méjì", "Ọ̀túúrúpọ̀n"]);

    if vm.stack.len() >= 2 {
        let cowries = vm.stack[1] as u16;
        println!("Cowries cast: {:016b}", cowries);
        println!("{} heads, {} tails", cowries.count_ones(), 16 - cowries.count_ones());
    }

    println!("Àṣẹ sealed.");
}
