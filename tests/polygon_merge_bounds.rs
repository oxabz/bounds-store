#![allow(dead_code, type_alias_bounds)]
use bounds_store::{bounds, bound_alias};

trait Float {
    fn foo();
}

type Point<F: Float> = (F, F);

trait Polygon<'a, F: Float>: 'a + IntoIterator<Item = &'a Point<F>> where F: 'a {
    fn bar(&self);
}

trait OtherTrait{
    fn baz();
}

bounds! {
    Polygon => <'a, F: 'a + Float, P: Polygon<'a, F>>
}


#[bound_alias(Polygon)]
fn area<F:OtherTrait>(polygon: P) -> F {
    F::foo();
    polygon.bar();
    F::baz();
    unimplemented!()
}

fn main() {}