// structureの導入
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体のメソッドの定義
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }//selfの型名は省略可

    //型関連関数．この場合は&selfは第1引数にならない
    //明示的にnew関数を置くことにより，インスタンス生成の歳に特定の条件を満たしているかを検証するロジックを組み込める
    // 特定のクラスのインスタンスが生成されるときに自動的に呼び出されるメソッドをコンストラクタと呼ぶが
    // Rustではコンストラクタという概念はないので，明示的にnew()で型関連関数を呼ぶ
    fn new(width: u32, height: u32) -> Self{
        Rectangle {width, height}
    }
}

//列挙型（という名前で新しい型を定義できる）．列挙型がとりうる値のことをバリアントまたは構成子と呼ぶ
enum Shape {
    Circle, 
    Square(u32), //データをもたせることもできる，タプル型バリアント．
    Triangle{base:u32, height:u32}, //keyとvalueでデータをもたせることもできるが，これはstructの構文．構造体バリアント．
}

//列挙型の各バリアントに対してもメソッドを定義できる
//特定のバリアントに対してのみメソッドを定義することはできない．match式を用いてバリアントごとの処理を記述する
impl Shape{

    fn sample_method(&self){
        println!("call sample_method");
    }
}


pub fn run4() {
    println!("hello, world");

    //インスタンスの呼び方
    let height = 5;
    let mut rectangle = Rectangle{
        width: 10, // インスタンス化する
        height, //同名変数であればそのまま値を挿入できる
    };
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    rectangle.height = 10; // let mut rectangle: とすると，あとから値の追記が可能
    println!("height changed to: {}", rectangle.height);

    println!("area: {}", rectangle.area());

    //コンストラクタ（つまり::new）を明示的に呼んでも良い．
    let mut rectangle2 = Rectangle::new(10, 5);
    println!("new width:{}", rectangle2.width);

    //列挙型を呼ぶ
    let c = Shape::Circle;
    let s = Shape::Square(1);
    let t = Shape::Triangle{base:10, height:5};

    c.sample_method();
    s.sample_method();
    t.sample_method();


    //オプション型のジェネリック型T
    let a = Some(1);
    let b = Some("str");
    let c: Option<i32> = None; //Noneを使用する場合は明示的に型を指定する

    let v = vec![1, 2, 3];
    let val = v.get(2); //indexが存在すれば，その要素への参照をSomeに包んで返す，したがってoptionになる

    // 以下の式は，valがOption型（つまりSomeかNoneしかない）という状況を想定している
    match val {
        Some(x) => println!("value exists:{}",x),
        None => println!("value is None"),
    }

    // valがoption型の場合に，matchは可能性がある分岐をすべて網羅する必要があるが，if letだと記述がない分岐について無視される
    if let Some(x) = val {
        println!("val={}", x)

    }

    // Someのときにさらに特定の値による分岐を入れられる
    match val {
        Some(1) => println!("value is 1"),
        Some(2 | 3 ) => println!("value is 1 or 2"),
        Some(x) if *x == 4 => println!("value is 4"), //これをマッチガードと呼ぶ．if letでは使用不可
        Some(x) => println!("value exists:{}",x),
        None => println!("value is None"),
    }





}