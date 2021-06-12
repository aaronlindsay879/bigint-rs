use bigint::bigint::BigInt;

fn main() {
    let one = BigInt::from_value(10);
    let two = BigInt::from_value(-4);

    println!("{:?} {:?}", one, two);
    let sum = one + two;

    println!("{:?}", sum);
    println!("{:?}", sum.to_value::<i32>());
}
