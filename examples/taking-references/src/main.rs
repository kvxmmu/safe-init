use safe_uninit::*;

fn main() {
    let mut x: safe_uninit![_] = SafeUninit::new(-10i32);
    println!("10 + x = {}", x.take_ref() + 10);  // 0
    println!("x = {}", x);  // -10

    *x = 100;
    println!("x = {}", x);  // 100

    println!("{:?}", x);  // SafeUninit { inner: Some(100) }

    let string: safe_uninit![_] = SafeUninit::new(
        "Hello, world!".to_owned());
    println!("{}", string);  // Hello, world!

    let mut integers: safe_uninit![Vec<i32>] = Default::default();
    integers.push(1024);

    println!("{:?}", integers);  // SafeUninit { inner: Some([1024]) }

    let out_vector = integers.take();  // Also you can use integers.into()
    println!("Moved out vector - {:?}", out_vector);
}
