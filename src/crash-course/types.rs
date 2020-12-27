/*bool : The boolean type.
char : A character type.
i8 : The 8-bit signed integer type.
i16 : The 16-bit signed integer type.
i32 : The 32-bit signed integer type.
i64 : The 64-bit signed integer type.
isize : The pointer-sized signed integer type.
u8 : The 8-bit unsigned integer type.
u16 : The 16-bit unsigned integer type.
u32 : The 32-bit unsigned integer type.
u64 : The 64-bit unsigned integer type.
usize : The pointer-sized unsigned integer type.
f32 : The 32-bit floating point type.
f64 : The 64-bit floating point type.
array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
slice : A dynamically-sized view into a contiguous sequence, [T].
str : String slices.
tuple : A finite heterogeneous sequence, (T, U, ..).
*/

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 65416516161;


    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F602}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
