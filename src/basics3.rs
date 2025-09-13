pub fn run3() {
    use std::rc::Rc;
    /*
    テキスト領域（機械語に翻訳されたプログラム），静的領域はconst，文字列リテラルなどの静的変数
    スタック領域（コンパイル時に確保すべきメモリ量が決定できる変数：i32であれば4バイト必要，など）Rustでは8MBしかない LIFO
    ヒープ領域：動的に変化するデータ（string, vectorなど）
    柔軟なメモリ管理が行える一方で，メモリリーク，二重解放などのバグの可能性が高まる．
    一方でプログラムが責任を持って確保と解放を行う場合はスピードが低速．
    所有権：メモリ上に存在する値を変数が所有するという考え方．所有権の移動や借用が行われる．コンパイル時に所有権チェックが行われる
    所有権のルール：1) 各値が所有者と呼ばれる変数に対応している．
    2) 値に対する所有者は必ず1つ．v2 = v1 とすると，v1に対応する値の所有権がv2に移る．
    3) 所有者がスコープから外れたら値は破棄される（つまりブロック内変数でブロックを抜けたら所有者が先に抜けるので値も消える）．
    メモリの生存期間（ライフタイム）が変数のスコープと同じになるので，コンパイル時に確定する．

    v1 = vec![1, 2, 3]とすると
    スタックにはptr len（長さ3） capacity（メモリ）→もしこの上限を超えたときに別の場所に移動する
    値のコピーには，v2 = v1.clone(); とする．コピー元のサイズは問題ないかなど注意が必要
    数値型や論理型などスタック領域に格納される値ではコピーが行われる．値をコピーしたとしても十分高速であると保証できるため
    実際にはcopyトレイとを実装した型は移動ではなくコピー：int/float/bool/str/tuple（要素がすべてcopy型である場合）



     */

    
    let a = 100;
    {
        let mut v1 = vec![1, 2, 3];
        println!("{:?}",v1);

        let mut v2 = v1; //この時点でvec!の所有権がv2に移動する
        println!("{:?}",v2);
        // println!("{:?}",v1); //これはエラーになる

        v2.push(4);
    }

    //関数で所有権が移動するケース

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s = concat(s1, s2); //この時点でs1とs2の所有権がconcatの引数aとbに入ってしまう
    println!("{}", s);
    // println!("{}", s1); //これはエラー
    // println!("{}", s2); //これはエラー

    let mut v1 = vec![1, 2, 3];
    println!("v1 ptr:{:?}", v1.as_ptr()); //ベクタが所有するヒープ上の最初の要素への生のポインタ
    println!("v1[0]:{:p}", &v1[0]); //{:p}を使うことでアドレス値を取得できる

    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity()); //capacityは3になる
    
    v1.push(4);
    println!("v1 ptr:{:?}", v1.as_ptr()); //ベクタが所有するヒープ上の最初の要素への生のポインタ
    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity()); //新たにcapcityの異なるメモリ領域を確保する

    let v2 = v1;
    println!("v2 ptr:{:?}", v2.as_ptr()); //移動が起こる

    let v3 = v2.clone();
    println!("v2 ptr:{:?}", v2.as_ptr());
    println!("v3 ptr:{:?}", v3.as_ptr()); //cloneでコピーすると参照先も変わる

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let (s, s1, s2) = concat_new(s1, s2);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2); //これはエラーにならない

    //混乱してしまったので，以下を試す
    let mut t1 = String::from("Hello");
    println!("t1 to: {:?}", t1.as_ptr());
    t1.push_str(", Rust");
    println!("new t1 to: {:?}", t1.as_ptr());


    /*
    参照：所有権を持たないポインタ
    値を代入しても所有権が移動しない．したがって値の生存期間にも影響しない
    変数に&をつけることで作成される．値を参照することを「借用する」と呼ぶ */
    
    let x1 = vec![1, 2, 3];
    let x2 = &x1; // この時点で所有権は移動しない
    println!("{:?}", x1);
    println!("{:?}", x2);

    /*
    参照は
    共有参照（参照先を読むことはできるが，変更は不可）：共有参照は同時に複数作成することもできる，値が複製される心配がないので
    可変参照：値の読み出しと変更が可能，変数に&mutをつけることで作成する．&mut vとして参照する，可変参照が存在する場合はその他の参照は作成不可

     */

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s = concat_ref(&s1, &s2); // ここも&を渡さないといけない
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2); //これはエラーにならない

    //ライフタイムは推論により省略されている．省略セずに書く場合，ライフタイムパラメータを'aのように書ける（tick A）．
    // fn f<'a>(p:&'a;i32){}

    {
        let r;
        {
            let x = 1;
            r = &x;
            println!("{}", r); //xがこのブロックが終了するとライフタイムは終了
        }
        // println!("{}", r); 
    }

    /*
    スマートポイントについて
    ボックスは値をヒープ領域に取る．通常コンパイル時にサイズが確定できないようなものはboxを使用する（明示的にヒープに値を取る）．
     */
    let x=Box::new(1);
    println!("x:{:p}", x);
    println!("+x * 2 = {}", *x + 2); //xは単に1という値が格納されているヒープ領域へのポインタしか返してくれないので，値にアクセスするためには*xが必要

    /*
    Rcスマートポインタ
    基本的に1つのデータは1つの所有者と紐づけられるが，例えばツリー構造のデータなどで
    1つのデータに対して複数の所有者が必要なときは，このスマートポインタを使う
     */

    let a = Rc::new("hello".to_string());
    println!("count1:{}", Rc::strong_count(&a));
    {
        let b= Rc::clone(&a);
                println!("a:{:p}", a);
        println!("b:{:p}", b);
        println!("count2: {}", Rc::strong_count(&a));
    }
    println!("count1:{}", Rc::strong_count(&a));





}

fn concat(a: String, b: String) -> String{
    let c: String = format!("{}, {}", a, b);
    c
}

fn concat_new(a: String, b: String) -> (String, String, String){
    let c: String = format!("{}, {}", a, b);
    (c, a, b)
}

fn concat_ref(a: &String, b: &String) -> String{
    let c: String = format!("{}, {}", a, b);
    c
}