
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

fn main() {

    // iter(), iter_mut(), into_iter()
    vec![3.0, 4.2].iter().any(|&x| x == 4.2); // return &T
    let mut x = vec![1.0];
    let mut x = x.iter_mut();         // return &mut T
    *x.next().unwrap() += 1.0;
    vec![3.0, 4.2].into_iter().fold(0.0, f64::max); // return T

    // Closure - Ownership moved or not
    let mut i = 5;
    (|x: i32| i += x)(5); // ownership of i is not moved
    assert_eq!(10, i);

    let mut i = 5;
    (move |x: i32| i += x)(5); // ownership of i is moved
    assert_eq!(5, i);

    //
    // Borrowing is a reference as a function parameter
    //   - guaranteed valid memory and non-Null

    // Example of immutable borrowing
    let s = String::from("hello");
    println!("{}", (|s: &String| s.len() )(&s));

    // Example of mutable borrowing
    let mut s = String::from("hello");
    println!("{}", (|s: &mut String| s.pop().unwrap() )(&mut s));
    // s can be mutably borrowed again as the above closure has scoped out here.
    println!("{}", (|s: &mut String| s.pop().unwrap() )(&mut s));

    // Dangling refference not allowed -> The borrowed value goes out of it's scope(closure)
    // let s = (|| &String::from("world"))();
    let s = (|| String::from("world"))();     // Just pass the ownership
    println!("{}", s);

    // Dangling refference as no one takes ownership of String
    // let s: &str = String::from("world").as_str();
    println!("{}", String::from("folks").as_str()); // ok as function takes as a local variable


    //
    // Raw pointer
    //   - not guaranteed valid memory and non-Null (unlike & and Box)
    //   - require manual resource management (unlike Box)
    //   - ownership not moved (unlike Box)
    //   - no lifetimes checked (unlike &)

    // Example of raw pointer
    let raw_imm = &99 as *const i32; // explicit const needed
    let raw_mut: *mut i32 = &mut 100;
    println!("{}", unsafe { *raw_imm + *raw_mut }); // Dereference only allowed inside unsafe

    // Conversion to reference
    let ref_imm: &i32;
    let ref_mut: &mut i32;
    unsafe {
        ref_imm = &*raw_imm;
        ref_mut = &mut *raw_mut;
    }
    println!("{}", ref_imm + *ref_mut);

    // Raw pointer allows access to non-valide memory
    unsafe {
        println!("{}", (|| String::from("ownership passed"))()); 
        // compiler allows below but runtime error as bad address
        // println!("{}", &*(|| &String::from("not passed") as *const String)());
    }
    
    //
    // Box (Smart Pointer: Deref trait)
    //   Main use is to wrap unknown size types in a recursive type definition
    //
    //   - guaranteed valid memory and non-Null
    //   - ownership moved, and manual resource management not needed
    //   - multiple ownership not allowed

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    // ?? cant just use & instead of Box?
    //       -> can but need lifetime?
    // enum List<'a> {
        // Cons(i32, &'a List<'a>),
        // Nil,
    // }

    let a = Box::new(5); // heap allocated

    let ls_box = List::Cons(*a, // a has Deref trait
        Box::new(List::Cons(*a,
            Box::new(List::Nil))));

    println!("{:?}", List::Cons(*a, Box::new(ls_box)));
    // not allowed to use ls_box as the value's moved above
    // List::Cons(*a, Box::new(ls_box));


    //
    // Rc (Smart Pointer: counts the references to itself)
    //   - to allow multiple ownership unlike Box and RefCell
    //   - cost at runtime

    #[derive(Debug)]
    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }

    let a = Box::new(5); // heap allocated
    let ls_rc = Rc::new(RcList::Nil);

    println!("{:?}", Rc::new(RcList::Cons(*a, Rc::clone(&ls_rc)))); // increments count by cloning
    println!("{:?}", Rc::new(RcList::Cons(*a, ls_rc))); // count gets 0 here
    // println!("{:?}", Rc::new(RcList::Cons(*a, ls_rc))); // ls_rc has been dropped above


    //
    // Cell
    //
    //   Zero cost internal mutability only for Copy types
    //   Copy type means the owned data is on the stack as size-known,
    //   So mutability is not a problem.
    //
    //      - use get() and set() methods to handle data
    //      - usefull for Rc to have mutable value(Rc<Cell<_>>) 
    //        because Rc<mut _> is not possible

    // let x = Cell::new("dfs".to_string()); // String is not Copy type
    let a = Box::new(5); // heap allocated

    let x = Cell::new(*a); // a is copied onto stack
    let y = &x;
    let z = &x;
    x.set(x.get() + 2);
    y.set(y.get() + 30);
    z.set(z.get() + 400);
    println!("{}", x.get());
    println!("{}", *a);

    //
    // RefCell
    //   Runtime cost internal mutability for Sized types
    //   Sized type has a constant size known at compile 
    //   Single ownership only
    //
    //      - unlike Cell, internal value can be taken out by Ref or RefMut
    //      - use borrow() and borrow_mut(), which increment reference count
    //
    //      !Error at runtime, so not recommended to use unless necessary

    // let x: Vec<i32> = vec![];
    // x.push(1);       // not allowed becase x is immutable
    let x = RefCell::new(vec![]);
    // x.push(1);       // does not have DeRef
    // *x.push(1);      // does not have DeRef
    x.borrow_mut().push(1);
    let y = x.borrow();
    // let mut z = x.borrow_mut();  // not allowed
    let z = x.borrow();
    println!("{:?}, {:?}", y, z);


    // Reference cycle
    // https://doc.rust-jp.rs/book/second-edition/ch15-06-reference-cycles.html
    // 
    
    // https://qiita.com/hibariya/items/b24f893f88d0dc931c61
    // Borrow , AsRef   


    //
    // Lifetime
    //   To help Rust check the reference validity
    //   Variables annotated with same lifetime must be droped in the same scope
    //
    //  - implicit and inferred when 3 check rules are applicable
    //       - multiple ref arg get different lifetimes
    //       - all returned refs get same lifetime if only one ref arg
    //       - all returned refs get same lifetime as self regardless of multiple ref args

    // 2nd check rule(implicit lifetime to the returned) not applicable in case of multiple same type ref args
    fn func_a<'a>(x: &'a str, y: &'a str) -> &'a str {
    // fn func(x: &str, y: &str) -> &str { // implicit x: &'a str, y: &'b str
        if x.len() > y.len() {x} else {y} // must tell 2 args and returned must be dropped in the same scope
    }

    let x = String::from("abc");
    let y = String::from("abcd");
    let mut z;

    {   
        z = func_a(x.as_str(), "ab");    // sounds like ok as "ab" not heap allocated
        // let y = String::from("abcd"); // not ok as y's lifetime is not same as x
        z = func_a(x.as_str(), y.as_str());
    }
    println!("{}", z);

    // fn func1(x: &str, y: &str) -> &str { // 2nd check rule also not applicable
    fn func_ab<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        println!("{}", y);
        x // although it's obvious
    }
    func_ab(x.as_str(), y.as_str());

    // 3rd rule applied
    struct Foo { hoge: String,}
    impl Foo {
        fn func(&self, x: &str, y: &str) -> &str { // same lifetime as self given to the returned
            if x.len() > y.len() {
                println!("x");
            }
            // x   // just validity check fails after lifetimes are correctly given
            self.hoge.as_str()
        }
    }
    println!("{}", 
        Foo {hoge: "hoge".to_string()   }.func(x.as_str(), y.as_str())
    )


}

