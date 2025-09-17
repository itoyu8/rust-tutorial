use std::fmt::{Debug,Display};
// mod basics;
// mod basics2;
// mod basics3;
//mod basics4;
//mod basics5;
mod basics6;
//mod basics6::submodules1;
use rust_tutorial::sample_trait::{Shape, Rectangle, Circle, double_area};


//引数の型を固定しないように関数を作ることで，関数の重複を防げる．例えば以下の関数は，aやbに整数以外が入るともう機能しない
// fn max(a: i32, b: i32) -> i32 {
//     if a >= b {
//         a
//     } else {
//         b
//     }
// }

//ジェネリック型を用いた関数について
fn max<T>(a: T, b: T) -> T //max<T:PartialOrg> でも可
    where T: PartialOrd + Debug
{
    if a >= b {
        a
    } else {
        b
    }
}

//ジェネリック型を用いた構造体について
struct Point<T> {
    x: T,
    y: T,
}

impl<T:PartialOrd + Debug> Point<T> {
    fn max(&self) ->&T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    //メソッド定義のジェネリック型と構造体のジェネリック型は必ずしも一致している必要はない
    //ここで，他のトレイト境界を持つような変数を用いた関数を定義してみる
    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x:{:?}", self.x);
        println!("val: {}", val);

    }
}

//特定の型の値の場合のみ機能するメソッド
impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}

//エラーハンドリングはResult型で行う
//Rustでは他の言語にあるようなエラーの爪がない
fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください．"))
    }
}

//エラーの移譲について

fn double_even(b: i32) -> Result<i32, String> {
    // match need_even(b) {
    //     Ok(val) => Ok(val * 2),
    //     Err(err) => Err(err),
    // }
    let x = need_even(b)?; //もし戻り値がOk(val)であればOkの中身であるvalを取り出してxに詰める，戻り値がErrであればそれをすぐに関数double_evenの呼び出し元に返す
    Ok(x * 2)
}
//これは，エラーハンドリングを関数側で規定せず，main()にその処理を移譲していることになる

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

    let rect = Rectangle{width: 2.0, height: 3.0};
    println!("{} and {}", rect.width, rect.height);
    let circ = Circle {
        radius: 2.0
    };
    Circle::do_something(); //do_somethingはprintln!を返す．関数
    println!("{}", circ.default_something()); //circ.default_somethingは単に&strを

    println!("Rectangle double area: {}", double_area(&rect));

    println!("{:?}", (1, 2, 3));

    #[derive(Debug, PartialEq)] //構造体を作るときは，いろいろと制約がある．deriveでDebugを乗せる必要がある
    // 自分で定義した構造体に演算など特定のメソッドを持たせたい場合、それに対応するトレイトを実装する必要があります
    struct S {
        val1: i32, 
        val2: i32,
    }

    let s_test = S {val1: 2, val2: 5};
    println!("s_test is test {:?}", s_test);

    println!("{:?}", S {val1: 1, val2: 2});

    let s1 = S {
        val1: 1,
        val2: 2
    };

    let s2 = S {
        val1: 1,
        val2: 3,
    };

    println!("{}", s1 == s2);

    println!("{}", max(2.0, 4 as f64));


    let p1 = Point {x: 2, y: 1};
    let p2 = Point {x: 2.0, y: 1.0};
    let p3 = Point {x: "a", y: "d"};
    println!("p1.max: {}", p1.max());
    p1.print_arg("test");
    p1.min(); //これはエラーにならない
    //p2.min() //これはエラーになる

    //パニックが起こると一時的に巻き戻しが起こり，順番に破棄されていく

    println!("{}", [1,2,3][0]);
    // 自力でパニックを発生させることもできる
    // panic!("This is my panic!");


    //result型を返す関数を使う場合，matchによる条件分岐が強制される
    let x = match need_even(2) {
        Ok(val) => val,
        Err(err) => {
            println!("Error message: {}", err);
            panic!()
        }
    };
    println!("{}", x);

    let s = need_even(1);
    println!("{}", s.is_ok()); //これはboolを返す
    println!("{}", s.is_err()); //これは

    // println!("{:?}",s.ok()); //some(2)またはNoneが返される．ただしここでは所有権が移動する！！！！

    // println!("{:?}", s.unwrap_or(0)); //ResultがOkならばその中の値を返し，Errならば引数のデフォルト値を返す
    // println!("{:?}", s.expect("expectから発生")); //matchを使うよりも簡潔なハンドリングができる

    println!("{:?}", double_even(3));

    match double_even(2) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("mainでハンドリング");
            println!("{}", err)
        }
    }

    let c1 = |x: i32| x + 1;
    println!("{:?}", c1(10)); //クロージャの引数を渡す

    let m = 10;
    let c2 =  |x:i32| x + m;
    println!("{:?}", c2(10));
    println!("{:?}", m);

    let m = 20;
    println!("{:?}", c2(20)); //クロージャ生成時点でのmの値を使い続ける，値の更新はない


    let v = vec![1, 2, 3];
    let c3 =  || { //ここでmoveを打つと，所有権の移動が起こるので，最終行がエラーになる
        println!("{:?}", v)
    };
    c3();
     println!("{:?}", v); //そのままだと変数の所有権の移動は起こらない


     //イテレータの例
     let v = vec![1, 2, 3, 4, 5];
     let v1_iter = v.iter();
     for x in v1_iter {
        println!("{:?}", x);
     }

     //next()を使うと，イテレータの消費が起こる
     //next()メソッドが呼ばれるたびに、イテレータの状態（次にどの要素を返すかの情報）が更新されるため，mutが必要
     let mut v2_iter = v.iter();
     println!("{:?}", v2_iter.next());
     println!("{:?}", v2_iter.next());
     println!("{:?}", v2_iter.next());
     println!("{:?}", v2_iter.next());
     println!("{:?}", v2_iter.next());
     println!("{:?}", v2_iter.next());

 








    








}


// Cargo.tomlに記載できる
// profile.devのopt-level = 0がデフォルト，profile.releaseのopt-level = 3がデフォルト
// コンパイルの最適化を表す