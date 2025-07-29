#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

pub trait Cache<HtmlNode> {
    fn dirtied(&mut self, path: &[u64]);
    fn recompute(&mut self, root: &mut HtmlNode);
}
// RDTSC 时间测量工具
#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        _rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

/// whether a part of input is: 1, 0, or unused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IState {
    IOne,
    IZero,
    IUnused,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OState {
    OOne,
    OZero,
    OFromParent,
}

// lets generalize IState first - this is two separate but very similar optimization
// (also, you should tag our old commit before today's work, we want the old version to compare in benchmark)  // it's already in the commit.
// note that not all input state is used, some state are downright ignored.
// as an example, imagine we have a query A B, saying we should match a node satisfying predicate B,
// where parent satsify predicate A
// the code will look something like this:
// if (B(self)) {
//   if (parent_bitvector.A) {
//     self.out[AB] = 1;
//   }
// }
// in such case, you can see that we are not actually reading A, if branch is not entered
// so, suppose the parent A changed, we should do 0 work recomputing
// todo this, we have to update co/pute/ let me explain how this work with an example
//
// Export HtmlNode structure
