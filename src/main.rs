use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;
    let g = BLS12381Curve::generator();
    let public_key = g.operate_with_self(secret_key);
    
    println!("Secret key: {:?}", secret_key);
    println!("Public key: {:?}", public_key);
    let public_key_affine = public_key.to_affine();

    // Extract the x and y coordinates
    let x = public_key_affine.x();
    let y = public_key_affine.y();

    // Print the coordinates
    println!("Public key (x): {}", x);
    println!("Public key (y): {}", y);
}
