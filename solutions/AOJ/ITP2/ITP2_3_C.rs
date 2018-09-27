#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &Array::<isize>::new_from_iter((0..n).map(|_| read!()));
    let q: usize = read!();

    for _ in 0..q {
        let b: usize = read!();
        let e: usize = read!();
        let k: isize = read!();
        let slice = &list::slice(a, b..e);
        let it = list::iter(slice);

        writelnf!("{:d}", it.filter(|&x| x == k).count())
    }
}