// fn main() {
//     let mut xs = [0; 120];
//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     let n: usize = line.trim().parse().unwrap();
//     for _ in 0..n {
//         line.clear();
//         std::io::stdin().read_line(&mut line).unwrap();
//         let v: Vec<i8> = line.split_whitespace()
//             .map(|s| s.parse().unwrap())
//             .collect();
//         let (b, f, r, p) = (v[0] as usize, v[1] as usize, v[2] as usize, v[3]);
//         xs[(b-1)*30+(f-1)*10+r-1] += p;
//     }
//     for (i, p) in xs.iter().enumerate() {
//         if i%30==0 && i!=0 {
//             println!("####################", );
//         }
//         print!(" {}", p);
//         if (i+1)%10==0 {
//             println!("");
//         }
//     }
// }


// use std::io;
// use std::io::BufRead;
// use std::collections::HashSet;

// fn main() {
//     let mut line = String::new();
//     io::stdin().read_line(&mut line).unwrap();
//     let n: usize = line.trim().parse().unwrap();

//     let mut set: HashSet<String> = HashSet::new();
//     let stdin = io::stdin();
//     for (i, line) in stdin.lock().lines().enumerate() {
//         set.insert(line.unwrap().trim().to_string());
//         if i == n-1 {break};
//     }

//     for s in ['S', 'H', 'C', 'D'].iter() {
//         for i in 1..14 {
//             let s = format!("{} {}", s, i);
//             if !set.contains(&s) {
//                 println!("{}", s);
//             }
//         }
//     }
// }


// use std::io;
// use std::io::BufRead;
// use std::collections::HashSet;

