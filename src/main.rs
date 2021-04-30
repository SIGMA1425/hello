fn main() {
    println!("Hello, world!");
    //{}を使うとプレースホルダとして扱われる
    println!("hello {}", "world");
    //{}で値が表示できるのはstd::fmt::Displayトレイトを実装している型
    //{:?}を使ってデバッグ用の値を表示できる場合もある
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
}
