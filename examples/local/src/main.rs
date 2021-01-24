#![feature(unboxed_closures)]

use hotpatch::*;

#[allow(non_upper_case_globals)]
static foo: Patchable<Box<dyn Fn(&str) -> &str + Send + Sync + 'static>> = Patchable::__new(|| {
    // direct copy
    fn foo(a: &str) -> &str {
        println!("I am Foo {}", a);
        a
    }
    Patchable::__new_internal(Box::new(foo), "local::foo", "fn(i32) -> ()")
});

#[allow(non_upper_case_globals)]
static foo2: Patchable<Box<dyn Fn(i32) -> i32 + Send + Sync + 'static>> = Patchable::__new(|| {
    // direct copy
    fn foo2(a: i32) -> i32 {
        println!("I am Foo {}", a);
        a
    }
    Patchable::__new_internal(Box::new(foo2), "local::foo", "fn(i32) -> ()")
});

// /// I'm a functor
// #[patchable]
// fn foo(_: i32) {
//     println!("I am Foo");
// }

// /// I'm a function with extra bits
// #[patch]
// fn tmp(_: i32) {

// }

fn bar(_: &str) -> &str {
    println!("Foo Becomes Bar");
    ""
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    foo("1");
    foo.hotpatch_fn(bar)?;
    foo("2");
    let a = 5;
    foo.hotpatch_fn(move |_: &str| {
        println!("Foo becomes anonymous {}", a);
        ""
    })?;
    foo("3");

    foo2(2);
    foo2.hotpatch_fn(|_| 0);
    Ok(())
}
