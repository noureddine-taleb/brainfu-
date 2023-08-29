#[cfg(test)]
use crate::__execute;

#[test]
fn prog_counter() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    let reti = __execute("   ".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 0);
    assert!(pc == 3);
}

#[test]
fn inc_data_pointer() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    let reti = __execute(">".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(dp == 1);
}

#[test]
#[should_panic]
fn inc_data_pointer2() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    let reti = __execute("<".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
}

#[test]
fn dec_data_pointer() {
    let mut memory = [0_u8; 30000];
    let mut dp = 1_u32;
    let mut pc = 0_u32;

    let reti = __execute("<".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(dp == 0);
}

#[test]
fn inc_data() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    let reti = __execute("+".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(memory[0] == 1);
    memory[0] = 255;
    pc = 0;
    let reti = __execute("+".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(memory[0] == 0);
}

#[test]
fn dec_data() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    memory[0] = 1;
    let reti = __execute("-".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(memory[0] == 0);
    assert!(dp == 0);
    pc = 0;
    let reti = __execute("-".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 1);
    assert!(memory[0] == 255);
    assert!(dp == 0);
}

#[test]
fn if_enter() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    memory[0] = 1;
    let reti = __execute("[>>]".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 4);
    assert!(pc == 4);
    assert!(dp == 2);
}

#[test]
#[should_panic]
fn if_enter2() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    memory[0] = 1;
    __execute("[>>".as_bytes(), &mut memory, &mut dp, &mut pc);
}

#[test]
fn if_goback() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    memory[0] = 2;
    let reti = __execute("[-]".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == 5);
    assert!(pc == 3);
    assert!(dp == 0);
}

#[test]
#[should_panic]
fn if_goback2() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    memory[0] = 2;
    __execute("-]".as_bytes(), &mut memory, &mut dp, &mut pc);
}

#[test]
fn all() {
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    let reti = __execute("-[-]".as_bytes(), &mut memory, &mut dp, &mut pc);
    assert!(reti == (2 + (0xff << 1)), "got reti={reti}");
    assert!(pc == 4);
}
