use std::mem;

fn main() {
    let a:u8 = 123; //8bits
    println!("a = {}", a);

    // mut
    let mut b:i8 = 0; //mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c =123456789; //32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    let z:isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, take up {} bytes, {}-bit os", z, size_of_z,size_of_z*8);

    let d = 'x'; //char
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; //double-precision
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4>0; //true
}
