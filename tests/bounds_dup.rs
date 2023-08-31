#[allow(dead_code)]
use bounds_store::{bounds, bound_alias};

trait Float {}

type Point<F: Float> = (F, F);

trait Polygon<'a, F: Float>: 'a + IntoIterator<Item = &'a Point<F>> where F: 'a {}

bounds! {
    Polygon => <'a, F: 'a + Float, P: Polygon<'a, F>>
}

bounds! {
    Polygon => <'a, F: 'a + Float, P: Polygon<'a, F>>
}

#[bound_alias(Polygon)]
fn area(_polygon: P) -> F {
    unimplemented!()
}

fn main() {}