// fn main() {
//     let mut set = HashSet::new();
//     let stdin = io::stdin();
//     for line in stdin.lock().lines().skip(1) {
//         set.insert(String::from(line.unwrap().trim()));
//     }
//     for s in ['S', 'H', 'C', 'D'].iter() {
//         for i in 1..14 {
//             let s = format!("{} {}", s, i);
//             if !set.contains(&s) {
//                 println!("{}", s);
//             }
//         }
//     }
// }

// use std::io;
// use std::io::BufRead;
// use std::collections::HashSet;

// fn main() {
//     let mut line = String::new();
//     io::stdin().read_line(&mut line).unwrap();
//     let n: usize = line.trim().parse().unwrap(); 
//     let mut set = HashSet::new();
//     let stdin = io::stdin();
//     for (i, line) in stdin.lock().lines().enumerate() {
//         set.insert(String::from(line.unwrap().trim()));
//         if i == n-1 {break}
//     }
//     for s in ['S', 'H', 'C', 'D'].iter() {
//         for i in 1..14 {
//             let s = format!("{} {}", s, i);
//             if !set.contains(&s) {
//                 println!("{}", s);
//             }
//         }
//     }
// }


// use std::io;
// use std::io::BufRead;

// fn main() {
//     let x = String::from('s');
//     println!("{}", x);
    // let mut xs = [false; 52];
    // let mut line = String::new();
    // io::stdin().read_line(&mut line).unwrap();
    // let n:u8 = line.trim().parse().unwrap();
    // for _ in 0..n {
    //     let mut line = String::new();
    //     io::stdin().read_line(&mut line).unwrap();
    //     let v: Vec<String> = line.split_whitespace()
    //         .map(|s| s.parse().unwrap()).collect();
    //     let (s, r): (char, usize) = (FromStr::from_str(v[0]), v[1].parse().unwrap());
    //     // let (s, r): (char, usize) = (v[0], v[1].to_digit(10).unwrap() as usize);
    //     for (i, c) in ['S', 'H', 'C', 'D'].iter().enumerate() {
    //         if *c == s { xs[i*13+r-1] = true; }
    //     }
    // }
    // for (i, b) in xs.into_iter().enumerate() {
    //     if !b {println!("{} {}", ['S', 'H', 'C', 'D'][i/13], i%13+1);}
    // }
// }


// use std::io;
// use std::io::BufRead;

// fn main() {
//     let stdin = io::stdin();
//     let line = stdin.lock().lines().nth(1).unwrap().unwrap();
//     let s = line.split_whitespace().rev().collect::<Vec<_>>().join(" ");
//     println!("{}", s);
// }

// fn main() {
// 	let mut line = String::new();
// 	std::io::stdin().read_line(&mut line).unwrap();
// 	let n: u16 = line.trim().parse().unwrap();
// 	for i in 3..n+1 {
// 		if i % 3 == 0 || 
// 			match i.to_string().find('3') { 
// 				Some(_) => true,
// 				None => false,
// 			}
// 		{
// 			print!(" {}", i);
// 		}
// 	}
// 	println!("");
// }


// use std::io;
// use std::io::BufRead;

// fn main() {
// 	let stdin = io::stdin();
// 	for line in stdin.lock().lines() {
// 		let v: Vec<usize> = line.unwrap().split_whitespace()
// 			.map(|s| s.parse().unwrap()).collect();
// 		if v == [0, 0] {break;}
// 		let (h, w) = (v[0], v[1]);
// 		let mut row1 = "#.".repeat(w/2+w%2);
// 		let mut row2 = ".#".repeat(w/2+w%2);
// 		if w%2==1 {
// 			row1.pop();
// 			row2.pop();
// 		}
// 		let mut rect = (row1 + "\n" + &row2 + "\n").repeat(h/2+h%2);
// 		if h%2==1 {
// 			for _ in 0..w+1 {rect.pop();}
// 		}
// 		println!("{}", rect);
// 	}
// }


// use std::io;
// use std::io::BufRead;

// fn main() {
// 	let stdin = io::stdin();
// 	for line in stdin.lock().lines() {
// 		let v: Vec<usize> = line.unwrap().split_whitespace()
// 			.map(|s| s.parse().unwrap()).collect();
// 		let (h, w) = (v[0], v[1]);
// 		let s = "#".repeat(w) + "\n";
// 		let rect = match (h, w) {
// 			(0, 0) => break,
// 			(x, y) if x < 3 || y < 3 => s.repeat(h),
// 			(_, _) => s + &("#".to_string() + &(".".repeat(w-2)) + "#\n").repeat(h-2) + &("#".repeat(w)) + "\n",
// 		};
// 		println!("{}", rect);
// 	}
// }