// mod basics;
// mod basics2;
// mod basics3;
//mod basics4;
//mod basics5;
mod basics6;
//mod basics6::submodules1;



fn main() {
    //basics::run();
    //basics2::run2();
    //basics2::fizzbuzz3(15);
    //basics3::run3();
    //basics4::run4();
    //basics5::run5();
    // 絶対パスで参照するにはcrate::から開始する
    //main.rsで実行する場合はcrate::とself::は同じ
    //crate::test_module::sub_module1::test_fn1();
    //self::test_module::sub_module1::test_fn1();
    //test_module::sub_module1::test_fn1(); //self::は省略可
    // test_module::sub_module2::test_fn1(); この辺はすべて実行不可

    // main()の中で，モジュールを定義する場合は
    // 最初にuse test_module::sub_module1;と宣言しておいて
    // あとは下記でOK
    //sub_module1::test_fn1();

    //basics6::submodule1::test_fn1();
    use basics6::submodule1;

    println!("hello");

    let test_structure = submodule1::teststruct::new(1, 2);
    rust_tutorial::say_hello();






}