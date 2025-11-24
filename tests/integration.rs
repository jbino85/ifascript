use ifascript::IfaVM;

#[test]
fn test_ase_program() {
    let mut vm = IfaVM::new();
    vm.execute(vec!["Èjì Ogbè", "Ìwòrì Méjì", "Ọ̀túúrúpọ̀n"]);
    assert_eq!(vm.stack, vec![1, 1]);
}

#[test]
fn test_cowrie_cast_deterministic_with_fixed_intent() {
    let mut vm1 = IfaVM::with_intent("Test intent");
    let mut vm2 = IfaVM::with_intent("Test intent");

    vm1.execute(vec!["CastCowries"]);
    vm2.execute(vec!["CastCowries"]);

    assert_eq!(vm1.stack[0], vm2.stack[0]);
}
