pub fn scalar() {
    // INTEGERS
    let _i: u32 = 23; // Unsigned Integer // u32 default
    let _i: i32 = -23; // Signed Integer // i32 default
    let _dec: i32 = 234_444; // Decimal Integer
    let _bin: i32 = 0b100; // Binary Integer
    let _hex: i32 = 0xfff; // HexaDecimal Integer
    let _oct: i32 = 0o7; // Octal Integer
    let _byte: u8 = b'0'; // Byte

    // FLOATING POINTS
    let _f: f64 = 2.23; // f64 by default
    let _f2: f64 = 23.23;

    // NUMERIC OPERATIONS
    let _sum: f64 = _f + _f2;
    let _diff: f64 = _f - _f2;
    let _mul: f64 = _f * _f2;
    let _div: f64 = _f / _f2;
    let _modulus: f64 = _f % _f2;

    // println!("Sum: {}", _sum);
    // println!("Sub: {}", _diff);
    // println!("Mul: {}", _mul);
    // println!("Div: {}", _div);
    // println!("Mod: {}", _modulus);

    // BOOLEAN TYPE
    let _b = true;
    let _b2 = false;

    // CHARACTER TYPE
    let _c: char = '7';
}
