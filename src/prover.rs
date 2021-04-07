// base trait for the natural numbers
trait Nat {}

// first Peano axiom: 0∈ℕ
struct Zero;
impl Nat for Zero {}

// second Peano axiom: ∀n(n∈ℕ ⇒ n'∈ℕ)
struct Succ<P: Nat>(P);
impl<P: Nat> Nat for Succ<P> {}

// some shortcuts for the first natural numbers
type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;
type Four = Succ<Three>;
type Five = Succ<Four>;

// trait to describe result type of an operation
trait TypeOp {
    type Result;
}

// helper to describe rules of adding two natural numbers
struct AddHelper<A: Nat, B: Nat>(A, B);

// shortcut for the result of adding two Nat types
type Add<A, B> = <AddHelper<A, B> as TypeOp>::Result;

// a + 0 = a
impl<A: Nat> TypeOp for AddHelper<A, Zero> {
    type Result = A;
}

// a + b' = a' + b
impl<A: Nat, B: Nat> TypeOp for AddHelper<A, Succ<B>>
where
    AddHelper<Succ<A>, B>: TypeOp,
{
    type Result = Add<Succ<A>, B>;
}

const QED1: Option<Add<Two, Three>> = Option::<Five>::None;
const QED2: Option<Add<One, Add<Two, One>>> = Option::<Four>::None;
const QED3: Option<Add<One, Zero>> = Option::<Add<Zero, One>>::None;

// fn prove_commutative_with_zero<A: Nat>() -> Option<Add<Succ<A>, Zero>>
// where
//      AddHelper<Zero, Succ<A>>: TypeOp,
//      //AddHelper<Succ<A>, Zero>: TypeOp,
// {
//     Option::<Add<Zero, Succ<A>>>::None
// }

struct MulHelper<A: Nat, B: Nat>(A, B);

// shortcut for the result of multiplying two Nat types
type Mul<A, B> = <MulHelper<A, B> as TypeOp>::Result;

// a * 0 = 0
impl<A: Nat> TypeOp for MulHelper<A, Zero> {
    type Result = Zero;
}


// a * (b + 1) = a + a * b
// impl<A: Nat, B: Nat> TypeOp for MulHelper<A, Succ<B>>
// where
//     MulHelper<A, B>: TypeOp,
//     AddHelper<A, MulHelper<A, B>>: TypeOp,
// {
//     type Result = Add<A, Mul<A, B>>;
// }
