#pragma once

#include <algebra/vector.hh>

// Shift2

typedef struct {
    real2 v;
} Shift2;

Shift2 shf2_new(real2 v);

#define $Self Shift2
#define $self shf2
#define $elem real2
#include "interface.inl"
#undef $Self
#undef $self
#undef $elem

#ifdef HOST
template <>
struct Distance<Shift2> {
    static real distance(Shift2 a, Shift2 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

// Shift3

typedef struct {
    real3 v;
} Shift3;

Shift3 shf3_new(real3 v);

#define $Self Shift3
#define $self shf3
#define $elem real3
#include "interface.inl"
#undef $Self
#undef $self
#undef $elem

#ifdef HOST
template <>
struct Distance<Shift3> {
    static real distance(Shift3 a, Shift3 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST


#ifndef HOST
#include "shift.cc"
#endif // !HOST