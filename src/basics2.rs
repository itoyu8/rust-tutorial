pub fn run2() {

    /*
    式はなんらかの評価を行い変数に束縛できる（戻せる）．
    文は処理は実行する，式は文の部分要素となる，変数に束縛できない．セミコロンが必要 */

    println!("a");
    let y = 0;

    {
        let x = 1; // ブロック内でのみ変数が有効
    }

    println!("{}", y);
    {
        let y = 5;
        println!("{}",y); // 変数のシャドーイングのためここでは5が出力

    }
    println!("{}",y);

    let z = {
        100 //ここが最後の行に相当し，セミコロンのない値なので，このブロック自体が100を返していると解釈される
    };

    println!("{}", z); // なのでこれでzに100が代入される

    let x = -5;
    if x > 0 && x < 10 { //空のリストならfalse，などの制約は存在しないのでboolの条件として入れる
        println!("x is 0 < x < 10");
    }
    
    if x > 0 || x < 10{
        println!("x > 0 or x < 10"); // orの場合はこれ
    }

    if x > 0 && x < 10 { //第一の条件
        println!("first condition");
    } else if x > 11 && x <= 20 { //第二の条件
        println!("2nd condition");
    } else {
        println!("else");
    }

    let y = if x > 0{
        x //値を入れることもできる
    } else {
        0 //すべてのelseまで含めて何らかの値を返すこと，返り値の形式はすべてで共通であること
    };

    println!("{}",y);

    let x = 2; //if-elseではなくてmatchでいける
    match x {
        0 => println!("zero!"),
        1 => {
            println!("one!");
            println!("one!");
        },
        _ => println!("Other!"), //ワイルドカードをつける．matchの場合はすべての分岐を網羅していないとコンパイルエラーになる
    };   

    let y= match x { //
        0 => 0,
        1 => 100,
        _ => 1000,
    };   

    // loop
    let mut cnt = 0;
    loop{
        println!("Hello!");
        if cnt == 10{
            break;
        }
        cnt += 1;
    }

    // while
    let mut cnt = 0;
    while cnt < 10 {
        println!("Hello");
        cnt += 1;
    }

    // for
    for i in [1, 2, 3]{
        println!("Hello,{}",i);
    }

    let r = 1..=10; //rangeを計算する時は別の変数に格納しておくとよい
    for x in r {
        println!("{}", x * x);
    }


}


pub fn fizzbuzz1(end: i32){
    let mut cnt = 1;
    while cnt < end {
        if cnt % 15 == 0 {
            println!("FizzBuzz");
        } else if cnt % 5 == 0 {
            println!("Buzz");
        } else if cnt % 3 == 0 {
            println!("Fizz");
        } else{
            println!("{}",cnt);
        }
        cnt += 1;
    }
}

pub fn fizzbuzz2(end: i32){
    for x in 1..=end {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}",x),
        }
    }

}

pub fn fizzbuzz3(end: i32){
    for x in 1..=end {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _=> println!("{}",x),
        }
    }

}