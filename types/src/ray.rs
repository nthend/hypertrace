use std::marker::PhantomData;
use ccgeom::{Geometry3};
use crate::{
    Config, Map, Named,
    source::{Sourced, SourceTree, SourceBuilder},
    include_template,
};

pub struct RayMap<
    G: Geometry3,
    M: Map<G::Pos, G::Dir>,   
>(PhantomData<(G, M)>);

impl<
    G: Geometry3,
    M: Map<G::Pos, G::Dir>,    
> Named for RayMap<G, M> {
    fn type_name(_: &Config) -> String { "".into() }
    fn type_prefix(_: &Config) -> String { "".into() }
}

impl<
    G: Geometry3 + Sourced,
    M: Map<G::Pos, G::Dir>,    
> Sourced for RayMap<G, M> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/ray_map_{}.hh", Self::type_tag()))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .content(&include_template!(
                "geometry/ray_map.inl",
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::type_prefix(cfg),
            ))
            .build()
    }
}