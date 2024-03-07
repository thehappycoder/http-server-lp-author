#[derive(Debug)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Russian,
  Estonian,
}

struct Greeting<'a> {
  message: &'a str,
  lang: Lang,
}

fn main() {
  let mut v: Vec<Greeting> = Vec::new();

  let g: Greeting = Greeting {
    lang: Lang::English,
    message: "Hello WasmEdge!",
  };
  v.push(g);
  let g: Greeting = Greeting {
    lang: Lang::Spanish,
    message: "Hola WasmEdge!",
  };
  v.push(g);
  let g: Greeting = Greeting {
    lang: Lang::Texan,
    message: "Howdy WasmEdge!",
  };
  v.push(g);
  let g: Greeting = Greeting {
    lang: Lang::Chinese,
    message: "WasmEdge 你好!",
  };
  v.push(g);

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }
}
