#[derive(Debug, Default)]
struct Foo {
    bar: Option<Bar>
}

#[derive(Debug, Default)]
struct Bar {
    baz: Option<Baz>
}

#[derive(Debug)]
struct Baz {
    qux: String
}

impl Default for Baz {
    fn default() -> Baz
    { 
        Baz { qux: "qux".to_string() }
    }
}

fn main() {
    println!("Nested Optional Structs");

    let mut foo = get_foo_maybe();

    //foo.iter_mut() = Some(Bar { baz: Some( Baz { qux: "qux".to_string() } ) } );

    println!("{:?}", foo);
    //println!("{:?}", foo.din().bar.din().baz.din().qux);

    foo.din().bar.din().baz.din().qux = "meep".to_string();

    println!("{:?}", foo);
    println!("{:?}", foo.din().bar.din().baz.din().qux);
}

fn get_foo_maybe() -> Option<Foo> {
    Some(Foo{ bar: None })
}

trait DefaultIfNone<T>
    where T: Default
{
    fn din(&mut self) -> &mut T; 
}

impl<T> DefaultIfNone<T> for Option<T> 
    where T: Default
{
    fn din(&mut self) -> &mut T {
        match self {
            Some(ref mut t) => t,
            None => {
                *self = Some(T::default());
                self.din()
            }
        }
    }
} 
