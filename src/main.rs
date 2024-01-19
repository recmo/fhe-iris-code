mod reference;

use itertools::{izip, Itertools};
use ruint::aliases::U256 as RU256;
use tfhe::{
    generate_keys,
    integer::U256,
    prelude::{FheEncrypt, *},
    set_server_key,
    shortint::{gen_keys, parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS},
    ConfigBuilder, FheUint16, FheUint256, FheUint8,
};

const BITS: usize = 16 * 2 * 2 * 200;
const N: usize = BITS / 256;

fn rand_u256() -> U256 {
    let r: RU256 = rand::random();
    U256::from(*r.as_limbs())
}

fn rand_bits() -> Box<[U256]> {
    (0..N).map(|_| rand_u256()).collect::<Box<[_]>>()
}

fn half_adder(a: &FheUint256, b: &FheUint256) -> (FheUint256, FheUint256) {
    let sum = a ^ b;
    let carry = a & b;
    (sum, carry)
}

fn full_adder(a: &FheUint256, b: &FheUint256, carry: &FheUint256) -> (FheUint256, FheUint256) {
    let (sum, carry_1) = half_adder(a, b);
    let (sum, carry_2) = half_adder(&sum, carry);
    let carry = carry_1 | carry_2;
    (sum, carry)
}

fn popcount(a: &FheUint256) -> FheUint16 {
    dbg!();
    let mask = U256::from((
        0x55555555555555555555555555555555_u128,
        0x55555555555555555555555555555555_u128,
    ));
    let odd = a & mask;
    let even = (a >> 1_u8) & mask;
    let sum = &odd + &even;

    dbg!();
    let mask = U256::from((
        0x33333333333333333333333333333333_u128,
        0x33333333333333333333333333333333_u128,
    ));
    let odd = &sum & mask;
    let even = (sum >> 2_u8) & mask;
    let sum = &odd + &even;

    // TODO: Remaining steps!

    sum.cast_into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    eprintln!("Generating keys.");
    let (client_key, server_keys) = generate_keys(config);

    eprintln!("Generating input.");
    let clear_a_bits = rand_bits();
    let clear_a_mask = rand_bits();
    let clear_b_bits = rand_bits();
    let clear_b_mask = rand_bits();

    // Encrypting the input data using the (private) client_key
    eprintln!("Encrypting input.");
    let encrypted_a_bits = clear_a_bits
        .iter()
        .map(|n| FheUint256::encrypt(*n, &client_key))
        .collect::<Box<[_]>>();
    let encrypted_b_bits = clear_b_bits
        .iter()
        .map(|n| FheUint256::encrypt(*n, &client_key))
        .collect::<Box<[_]>>();

    // On the server side:
    eprintln!("Setting server key.");
    set_server_key(server_keys);

    eprintln!("Computing mask.");
    let mask = izip!(clear_a_mask.iter(), clear_b_mask.iter())
        .map(|(a, b)| *a | *b)
        .collect::<Box<[_]>>();

    eprintln!("FHE computation.");
    let start = std::time::Instant::now();

    // Compute template intersection
    eprintln!("    - Template intersection.");
    let start1 = std::time::Instant::now();
    let bits = izip!(
        encrypted_a_bits.iter(),
        encrypted_b_bits.iter(),
        mask.iter()
    )
    .map(|(a, b, m)| (a ^ b) | *m)
    .collect::<Box<[FheUint256]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    // Compute first round of popcount (pairs)
    eprintln!("    - Count {} bits as 2-bit uints.", bits.len());
    let start1 = std::time::Instant::now();
    let counts = bits
        .into_iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| {
            // Sum two 1-bit numbers into one 2-bit number
            let (ones, carry) = half_adder(a, b);
            [ones, carry]
        })
        .collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    eprintln!("    - Sum {} counts to 3-bit uints.", counts.len());
    let start1 = std::time::Instant::now();
    let counts = counts
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| {
            // Sum two 2-bit numbers into one 3-bit number
            let (ones, carry) = half_adder(&a[0], &b[0]);
            let (twos, carry) = full_adder(&a[1], &b[1], &carry);
            [ones, twos, carry]
        })
        .collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    eprintln!("    - Sum {} counts to 4-bit uints.", counts.len());
    let start1 = std::time::Instant::now();
    let counts = counts
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| {
            // Sum two 3-bit numbers into one 4-bit number
            let (ones, carry) = half_adder(&a[0], &b[0]);
            let (twos, carry) = full_adder(&a[1], &b[1], &carry);
            let (fours, carry) = full_adder(&a[2], &b[2], &carry);
            [ones, twos, fours, carry]
        })
        .collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    eprintln!("    - Sum {} counts to 5-bit uints.", counts.len());
    let start1 = std::time::Instant::now();
    let counts = counts
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| {
            // Sum two 4-bit numbers into one 5-bit number
            let (ones, carry) = half_adder(&a[0], &b[0]);
            let (twos, carry) = full_adder(&a[1], &b[1], &carry);
            let (fours, carry) = full_adder(&a[2], &b[2], &carry);
            let (eights, carry) = full_adder(&a[3], &b[3], &carry);
            [ones, twos, fours, eights, carry]
        })
        .collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    eprintln!("    - Sum {} counts to 6-bit uints.", counts.len());
    let start1 = std::time::Instant::now();
    let counts = counts
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| {
            // Sum two 4-bit numbers into one 5-bit number
            let (ones, carry) = half_adder(&a[0], &b[0]);
            let (twos, carry) = full_adder(&a[1], &b[1], &carry);
            let (fours, carry) = full_adder(&a[2], &b[2], &carry);
            let (eights, carry) = full_adder(&a[3], &b[3], &carry);
            let (sixteens, carry) = full_adder(&a[4], &b[4], &carry);
            [ones, twos, fours, eights, sixteens, carry]
        })
        .collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());
    assert_eq!(counts.len(), 1);
    let counts = counts[0].clone();

    eprintln!("    - Popcount {} u256s.", counts.len());
    let start1 = std::time::Instant::now();
    let counts = counts.iter().map(popcount).collect::<Box<[_]>>();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    eprintln!("    - Sum {} counts.", counts.len());
    let start1 = std::time::Instant::now();
    let count: FheUint16 = counts.iter().enumerate().map(|(i, n)| n << (i as u8)).sum();
    eprintln!("      Time elapsed {:?}", start1.elapsed());

    let duration = start.elapsed();
    eprintln!("Time elapsed in FHE computation is: {:?}", duration);

    // Decrypting on the client side:
    eprintln!("Decrypting result.");
    let clear_res: u8 = count.decrypt(&client_key);
    eprint!("Result = {clear_res}");

    Ok(())
}
