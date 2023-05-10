use std::{collections::HashMap, io::StdoutLock};

fn main() {
	// 変数
  // 宣言の後に=で値を指定することを、"束縛"という。代入とは違う。

	// 定数
	// const宣言、型アノテーションが必須
	const URL_CONST:&str = "https://example.com";

	// # データ型

	// 文字列スライス ⇒ コンパイル時にサイズ固定。高速。参照のみに利用。
	let msg = "hello, world";
	// String型 ⇒ サイズ可変。更新、結合が可能。
	let mut msg = String::from("hello, world");
	msg.push_str(". KS!"); // 破壊的
	println!("{}", msg);

	// タプル(混合型)
	let a = ("hoge", 123);
	// 配列(統一型)
	let b = ["hoge", "fuga"];

	// ## 可変長コレクション. push等は破壊的メソッドなので、mut宣言が必要
	let mut v = Vec::new();
	v.push(99);// 破壊的
	// 上記のvec方法よりも、こちらの初期化利用に対応したマクロの利用を推奨
	let v = vec![1,4,9];
	println!("{:?}", v);

	// Hash Map
	// 伸長可能
	// 最初にキーバリューの型を定義できるが、直ぐにinsertするなら型推論が働くので不要
	// let mut scores: HashMap<&str, i64> = std::collections::HashMap::new();
	let mut scores = std::collections::HashMap::new();
	// scores.insert("hoge", 85); // &str: 文字列スライス判定. 切り替えると推論が変わるから面白い
	scores.insert(String::from("hoge"), 85); // String型判定

	// HashMap::fromで、タプル配列から初期値生成可能
	// let solar_distance = std::collections::HashMap::from(
	// 	{}
	// );

	// # 所有権
	// ## Case 1
	// let a = String::from("hoge");
	// let b = a;
	// // 上記の記述で、権利はbにmoveしている
	// // 所有権を失った値への参照は禁止されている
	// print!("{}", a)

	// ## Case 2
	// このコンパイルは通る
	// i32型は、Copyトレイトを実装いるため、束縛ではなくコピーされる
	let a = 100;
	let b = a;
	print!("{}", a);
	print!("{}", b);

	// # 借用
	// let world = String
	// hello_print();
}

fn hello_print(message: String){
	println!("hello, {}", message);
}