use water_levels::rain;

fn main() {
    println!("{:?}", rain(&[3, 1, 6, 4, 5, 6], 1));
    println!("{:?}", rain(&[3, 1, 6, 4, 8, 9], 1));
    println!("{:?}", rain(&[1, 9, 1], 1));
    println!("{:?}", rain(&[1, 1, 9], 1));
    println!("{:?}", rain(&[8, 8, 1], 1));
    println!("{:?}", rain(&[8, 8, 8, 1], 1));
    println!("{:?}", rain(&[8, 1, 8, 8, 1], 1));
    println!("{:?}", rain(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 1));


}
