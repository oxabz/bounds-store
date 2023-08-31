#![allow(dead_code, type_alias_bounds)]
use bounds_store::{bound_alias, bounds};

trait Float {
    fn foo();
}

type Point<F: Float> = (F, F);

trait Polygon<F: Float>
where
    for<'a> &'a Self: IntoIterator<Item = &'a Point<F>>,
{
    fn bar(&self);
}

bounds! {
    Polygon => <F: Float, P: Polygon<F>>
        where for<'a> &'a P: IntoIterator<Item = &'a Point<F>>;
}

#[bound_alias(Polygon)]
fn area(polygon: P) -> F {
    F::foo();
    polygon.bar();
    unimplemented!()
}

fn main() {}
