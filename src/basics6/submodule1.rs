pub fn test_fn1(){
    println!("Hello world1");
}
fn test_fn2() {
    println!("Hello rust1");
}


///構造体にpubをつけると構造体はpublicになるもののフィールドはpribateのままです
///つまり，インスタンス化はできない
///newを定義することにより，公式な手順でインスタンス化が可能
pub struct teststruct {
    val1: i32,
    val2: i32,
}

impl teststruct {
    pub fn new(val1: i32, val2: i32) -> teststruct {
        teststruct {val1, val2}
    }
}