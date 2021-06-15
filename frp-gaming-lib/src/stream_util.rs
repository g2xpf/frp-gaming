use sodium_rust::{IsLambda1, Stream};

pub fn filter_map<A, B, FN>(s: &Stream<A>, f: FN) -> Stream<B>
where
    FN: IsLambda1<A, Option<B>> + Send + Sync + 'static,
    A: Clone + Send + 'static,
    B: Clone + Send + 'static,
{
    s.map(f).filter_option()
}
