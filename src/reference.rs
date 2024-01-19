use ruint::Uint;

const ROWS: usize = 16;
const COLUMNS: usize = 200;
const LAYERS: usize = 2;

const ROTATIONS: usize = 15;

const IRIS_CODE_BITS: usize = ROWS * COLUMNS * LAYERS;
const IRIS_CODE_LIMBS: usize = IRIS_CODE_BITS / 64;

type Bits = Uint<IRIS_CODE_BITS, IRIS_CODE_LIMBS>;

// Iris codes are 16 × 200 binary images with masks. Two different quantizations
// (sign of real and imaginary). Two different convolution kernels, Four total.

// (16, 200, 2): row, column, quantization.
// (16, 200, 2): row, column, quantization.

struct MaskedBits {
    bits: Bits,
    mask: Bits,
}

struct IrisTemplate {
    codes: [MaskedBits; 4],
}

// TODO: Rotations.
fn fractional_hamming_distance(a: &MaskedBits, b: &MaskedBits) -> f64 {
    let masked = a.mask | b.mask;
    let matched = !(a.bits ^ b.bits);
    unimplemented!()
}
