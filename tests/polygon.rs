#![allow(dead_code, type_alias_bounds)]
use bounds_store::{bound_alias, bounds};

trait Float {
    fn foo();
}

type Point<F: Float> = (F, F);

trait Polygon<'a, F: Float>: 'a + IntoIterator<Item = &'a Point<F>>
where
    F: 'a,
{
    fn bar(&self);
}

bounds! {
    Polygon => <'a, F: 'a + Float, P: Polygon<'a, F>>
}

#[bound_alias(Polygon)]
fn area(polygon: P) -> F {
    F::foo();
    polygon.bar();
    unimplemented!()
}

fn main() {}
