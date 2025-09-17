//! これはライブラリクレートのドキュメント

/// say_hello関数のドキュメント
pub fn say_hello() {
    println!("Hello!");
}

//トレイトにはメソッドのシグネチャーのみを実装する．トレイトに記載されたメソッドの実装を強制する
//任意の型とトレイトを結びつけることをトレイトを実装するとよぶ
//main.rsに定義することもできるが今回はlib.rsに入れる
//// 構造体のメソッドの定義はこれだった
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }//selfの型名は省略可

//     //型関連関数．この場合は&selfは第1引数にならない
//     //明示的にnew関数を置くことにより，インスタンス生成の歳に特定の条件を満たしているかを検証するロジックを組み込める
//     // 特定のクラスのインスタンスが生成されるときに自動的に呼び出されるメソッドをコンストラクタと呼ぶが
//     // Rustではコンストラクタという概念はないので，明示的にnew()で型関連関数を呼ぶ
//     fn new(width: u32, height: u32) -> Self{
//         Rectangle {width, height}
//     }
// }

pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something(); //構造体と同様に，引数にselfを取らない型関連関数も定義できる
        fn default_something(&self) -> &str {
            "This is default method!" //デフォルトメソッド
        }
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle { //すべてのtraitを実装する必要がある
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn do_something() {
            println!("This is Rectangle function");
        }
        fn default_something(&self) -> &str {
            "This is rectangle default!"
        }


    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64 {
            2.0 * self.radius * std::f64::consts::PI
        }

        fn do_something(){
            println!("This is circle function");
        }
    }

      //面積の2倍という関数を実装するときに，トレイトを共有するすべての構造体に対する参照を渡して定義する
      //これは関数なので，呼び出すときはインスタンスに対する参照（&）を渡す
    pub fn double_area(shape: &impl Shape) -> f64{
        shape.calc_area() * 2.0
    }


}