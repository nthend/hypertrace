#pragma once

#include "vector.hh"
#include "complex.hh"
#include "quaternion.hh"


typedef real8 comp2x2;

typedef real16 real4x4;


#define c22_new MAKE(comp2x2)

comp2x2 c22_zero();
comp2x2 c22_one();

comp2x2 c22_transpose(comp2x2 m);
comp2x2 c22_mul(comp2x2 m, comp c);
comp2x2 c22_div(comp2x2 m, comp c);
real c22_norm_l1(comp2x2 m);

comp2x2 c22_dot(comp2x2 a, comp2x2 b);

comp c22_det(comp2x2 m);
comp2x2 c22_normalize(comp2x2 m);

comp2x2 c22_inverse(comp2x2 m);
comp2x2 c22_inverse_n(comp2x2 m);

void c22_eigen(comp2x2 m, comp2x2 *l, comp2x2 *v);
void c22_eigen_n(comp2x2 m, comp2x2 *l, comp2x2 *v);

comp2x2 c22_pow(comp2x2 m, real p);
comp2x2 c22_pow_n(comp2x2 m, real p);


#define r44_new MAKE(real4x4)

real4x4 r44_zero();
real4x4 r44_one();

real4x4 r44_transpose(real4x4 m);
real r44_norm_l1(real4x4 m);

real4x4 r44_dot(real4x4 a, real4x4 b);
real4 r44_dot_mv(real4x4 a, real4 b);
real4 r44_dot_vm(real4 a, real4x4 b);
real4x4 r44_outer(real4 a, real4 b);

real r33_det(real4x4 m);
real4x4 r33_inverse(real4x4 m);

real4x4 r33_clip(real4x4 m);


#ifdef UNIT_TEST

class TestRngComp2x2 {
private:
    TestRng<comp2x2> rng;

public:
    inline TestRngComp2x2() = default;
    inline explicit TestRngComp2x2(uint32_t seed) : rng(seed) {}

    comp2x2 normal();
    comp2x2 uniform();
    comp2x2 invertible();
    comp2x2 normalized();
};

class TestRngReal3x3 {
private:
    TestRng<real4x4> rng;
    static const real4x4 mask;

    static real4x4 clip(real4x4 m);

public:
    inline TestRngReal3x3() = default;
    inline explicit TestRngReal3x3(uint32_t seed) : rng(seed) {}

    real4x4 normal();
    real4x4 uniform();
    real4x4 invertible();
};

class TestRngReal4x4 {
private:
    TestRng<real4x4> rng;

public:
    inline TestRngReal4x4() = default;
    inline explicit TestRngReal4x4(uint32_t seed) : rng(seed) {}

    real4x4 normal();
    real4x4 uniform();
};

#endif // UNIT_TEST
