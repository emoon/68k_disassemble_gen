extern crate rand;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum SP {
    n,    // normal
    s,    // static operand
    r,    // register operand
    rr,   // register to register
    mm,   // memory to memory
    er,   // effective address to register
    re,   // register to effective address
    dd,   // data register to data register
    da,   // data register to address register
    di,   // address register indirect with displacement
    al,   // absolute long address
    aa,   // address register to address register
    cr,   // control register to register
    rc,   // register to control register
    aw,   // absolute word address
    pd,   // address register indirect with predecrement
    pi,   // address register indirect with postincrement
    ix,   // address register indirect with index
    ai,   // address register indirect
    d,    // data register
    pd7,  // ????
    pi7,  // ????
    toc,  // to condition code register
    tos,  // to status register
    tou,  // to user stack pointer
    frc,  // from condition code register
    frs,  // from status register
    fru,  // from user stack pointer
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum SE {
    n,    // normal
    i,    // immediate
    d,    // data register
    a,    // address register
    ai,   // address register indirect
    pi,   // address register indirect with postincrement
    pd,   // address register indirect with predecrement
    di,   // address register indirect with displacement
    ix,   // address register indirect with index
    aw,   // absolute word address
    al,   // absolute long address
    pcdi, // program counter relative with displacement
    pcix, // program counter relative with index
    a7,   // register specified in instruction is A7
    ax7,  // register field X of instruction is A7
    ay7,  // register field Y of instruction is A7
    axy7, // register fields X and Y of instruction are A7
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Inst {
    name: &'static str,
    size: u16, 
    sp: SP,
    se: SE,
    bitp: &'static str,
    aea: &'static [u8; 10],
    cycles: [u32; 3],
}

/*
allowed ea:  List of allowed addressing modes:
                 .: not present
                 A: address register indirect
                 +: ARI with postincrement
                 -: ARI with predecrement
                 D: ARI with displacement
                 X: ARI with index
                 W: absolute word address
                 L: absolute long address
                 d: program counter indirect with displacement
                 x: program counter indirect with index
                 I: immediate
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/*
fn generate_ea(eae: &'static str) -> String {
    let modes = 0;

    for t in eae {
        if t = '.' {
            break;
        }

        modes += 1;
    }

}
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct NewInst {
    name: &'static str,
    bitp: &'static str,
    aea: &'static str,
    cycles: [u32; 3],
}

/*
let inst_list = [ 
		NewInst { name: "add", bitp: "1101DDD000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		NewInst { name: "add", bitp: "1101DDD000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
];
*/
/*
                 A: address register indirect
                 +: ARI with postincrement
                 -: ARI with predecrement
                 D: ARI with displacement
                 X: ARI with index
                 W: absolute word address
                 L: absolute long address
                 d: program counter indirect with displacement
                 x: program counter indirect with index
                 I: immediate
*/

fn get_static_addressing_mode(aea: &[u8], index: usize) -> Option<&'static str> {
    match aea[index] {
        b'A' => Some("(a0)"),
        b'+' => Some("(a0)+"),
        b'-' => Some("-(a0)"),
        b'D' => Some("42(a0)"),
        _ => None,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

    let test = [
        /*
		Inst { name: "1111",     size:  0, sp: SP::n,   se: SE::n,    bitp: "1111............", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "abcd",     size:  8, sp: SP::rr,  se: SE::n,    bitp: "1100...100000...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::ax7,  bitp: "1100111100001...", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::ay7,  bitp: "1100...100001111", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::axy7, bitp: "1100111100001111", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::n,    bitp: "1100...100001...", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "add",      size:  8, sp: SP::er,  se: SE::d,    bitp: "1101...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		*/
		Inst { name: "add",      size:  8, sp: SP::er,  se: SE::n,    bitp: "1101...000......", aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2] },
		/*
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::d,    bitp: "1101...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::a,    bitp: "1101...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::n,    bitp: "1101...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::d,    bitp: "1101...010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::a,    bitp: "1101...010001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::n,    bitp: "1101...010......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "add",      size:  8, sp: SP::re,  se: SE::n,    bitp: "1101...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "add",      size: 16, sp: SP::re,  se: SE::n,    bitp: "1101...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "add",      size: 32, sp: SP::re,  se: SE::n,    bitp: "1101...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1101...011000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::a,    bitp: "1101...011001...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1101...011......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  2] },
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::d,    bitp: "1101...111000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::a,    bitp: "1101...111001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::n,    bitp: "1101...111......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "addi",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0000011000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "addi",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0000011000......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "addi",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0000011001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "addi",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000011001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "addi",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0000011010000...", aea: "..........", cycles: [16, 14,  2] },
		Inst { name: "addi",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000011010......", aea: "A+-DXWL...", cycles: [20, 20,  4] },
		Inst { name: "addq",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0101...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "addq",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0101...000......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0101...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::a,    bitp: "0101...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0101...001......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0101...010000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::a,    bitp: "0101...010001...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0101...010......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "addx",     size:  8, sp: SP::rr,  se: SE::n,    bitp: "1101...100000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "addx",     size: 16, sp: SP::rr,  se: SE::n,    bitp: "1101...101000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "addx",     size: 32, sp: SP::rr,  se: SE::n,    bitp: "1101...110000...", aea: "..........", cycles: [ 8,  6,  2] },
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::ax7,  bitp: "1101111100001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::ay7,  bitp: "1101...100001111", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::axy7, bitp: "1101111100001111", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::n,    bitp: "1101...100001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "addx",     size: 16, sp: SP::mm,  se: SE::n,    bitp: "1101...101001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "addx",     size: 32, sp: SP::mm,  se: SE::n,    bitp: "1101...110001...", aea: "..........", cycles: [30, 30, 12] },
		Inst { name: "and",      size:  8, sp: SP::er,  se: SE::d,    bitp: "1100...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "and",      size:  8, sp: SP::er,  se: SE::n,    bitp: "1100...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "and",      size: 16, sp: SP::er,  se: SE::d,    bitp: "1100...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "and",      size: 16, sp: SP::er,  se: SE::n,    bitp: "1100...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "and",      size: 32, sp: SP::er,  se: SE::d,    bitp: "1100...010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "and",      size: 32, sp: SP::er,  se: SE::n,    bitp: "1100...010......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "and",      size:  8, sp: SP::re,  se: SE::n,    bitp: "1100...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "and",      size: 16, sp: SP::re,  se: SE::n,    bitp: "1100...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "and",      size: 32, sp: SP::re,  se: SE::n,    bitp: "1100...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "andi",     size: 16, sp: SP::toc, se: SE::n,    bitp: "0000001000111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "andi",     size: 16, sp: SP::tos, se: SE::n,    bitp: "0000001001111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "andi",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0000001000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "andi",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0000001000......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "andi",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0000001001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "andi",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000001001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "andi",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0000001010000...", aea: "..........", cycles: [14, 14,  2] },
		Inst { name: "andi",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000001010......", aea: "A+-DXWL...", cycles: [20, 20,  4] },
		Inst { name: "asr",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...000000...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "asr",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...001000...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "asr",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...010000...", aea: "..........", cycles: [ 8,  8,  6] },
		Inst { name: "asr",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...000100...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "asr",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...001100...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "asr",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...010100...", aea: "..........", cycles: [ 8,  8,  6] },
		Inst { name: "asr",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110000011......", aea: "A+-DXWL...", cycles: [ 8,  8,  5] },
		Inst { name: "asl",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...100000...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "asl",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...101000...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "asl",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...110000...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "asl",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...100100...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "asl",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...101100...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "asl",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...110100...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "asl",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110000111......", aea: "A+-DXWL...", cycles: [ 8,  8,  6] },
		Inst { name: "bcc",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0110............", aea: "..........", cycles: [ 8,  8,  6] },
		Inst { name: "bcc",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0110....00000000", aea: "..........", cycles: [10, 10,  6] },
		Inst { name: "bcc",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0110....11111111", aea: "..........", cycles: [10, 10,  6] },
		Inst { name: "bchg",     size:  8, sp: SP::r,   se: SE::n,    bitp: "0000...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "bchg",     size: 32, sp: SP::r,   se: SE::d,    bitp: "0000...101000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "bchg",     size:  8, sp: SP::s,   se: SE::n,    bitp: "0000100001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "bchg",     size: 32, sp: SP::s,   se: SE::d,    bitp: "0000100001000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "bclr",     size:  8, sp: SP::r,   se: SE::n,    bitp: "0000...110......", aea: "A+-DXWL...", cycles: [ 8, 10,  4] },
		Inst { name: "bclr",     size: 32, sp: SP::r,   se: SE::d,    bitp: "0000...110000...", aea: "..........", cycles: [10, 10,  4] },
		Inst { name: "bclr",     size:  8, sp: SP::s,   se: SE::n,    bitp: "0000100010......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "bclr",     size: 32, sp: SP::s,   se: SE::d,    bitp: "0000100010000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfchg",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110101011000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfchg",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110101011......", aea: "A..DXWL...", cycles: [14, 14,  4] },
		Inst { name: "bfclr",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110110011000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfclr",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110110011......", aea: "A..DXWL...", cycles: [14, 14,  4] },
		Inst { name: "bfexts",   size: 32, sp: SP::n,   se: SE::d,    bitp: "1110101111000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfexts",   size: 32, sp: SP::n,   se: SE::n,    bitp: "1110101111......", aea: "A..DXWLdx.", cycles: [14, 14,  4] },
		Inst { name: "bfextu",   size: 32, sp: SP::n,   se: SE::d,    bitp: "1110100111000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfextu",   size: 32, sp: SP::n,   se: SE::n,    bitp: "1110100111......", aea: "A..DXWLdx.", cycles: [14, 14,  4] },
		Inst { name: "bfffo",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110110111000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfffo",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110110111......", aea: "A..DXWLdx.", cycles: [14, 14,  4] },
		Inst { name: "bfins",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110111111000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfins",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110111111......", aea: "A..DXWL...", cycles: [14, 14,  4] },
		Inst { name: "bfset",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110111011000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bfset",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110111011......", aea: "A..DXWL...", cycles: [14, 14,  4] },
		Inst { name: "bftst",    size: 32, sp: SP::n,   se: SE::d,    bitp: "1110100011000...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bftst",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1110100011......", aea: "A..DXWLdx.", cycles: [14, 14,  4] },
		Inst { name: "bkpt",     size:  0, sp: SP::n,   se: SE::n,    bitp: "0100100001001...", aea: "..........", cycles: [14, 14,  4] },
		Inst { name: "bra",      size:  8, sp: SP::n,   se: SE::n,    bitp: "01100000........", aea: "..........", cycles: [10, 10, 10] },
		Inst { name: "bra",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0110000000000000", aea: "..........", cycles: [10, 10, 10] },
		Inst { name: "bra",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0110000011111111", aea: "..........", cycles: [10, 10, 10] },
		Inst { name: "bset",     size: 32, sp: SP::r,   se: SE::d,    bitp: "0000...111000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "bset",     size:  8, sp: SP::r,   se: SE::n,    bitp: "0000...111......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "bset",     size:  8, sp: SP::s,   se: SE::n,    bitp: "0000100011......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "bset",     size: 32, sp: SP::s,   se: SE::d,    bitp: "0000100011000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "bsr",      size:  8, sp: SP::n,   se: SE::n,    bitp: "01100001........", aea: "..........", cycles: [18, 18,  7] },
		Inst { name: "bsr",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0110000100000000", aea: "..........", cycles: [18, 18,  7] },
		Inst { name: "bsr",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0110000111111111", aea: "..........", cycles: [18, 18,  7] },
		Inst { name: "btst",     size:  8, sp: SP::r,   se: SE::n,    bitp: "0000...100......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  4] },
		Inst { name: "btst",     size: 32, sp: SP::r,   se: SE::d,    bitp: "0000...100000...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "btst",     size:  8, sp: SP::s,   se: SE::n,    bitp: "0000100000......", aea: "A+-DXWLdx.", cycles: [ 8,  8,  4] },
		Inst { name: "btst",     size: 32, sp: SP::s,   se: SE::d,    bitp: "0000100000000...", aea: "..........", cycles: [10, 10,  4] },
		Inst { name: "callm",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0000011011......", aea: "A..DXWLdx.", cycles: [10, 10,  4] },
		Inst { name: "cas",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0000101011......", aea: "A+-DXWL...", cycles: [10, 10,  4] },
		Inst { name: "cas",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0000110011......", aea: "A+-DXWL...", cycles: [10, 10,  4] },
		Inst { name: "cas",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0000111011......", aea: "A+-DXWL...", cycles: [10, 10,  4] },
		Inst { name: "cas2",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000110011111100", aea: "..........", cycles: [10, 10,  4] },
		Inst { name: "cas2",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000111011111100", aea: "..........", cycles: [10, 10,  4] },
		Inst { name: "chk",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0100...110000...", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100...110......", aea: "A+-DXWLdxI", cycles: [10,  8,  8] },
		Inst { name: "chk",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0100...100000...", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100...100......", aea: "A+-DXWLdxI", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::pcdi, bitp: "0000000011111010", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::pcix, bitp: "0000000011111011", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::n,    bitp: "0000000011......", aea: "A..DXWL...", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::pcdi, bitp: "0000001011111010", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::pcix, bitp: "0000001011111011", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::n,    bitp: "0000001011......", aea: "A..DXWL...", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::pcdi, bitp: "0000010011111010", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::pcix, bitp: "0000010011111011", aea: "..........", cycles: [10,  8,  8] },
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::n,    bitp: "0000010011......", aea: "A..DXWL...", cycles: [10,  8,  8] },
		Inst { name: "clr",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0100001000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "clr",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0100001000......", aea: "A+-DXWL...", cycles: [ 8,  4,  4] },
		Inst { name: "clr",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0100001001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "clr",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100001001......", aea: "A+-DXWL...", cycles: [ 8,  4,  4] },
		Inst { name: "clr",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0100001010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "clr",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100001010......", aea: "A+-DXWL...", cycles: [12,  6,  4] },
		Inst { name: "cmp",      size:  8, sp: SP::n,   se: SE::d,    bitp: "1011...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "cmp",      size:  8, sp: SP::n,   se: SE::n,    bitp: "1011...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::d,    bitp: "1011...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::a,    bitp: "1011...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1011...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::d,    bitp: "1011...010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::a,    bitp: "1011...010001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::n,    bitp: "1011...010......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1011...011000...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::a,    bitp: "1011...011001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1011...011......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  4] },
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::d,    bitp: "1011...111000...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::a,    bitp: "1011...111001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::n,    bitp: "1011...111......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  4] },
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0000110000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0000110000......", aea: "A+-DXWL...", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::pcdi, bitp: "0000110000111010", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::pcix, bitp: "0000110000111011", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0000110001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000110001......", aea: "A+-DXWL...", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::pcdi, bitp: "0000110001111010", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::pcix, bitp: "0000110001111011", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0000110010000...", aea: "..........", cycles: [14, 12,  2] },
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000110010......", aea: "A+-DXWL...", cycles: [12, 12,  2] },
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::pcdi, bitp: "0000110010111010", aea: "..........", cycles: [12, 12,  2] },
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::pcix, bitp: "0000110010111011", aea: "..........", cycles: [12, 12,  2] },
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::ax7,  bitp: "1011111100001...", aea: "..........", cycles: [12, 12,  9] },
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::ay7,  bitp: "1011...100001111", aea: "..........", cycles: [12, 12,  9] },
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::axy7, bitp: "1011111100001111", aea: "..........", cycles: [12, 12,  9] },
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::n,    bitp: "1011...100001...", aea: "..........", cycles: [12, 12,  9] },
		Inst { name: "cmpm",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1011...101001...", aea: "..........", cycles: [12, 12,  9] },
		Inst { name: "cmpm",     size: 32, sp: SP::n,   se: SE::n,    bitp: "1011...110001...", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "cpbcc",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1111...01.......", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "cpdbcc",   size: 32, sp: SP::n,   se: SE::n,    bitp: "1111...001001...", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "cpgen",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1111...000......", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "cpscc",    size: 32, sp: SP::n,   se: SE::n,    bitp: "1111...001......", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "cptrapcc", size:  2, sp: SP::n,   se: SE::n,    bitp: "1111...001111...", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "dbt",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0101000011001...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "dbf",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0101000111001...", aea: "..........", cycles: [14, 14,  6] },
		Inst { name: "dbcc",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0101....11001...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "divs",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1000...111000...", aea: "..........", cycles: [158, 122, 56] },
		Inst { name: "divs",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1000...111......", aea: "A+-DXWLdxI", cycles: [158, 122, 56] },
		Inst { name: "divu",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1000...011000...", aea: "..........", cycles: [140, 108, 44] },
		Inst { name: "divu",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1000...011......", aea: "A+-DXWLdxI", cycles: [140, 108, 44] },
		Inst { name: "divl",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0100110001000...", aea: "..........", cycles: [140, 108, 44] },
		Inst { name: "divl",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100110001......", aea: "A+-DXWLdxI", cycles: [140, 108, 44] },
		Inst { name: "eor",      size:  8, sp: SP::n,   se: SE::d,    bitp: "1011...100000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "eor",      size:  8, sp: SP::n,   se: SE::n,    bitp: "1011...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "eor",      size: 16, sp: SP::n,   se: SE::d,    bitp: "1011...101000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "eor",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1011...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "eor",      size: 32, sp: SP::n,   se: SE::d,    bitp: "1011...110000...", aea: "..........", cycles: [ 8,  6,  2] },
		Inst { name: "eor",      size: 32, sp: SP::n,   se: SE::n,    bitp: "1011...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "eori",     size: 16, sp: SP::toc, se: SE::n,    bitp: "0000101000111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "eori",     size: 16, sp: SP::tos, se: SE::n,    bitp: "0000101001111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "eori",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0000101000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "eori",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0000101000......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "eori",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0000101001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "eori",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000101001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "eori",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0000101010000...", aea: "..........", cycles: [16, 14,  2] },
		Inst { name: "eori",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000101010......", aea: "A+-DXWL...", cycles: [20, 20,  4] },
		Inst { name: "exg",      size: 32, sp: SP::dd,  se: SE::n,    bitp: "1100...101000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "exg",      size: 32, sp: SP::aa,  se: SE::n,    bitp: "1100...101001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "exg",      size: 32, sp: SP::da,  se: SE::n,    bitp: "1100...110001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "ext",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100100010000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "ext",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100100011000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "extb",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100100111000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "illegal",  size:  0, sp: SP::n,   se: SE::n,    bitp: "0100101011111100", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "jmp",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111011......", aea: "A..DXWLdx.", cycles: [ 4,  4,  0] },
		Inst { name: "jsr",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111010......", aea: "A..DXWLdx.", cycles: [12, 12,  0] },
		Inst { name: "lea",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100...111......", aea: "A..DXWLdx.", cycles: [ 0,  0,  2] },
		Inst { name: "link",     size: 16, sp: SP::n,   se: SE::a7,   bitp: "0100111001010111", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "link",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0100111001010...", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "link",     size: 32, sp: SP::n,   se: SE::a7,   bitp: "0100100000001111", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "link",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100100000001...", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "lsr",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...000001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "lsr",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...001001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "lsr",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...010001...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "lsr",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...000101...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "lsr",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...001101...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "lsr",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...010101...", aea: "..........", cycles: [ 8,  8,  6] },
		Inst { name: "lsr",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110001011......", aea: "A+-DXWL...", cycles: [ 8,  8,  5] },
		Inst { name: "lsl",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...100001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "lsl",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...101001...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "lsl",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...110001...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "lsl",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...100101...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "lsl",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...101101...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "lsl",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...110101...", aea: "..........", cycles: [ 8,  8,  6] },
		Inst { name: "lsl",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110001111......", aea: "A+-DXWL...", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size:  8, sp: SP::d,   se: SE::d,    bitp: "0001...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size:  8, sp: SP::d,   se: SE::n,    bitp: "0001...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size:  8, sp: SP::ai,  se: SE::d,    bitp: "0001...010000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::ai,  se: SE::n,    bitp: "0001...010......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::pi,  se: SE::d,    bitp: "0001...011000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::pi,  se: SE::n,    bitp: "0001...011......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::pi7, se: SE::d,    bitp: "0001111011000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::pi7, se: SE::n,    bitp: "0001111011......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size:  8, sp: SP::pd,  se: SE::d,    bitp: "0001...100000...", aea: "..........", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size:  8, sp: SP::pd,  se: SE::n,    bitp: "0001...100......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size:  8, sp: SP::pd7, se: SE::d,    bitp: "0001111100000...", aea: "..........", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size:  8, sp: SP::pd7, se: SE::n,    bitp: "0001111100......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size:  8, sp: SP::di,  se: SE::d,    bitp: "0001...101000...", aea: "..........", cycles: [12, 12,  5] },
		Inst { name: "move",     size:  8, sp: SP::di,  se: SE::n,    bitp: "0001...101......", aea: "A+-DXWLdxI", cycles: [12, 12,  5] },
		Inst { name: "move",     size:  8, sp: SP::ix,  se: SE::d,    bitp: "0001...110000...", aea: "..........", cycles: [14, 14,  7] },
		Inst { name: "move",     size:  8, sp: SP::ix,  se: SE::n,    bitp: "0001...110......", aea: "A+-DXWLdxI", cycles: [14, 14,  7] },
		Inst { name: "move",     size:  8, sp: SP::aw,  se: SE::d,    bitp: "0001000111000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size:  8, sp: SP::aw,  se: SE::n,    bitp: "0001000111......", aea: "A+-DXWLdxI", cycles: [12, 12,  4] },
		Inst { name: "move",     size:  8, sp: SP::al,  se: SE::d,    bitp: "0001001111000...", aea: "..........", cycles: [16, 16,  6] },
		Inst { name: "move",     size:  8, sp: SP::al,  se: SE::n,    bitp: "0001001111......", aea: "A+-DXWLdxI", cycles: [16, 16,  6] },
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::d,    bitp: "0011...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::a,    bitp: "0011...000001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::n,    bitp: "0011...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::d,    bitp: "0011...010000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::a,    bitp: "0011...010001...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::n,    bitp: "0011...010......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::d,    bitp: "0011...011000...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::a,    bitp: "0011...011001...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::n,    bitp: "0011...011......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  4] },
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::d,    bitp: "0011...100000...", aea: "..........", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::a,    bitp: "0011...100001...", aea: "..........", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::n,    bitp: "0011...100......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  5] },
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::d,    bitp: "0011...101000...", aea: "..........", cycles: [12, 12,  5] },
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::a,    bitp: "0011...101001...", aea: "..........", cycles: [12, 12,  5] },
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::n,    bitp: "0011...101......", aea: "A+-DXWLdxI", cycles: [12, 12,  5] },
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::d,    bitp: "0011...110000...", aea: "..........", cycles: [14, 14,  7] },
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::a,    bitp: "0011...110001...", aea: "..........", cycles: [14, 14,  7] },
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::n,    bitp: "0011...110......", aea: "A+-DXWLdxI", cycles: [14, 14,  7] },
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::d,    bitp: "0011000111000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::a,    bitp: "0011000111001...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::n,    bitp: "0011000111......", aea: "A+-DXWLdxI", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::d,    bitp: "0011001111000...", aea: "..........", cycles: [16, 16,  6] },
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::a,    bitp: "0011001111001...", aea: "..........", cycles: [16, 16,  6] },
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::n,    bitp: "0011001111......", aea: "A+-DXWLdxI", cycles: [16, 16,  6] },
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::d,    bitp: "0010...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::a,    bitp: "0010...000001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::n,    bitp: "0010...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::d,    bitp: "0010...010000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::a,    bitp: "0010...010001...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::n,    bitp: "0010...010......", aea: "A+-DXWLdxI", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::d,    bitp: "0010...011000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::a,    bitp: "0010...011001...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::n,    bitp: "0010...011......", aea: "A+-DXWLdxI", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::d,    bitp: "0010...100000...", aea: "..........", cycles: [12, 14,  5] },
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::a,    bitp: "0010...100001...", aea: "..........", cycles: [12, 14,  5] },
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::n,    bitp: "0010...100......", aea: "A+-DXWLdxI", cycles: [12, 14,  5] },
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::d,    bitp: "0010...101000...", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::a,    bitp: "0010...101001...", aea: "..........", cycles: [16, 16,  5] },
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::n,    bitp: "0010...101......", aea: "A+-DXWLdxI", cycles: [16, 16,  5] },
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::d,    bitp: "0010...110000...", aea: "..........", cycles: [18, 18,  7] },
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::a,    bitp: "0010...110001...", aea: "..........", cycles: [18, 18,  7] },
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::n,    bitp: "0010...110......", aea: "A+-DXWLdxI", cycles: [18, 18,  7] },
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::d,    bitp: "0010000111000...", aea: "..........", cycles: [16, 16,  4] },
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::a,    bitp: "0010000111001...", aea: "..........", cycles: [16, 16,  4] },
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::n,    bitp: "0010000111......", aea: "A+-DXWLdxI", cycles: [16, 16,  4] },
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::d,    bitp: "0010001111000...", aea: "..........", cycles: [20, 20,  6] },
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::a,    bitp: "0010001111001...", aea: "..........", cycles: [20, 20,  6] },
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::n,    bitp: "0010001111......", aea: "A+-DXWLdxI", cycles: [20, 20,  6] },
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::d,    bitp: "0011...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::a,    bitp: "0011...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::n,    bitp: "0011...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::d,    bitp: "0010...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::a,    bitp: "0010...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0010...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::frc, se: SE::d,    bitp: "0100001011000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::frc, se: SE::n,    bitp: "0100001011......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "move",     size: 16, sp: SP::toc, se: SE::d,    bitp: "0100010011000...", aea: "..........", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 16, sp: SP::toc, se: SE::n,    bitp: "0100010011......", aea: "A+-DXWLdxI", cycles: [12, 12,  4] },
		Inst { name: "move",     size: 16, sp: SP::frs, se: SE::d,    bitp: "0100000011000...", aea: "..........", cycles: [ 6,  4,  8] },
		Inst { name: "move",     size: 16, sp: SP::frs, se: SE::n,    bitp: "0100000011......", aea: "A+-DXWL...", cycles: [ 8,  8,  8] },
		Inst { name: "move",     size: 16, sp: SP::tos, se: SE::d,    bitp: "0100011011000...", aea: "..........", cycles: [12, 12,  8] },
		Inst { name: "move",     size: 16, sp: SP::tos, se: SE::n,    bitp: "0100011011......", aea: "A+-DXWLdxI", cycles: [12, 12,  8] },
		Inst { name: "move",     size: 32, sp: SP::fru, se: SE::n,    bitp: "0100111001101...", aea: "..........", cycles: [ 4,  6,  2] },
		Inst { name: "move",     size: 32, sp: SP::tou, se: SE::n,    bitp: "0100111001100...", aea: "..........", cycles: [ 4,  6,  2] },
		Inst { name: "movec",    size: 32, sp: SP::cr,  se: SE::n,    bitp: "0100111001111010", aea: "..........", cycles: [ 4,  6,  2] },
		Inst { name: "movec",    size: 32, sp: SP::rc,  se: SE::n,    bitp: "0100111001111011", aea: "..........", cycles: [ 4,  6,  2] },
		Inst { name: "movem",    size: 16, sp: SP::re,  se: SE::pd,   bitp: "0100100010100...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "movem",    size: 16, sp: SP::re,  se: SE::n,    bitp: "0100100010......", aea: "A..DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "movem",    size: 32, sp: SP::re,  se: SE::pd,   bitp: "0100100011100...", aea: "..........", cycles: [ 8,  8,  4] },
		Inst { name: "movem",    size: 32, sp: SP::re,  se: SE::n,    bitp: "0100100011......", aea: "A..DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pi,   bitp: "0100110010011...", aea: "..........", cycles: [12, 12,  8] },
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pcdi, bitp: "0100110010111010", aea: "..........", cycles: [16, 16,  9] },
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pcix, bitp: "0100110010111011", aea: "..........", cycles: [18, 18, 11] },
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::n,    bitp: "0100110010......", aea: "A..DXWL...", cycles: [12, 12,  8] },
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pi,   bitp: "0100110011011...", aea: "..........", cycles: [12, 12,  8] },
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pcdi, bitp: "0100110011111010", aea: "..........", cycles: [20, 20,  9] },
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pcix, bitp: "0100110011111011", aea: "..........", cycles: [22, 22, 11] },
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::n,    bitp: "0100110011......", aea: "A..DXWL...", cycles: [12, 12,  8] },
		Inst { name: "movep",    size: 16, sp: SP::er,  se: SE::n,    bitp: "0000...100001...", aea: "..........", cycles: [16, 16, 12] },
		Inst { name: "movep",    size: 32, sp: SP::er,  se: SE::n,    bitp: "0000...101001...", aea: "..........", cycles: [24, 24, 18] },
		Inst { name: "movep",    size: 16, sp: SP::re,  se: SE::n,    bitp: "0000...110001...", aea: "..........", cycles: [16, 16, 11] },
		Inst { name: "movep",    size: 32, sp: SP::re,  se: SE::n,    bitp: "0000...111001...", aea: "..........", cycles: [24, 24, 17] },
		Inst { name: "moveq",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0111...0........", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "moves",    size:  8, sp: SP::n,   se: SE::n,    bitp: "0000111000......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "moves",    size: 16, sp: SP::n,   se: SE::n,    bitp: "0000111001......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "moves",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0000111010......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "muls",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1100...111000...", aea: "..........", cycles: [54, 32, 27] },
		Inst { name: "muls",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1100...111......", aea: "A+-DXWLdxI", cycles: [54, 32, 27] },
		Inst { name: "mulu",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1100...011000...", aea: "..........", cycles: [54, 30, 27] },
		Inst { name: "mulu",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1100...011......", aea: "A+-DXWLdxI", cycles: [54, 30, 27] },
		Inst { name: "mull",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0100110000000...", aea: "..........", cycles: [54, 30, 27] },
		Inst { name: "mull",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100110000......", aea: "A+-DXWLdxI", cycles: [54, 30, 27] },
		Inst { name: "nbcd",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0100100000000...", aea: "..........", cycles: [ 6,  6,  6] },
		Inst { name: "nbcd",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0100100000......", aea: "A+-DXWL...", cycles: [ 8,  8,  6] },
		Inst { name: "neg",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0100010000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "neg",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0100010000......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "neg",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0100010001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "neg",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100010001......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "neg",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0100010010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "neg",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100010010......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "negx",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0100000000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "negx",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0100000000......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "negx",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0100000001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "negx",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0100000001......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "negx",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0100000010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "negx",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100000010......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "nop",      size:  0, sp: SP::n,   se: SE::n,    bitp: "0100111001110001", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "not",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0100011000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "not",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0100011000......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "not",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0100011001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "not",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100011001......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "not",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0100011010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "not",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100011010......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "or",       size:  8, sp: SP::er,  se: SE::d,    bitp: "1000...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "or",       size:  8, sp: SP::er,  se: SE::n,    bitp: "1000...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "or",       size: 16, sp: SP::er,  se: SE::d,    bitp: "1000...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "or",       size: 16, sp: SP::er,  se: SE::n,    bitp: "1000...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "or",       size: 32, sp: SP::er,  se: SE::d,    bitp: "1000...010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "or",       size: 32, sp: SP::er,  se: SE::n,    bitp: "1000...010......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "or",       size:  8, sp: SP::re,  se: SE::n,    bitp: "1000...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "or",       size: 16, sp: SP::re,  se: SE::n,    bitp: "1000...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "or",       size: 32, sp: SP::re,  se: SE::n,    bitp: "1000...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "ori",      size: 16, sp: SP::toc, se: SE::n,    bitp: "0000000000111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "ori",      size: 16, sp: SP::tos, se: SE::n,    bitp: "0000000001111100", aea: "..........", cycles: [20, 16, 12] },
		Inst { name: "ori",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0000000000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "ori",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0000000000......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "ori",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0000000001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "ori",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0000000001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "ori",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0000000010000...", aea: "..........", cycles: [16, 14,  2] },
		Inst { name: "ori",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0000000010......", aea: "A+-DXWL...", cycles: [20, 20,  4] },
		Inst { name: "pack",     size: 16, sp: SP::rr,  se: SE::n,    bitp: "1000...101000...", aea: "..........", cycles: [20, 20,  4] },
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::ax7,  bitp: "1000111101001...", aea: "..........", cycles: [20, 20,  4] },
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::ay7,  bitp: "1000...101001111", aea: "..........", cycles: [20, 20,  4] },
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::axy7, bitp: "1000111101001111", aea: "..........", cycles: [20, 20,  4] },
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::n,    bitp: "1000...101001...", aea: "..........", cycles: [20, 20,  4] },
		Inst { name: "pea",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100100001......", aea: "A..DXWLdx.", cycles: [ 6,  6,  5] },
		Inst { name: "reset",    size:  0, sp: SP::n,   se: SE::n,    bitp: "0100111001110000", aea: "..........", cycles: [ 0,  0,  0] },
		Inst { name: "ror",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...000011...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "ror",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...001011...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "ror",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...010011...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "ror",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...000111...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "ror",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...001111...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "ror",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...010111...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "ror",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110011011......", aea: "A+-DXWL...", cycles: [ 8,  8,  7] },
		Inst { name: "rol",      size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...100011...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "rol",      size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...101011...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "rol",      size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...110011...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "rol",      size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...100111...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "rol",      size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...101111...", aea: "..........", cycles: [ 6,  6,  8] },
		Inst { name: "rol",      size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...110111...", aea: "..........", cycles: [ 8,  8,  8] },
		Inst { name: "rol",      size: 16, sp: SP::n,   se: SE::n,    bitp: "1110011111......", aea: "A+-DXWL...", cycles: [ 8,  8,  7] },
		Inst { name: "roxr",     size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...000010...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxr",     size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...001010...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxr",     size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...010010...", aea: "..........", cycles: [ 8,  8, 12] },
		Inst { name: "roxr",     size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...000110...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxr",     size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...001110...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxr",     size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...010110...", aea: "..........", cycles: [ 8,  8, 12] },
		Inst { name: "roxr",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1110010011......", aea: "A+-DXWL...", cycles: [ 8,  8,  5] },
		Inst { name: "roxl",     size:  8, sp: SP::s,   se: SE::n,    bitp: "1110...100010...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxl",     size: 16, sp: SP::s,   se: SE::n,    bitp: "1110...101010...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxl",     size: 32, sp: SP::s,   se: SE::n,    bitp: "1110...110010...", aea: "..........", cycles: [ 8,  8, 12] },
		Inst { name: "roxl",     size:  8, sp: SP::r,   se: SE::n,    bitp: "1110...100110...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxl",     size: 16, sp: SP::r,   se: SE::n,    bitp: "1110...101110...", aea: "..........", cycles: [ 6,  6, 12] },
		Inst { name: "roxl",     size: 32, sp: SP::r,   se: SE::n,    bitp: "1110...110110...", aea: "..........", cycles: [ 8,  8, 12] },
		Inst { name: "roxl",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1110010111......", aea: "A+-DXWL...", cycles: [ 8,  8,  5] },
		Inst { name: "rtd",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111001110100", aea: "..........", cycles: [ 8,  8,  5] },
		Inst { name: "rte",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111001110011", aea: "..........", cycles: [20, 24, 20] },
		Inst { name: "rtm",      size: 32, sp: SP::n,   se: SE::n,    bitp: "000001101100....", aea: "..........", cycles: [20, 24, 20] },
		Inst { name: "rtr",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111001110111", aea: "..........", cycles: [20, 20, 14] },
		Inst { name: "rts",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111001110101", aea: "..........", cycles: [16, 16, 10] },
		Inst { name: "sbcd",     size:  8, sp: SP::rr,  se: SE::n,    bitp: "1000...100000...", aea: "..........", cycles: [ 6,  6,  4] },
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::ax7,  bitp: "1000111100001...", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::ay7,  bitp: "1000...100001111", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::axy7, bitp: "1000111100001111", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::n,    bitp: "1000...100001...", aea: "..........", cycles: [18, 18, 16] },
		Inst { name: "st",       size:  8, sp: SP::n,   se: SE::d,    bitp: "0101000011000...", aea: "..........", cycles: [ 6,  4,  4] },
		Inst { name: "st",       size:  8, sp: SP::n,   se: SE::n,    bitp: "0101000011......", aea: "A+-DXWL...", cycles: [ 8,  8,  6] },
		Inst { name: "sf",       size:  8, sp: SP::n,   se: SE::d,    bitp: "0101000111000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "sf",       size:  8, sp: SP::n,   se: SE::n,    bitp: "0101000111......", aea: "A+-DXWL...", cycles: [ 8,  8,  6] },
		Inst { name: "scc",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0101....11000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "scc",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0101....11......", aea: "A+-DXWL...", cycles: [ 8,  8,  6] },
		Inst { name: "stop",     size:  0, sp: SP::n,   se: SE::n,    bitp: "0100111001110010", aea: "..........", cycles: [ 4,  4,  8] },
		Inst { name: "sub",      size:  8, sp: SP::er,  se: SE::d,    bitp: "1001...000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "sub",      size:  8, sp: SP::er,  se: SE::n,    bitp: "1001...000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::d,    bitp: "1001...001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::a,    bitp: "1001...001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::n,    bitp: "1001...001......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::d,    bitp: "1001...010000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::a,    bitp: "1001...010001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::n,    bitp: "1001...010......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "sub",      size:  8, sp: SP::re,  se: SE::n,    bitp: "1001...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "sub",      size: 16, sp: SP::re,  se: SE::n,    bitp: "1001...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "sub",      size: 32, sp: SP::re,  se: SE::n,    bitp: "1001...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::d,    bitp: "1001...011000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::a,    bitp: "1001...011001...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1001...011......", aea: "A+-DXWLdxI", cycles: [ 8,  8,  2] },
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::d,    bitp: "1001...111000...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::a,    bitp: "1001...111001...", aea: "..........", cycles: [ 6,  6,  2] },
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::n,    bitp: "1001...111......", aea: "A+-DXWLdxI", cycles: [ 6,  6,  2] },
		Inst { name: "subi",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0000010000000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "subi",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0000010000......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "subi",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0000010001000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "subi",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0000010001......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "subi",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0000010010000...", aea: "..........", cycles: [16, 14,  2] },
		Inst { name: "subi",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0000010010......", aea: "A+-DXWL...", cycles: [20, 20,  4] },
		Inst { name: "subq",     size:  8, sp: SP::n,   se: SE::d,    bitp: "0101...100000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "subq",     size:  8, sp: SP::n,   se: SE::n,    bitp: "0101...100......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::d,    bitp: "0101...101000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::a,    bitp: "0101...101001...", aea: "..........", cycles: [ 8,  4,  2] },
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::n,    bitp: "0101...101......", aea: "A+-DXWL...", cycles: [ 8,  8,  4] },
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::d,    bitp: "0101...110000...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::a,    bitp: "0101...110001...", aea: "..........", cycles: [ 8,  8,  2] },
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0101...110......", aea: "A+-DXWL...", cycles: [12, 12,  4] },
		Inst { name: "subx",     size:  8, sp: SP::rr,  se: SE::n,    bitp: "1001...100000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "subx",     size: 16, sp: SP::rr,  se: SE::n,    bitp: "1001...101000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "subx",     size: 32, sp: SP::rr,  se: SE::n,    bitp: "1001...110000...", aea: "..........", cycles: [ 8,  6,  2] },
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::ax7,  bitp: "1001111100001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::ay7,  bitp: "1001...100001111", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::axy7, bitp: "1001111100001111", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::n,    bitp: "1001...100001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "subx",     size: 16, sp: SP::mm,  se: SE::n,    bitp: "1001...101001...", aea: "..........", cycles: [18, 18, 12] },
		Inst { name: "subx",     size: 32, sp: SP::mm,  se: SE::n,    bitp: "1001...110001...", aea: "..........", cycles: [30, 30, 12] },
		Inst { name: "swap",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100100001000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "tas",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0100101011000...", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "tas",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0100101011......", aea: "A+-DXWL...", cycles: [14, 14, 12] },
		Inst { name: "trap",     size:  0, sp: SP::n,   se: SE::n,    bitp: "010011100100....", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapt",    size:  0, sp: SP::n,   se: SE::n,    bitp: "0101000011111100", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapt",    size: 16, sp: SP::n,   se: SE::n,    bitp: "0101000011111010", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapt",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0101000011111011", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapf",    size:  0, sp: SP::n,   se: SE::n,    bitp: "0101000111111100", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapf",    size: 16, sp: SP::n,   se: SE::n,    bitp: "0101000111111010", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapf",    size: 32, sp: SP::n,   se: SE::n,    bitp: "0101000111111011", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapcc",   size:  0, sp: SP::n,   se: SE::n,    bitp: "0101....11111100", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapcc",   size: 16, sp: SP::n,   se: SE::n,    bitp: "0101....11111010", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapcc",   size: 32, sp: SP::n,   se: SE::n,    bitp: "0101....11111011", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "trapv",    size:  0, sp: SP::n,   se: SE::n,    bitp: "0100111001110110", aea: "..........", cycles: [ 4,  4,  4] },
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::d,    bitp: "0100101000000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::n,    bitp: "0100101000......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::pcdi, bitp: "0100101000111010", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::pcix, bitp: "0100101000111011", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::i,    bitp: "0100101000111100", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::d,    bitp: "0100101001000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::a,    bitp: "0100101001001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::n,    bitp: "0100101001......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::pcdi, bitp: "0100101001111010", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::pcix, bitp: "0100101001111011", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::i,    bitp: "0100101001111100", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::d,    bitp: "0100101010000...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::a,    bitp: "0100101010001...", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::n,    bitp: "0100101010......", aea: "A+-DXWL...", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::pcdi, bitp: "0100101010111010", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::pcix, bitp: "0100101010111011", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::i,    bitp: "0100101010111100", aea: "..........", cycles: [ 4,  4,  2] },
		Inst { name: "unlk",     size: 32, sp: SP::n,   se: SE::a7,   bitp: "0100111001011111", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unlk",     size: 32, sp: SP::n,   se: SE::n,    bitp: "0100111001011...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::rr,  se: SE::n,    bitp: "1000...110000...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::ax7,  bitp: "1000111110001...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::ay7,  bitp: "1000...110001111", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::axy7, bitp: "1000111110001111", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::n,    bitp: "1000...110001...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1000...110001...", aea: "..........", cycles: [12, 12,  6] },
		Inst { name: "unpk",     size: 16, sp: SP::n,   se: SE::n,    bitp: "1000...110001...", aea: "..........", cycles: [12, 12,  6] },
		*/
    ];

    let arg0 = "".to_string();

    for i in 0..test.len() { 
        let ref t = test[i];

        /*
        match t.sp {
            SP::er => arg0 = generate_ea(t.eae),
            _ => (),
        }
        */


        match t.size {
            8 => println!("Name {}.b", t.name),
            16 => println!("Name {}.w", t.name),
            32 => println!("Name {}.l", t.name),
            _ => (),
        } 
    }
}
