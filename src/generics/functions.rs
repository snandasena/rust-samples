struct A;

struct S(A);

struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}


fn generic<T>(_s: SGen<T>) {}

pub fn test_generic_functions()
{
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(9));

    generic::<char>(SGen('a'));

    generic(SGen('a'));
}