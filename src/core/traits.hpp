#pragma once

#include <cstdlib>
#include <memory>



template <size_t P, typename ...Types>
struct NthType {};
template <size_t P, typename T, typename ...Types>
struct NthType<P, T, Types...> {
    typedef typename NthType<P - 1, Types...>::type type;
};
template <typename T, typename ...Types>
struct NthType<0, T, Types...> {
    typedef T type;
};
template <size_t P, typename ...Types>
using nth_type = typename NthType<P, Types...>::type;

template <bool ...Values>
struct Any {
    static const bool value = false;
};
template <bool X, bool ...Values>
struct Any<X, Values...> {
    static const bool value = X || Any<Values...>::value;
};
template <bool ...Values>
inline constexpr bool any_v = Any<Values...>::value;

template <bool ...Values>
struct All {
    static const bool value = true;
};
template <bool X, bool ...Values>
struct All<X, Values...> {
    static const bool value = X && All<Values...>::value;
};
template <bool ...Values>
inline constexpr bool all_v = All<Values...>::value;

template <typename T>
T copy(const T &x) {
    return T(x);
}
template <typename T>
void drop(T &x) {
    std::move(x);
}