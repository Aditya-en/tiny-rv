use risc_v::assembler;

#[test]
fn assembler_r_type_add() {
    let encoded = assembler::assemble_add(1, 2, 3);
    let expected = 0b0000000_00011_00010_000_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_i_type_addi() {
    let encoded = assembler::assemble_addi(1, 2, 0x123);

    assert_eq!(encoded & 0x7F, 0b0010011);
    assert_eq!((encoded >> 7) & 0x1F, 1);
    assert_eq!((encoded >> 15) & 0x1F, 2);
    assert_eq!((encoded >> 20) & 0xFFF, 0x123);
}

#[test]
fn assembler_i_type_srai() {
    let encoded = assembler::assemble_srai(1, 2, 3);
    let expected = 0b0100000_00011_00010_101_00001_0010011;
    assert_eq!(encoded, expected);
}


#[test]
fn assembler_r_type_mul() {
    let encoded = assembler::assemble_mul(1, 2, 3);
    let expected = 0b0000001_00011_00010_000_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulh() {
    let encoded = assembler::assemble_mulh(1, 2, 3);
    let expected = 0b0000001_00011_00010_001_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulhsu() {
    let encoded = assembler::assemble_mulhsu(1, 2, 3);
    let expected = 0b0000001_00011_00010_010_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_mulhu() {
    let encoded = assembler::assemble_mulhu(1, 2, 3);
    let expected = 0b0000001_00011_00010_011_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_div() {
    let encoded = assembler::assemble_div(1, 2, 3);
    let expected = 0b0000001_00011_00010_100_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_divu() {
    let encoded = assembler::assemble_divu(1, 2, 3);
    let expected = 0b0000001_00011_00010_101_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_rem() {
    let encoded = assembler::assemble_rem(1, 2, 3);
    let expected = 0b0000001_00011_00010_110_00001_0110011;
    assert_eq!(encoded, expected);
}

#[test]
fn assembler_r_type_remu() {
    let encoded = assembler::assemble_remu(1, 2, 3);
    let expected = 0b0000001_00011_00010_111_00001_0110011;
    assert_eq!(encoded, expected);
}
