macro_rules! _print {
    () => {};
    ($x: expr) => {
        print!("{}, ", $x)
    };
    ($x: expr, $y: expr $(, $z: expr)*) => {
        print!("{}, ", $x);
        _print!($($z),*);
    };
}

macro_rules! _revprint {
    () => {};
    ($x: expr) => {
        print!("{}, ", $x)
    };
    ($x: expr, $y: expr $(, $z: expr)*) => {
        _revprint!($($z),*);
        print!("{}, ", $x);
    };
}

macro_rules! myprint {
    ($x: expr $(,$y: expr)*) => {
        _print!($x $(,$y)*);
        _revprint!($($y),*);
        println!("");
    };
}

fn main() {
    // we want: 1, 3, 5, 6, 4, 2
    myprint!(1, 2, 3, 4, 5, 6);
    // want: 1, 3, 5, 4, 2
    myprint!(1, 2, 3, 4, 5);
    myprint!(1);
    myprint!(1, 2);
}
