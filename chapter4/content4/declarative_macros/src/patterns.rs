macro_rules! match_all {
    ($($other:tt)*) => {
        println!("Matched: {:?}", stringify!($($other)*));
    };
}

macro_rules! recognize_tree {
    (larch) => {
        println!("Recognized the Larch tree!");
    };
    (oak) => {
        println!("Recognized the Oak tree!");
    };
    ($($other:tt)*) => {
        println!("Unknown tree!")
    };
}

macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}

macro_rules! count_tts {
    () => { 0 };
    ($_tt:tt $($rest:tt)*) => { 1 + count_tts!($($rest)*) };
}

pub fn demo() {
    println!("=== 4.1.5 Macro Patterns ===");

    println!("-- Match All --");
    match_all!(if let Some(x) = Some(5) {});

    println!("-- Callbacks --");
    recognize_tree!(larch);
    recognize_tree!(oak);
    recognize_tree!(pine);
    call_with_larch!(recognize_tree);

    println!("-- TT Munchers --");
    println!("count_tts!(a b c d e) = {}", count_tts!(a b c d e));

    println!();
}
