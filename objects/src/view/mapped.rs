use ccgeom::Geometry3;
use types::{Named, Entity, Sourced, Config, Map, source::{SourceTree, SourceBuilder, include}, include_template};
use std::{marker::PhantomData};
use type_macros::{Named, SizedEntity};
use crate::View;


#[derive(Clone, Debug, Named, SizedEntity)]
pub struct MappedView<G: Geometry3, V: View<G>, M: Map<G::Pos, G::Dir>> {
    pub inner: V,
    pub map: M,
    phantom: PhantomData<G>,
}

impl<
    G: Geometry3,
    V: View<G>,
    M: Map<G::Pos, G::Dir>,
> MappedView<G, V, M> {
    pub fn new(inner: V, map: M) -> Self {
        Self { inner, map, phantom: PhantomData }
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> Sourced for MappedView<G, V, M> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .tree(V::source(cfg))
            .content(&include(&format!("geometry/ray_{}.hh", &G::type_prefix(cfg))))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::type_prefix(cfg),
                "View": &V::type_name(cfg),
                "view": &V::type_prefix(cfg),
            ))
            .build()
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> View<G> for MappedView<G, V, M> {}