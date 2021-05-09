use std::{marker::PhantomData};
use types::{Config, Named, Entity, SizedEntity, Sourced, source::{SourceTree, SourceBuilder, include}, include_template};
use type_macros::{Named, SizedEntity};
use ccgeom::{Geometry3};
use crate::{Shape, Material, Object};

#[derive(Clone, Copy, Debug, Named, SizedEntity)]
pub struct Covered<G: Geometry3, S: Shape<G>, M: Material> {
    geometry: PhantomData<G>,
    #[getter] pub material: M,
    #[getter] pub shape: S,
}

impl<G: Geometry3, S: Shape<G>, M: Material> Covered<G, S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self {
            geometry: PhantomData,
            material,
            shape,
        }
    }
}

impl<
    G: Geometry3 + Sourced,
    S: Shape<G> + Sourced,
    M: Material + Sourced,
> Sourced for Covered<G, S, M> where Self: SizedEntity {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/covered_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(S::source(cfg))
            .tree(M::source(cfg))
            .content(&include(&format!("geometry/ray_{}.hh", G::type_prefix(cfg))))
            .content(&include(&format!("render/light/{}.hh", G::type_prefix(cfg))))
            .content(&include_template!(
                "object/covered.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Shape": &S::type_name(cfg),
                "shape": &S::type_prefix(cfg),
                "Material": &M::type_name(cfg),
                "material": &M::type_prefix(cfg),
            ))
            .build()
    }
}

impl<
    G: Geometry3 + Sourced,
    S: Shape<G> + Sourced,
    M: Material + Sourced,
> Object<G> for Covered<G, S, M> where Self: SizedEntity {}
