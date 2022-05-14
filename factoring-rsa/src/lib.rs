#![allow(dead_code)]
use num_bigint::BigUint;
use num_traits::One;

fn challenge1(n: BigUint) -> (BigUint, BigUint) {
    let one: BigUint = One::one();
    let a: BigUint = n.sqrt() + one;
    let x = (&a * &a - &n).sqrt();
    let p = &a - &x;
    let q = a + x;
    (p, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge1() {
        let n: BigUint = BigUint::parse_bytes(
            b"1797693134862315907729305190789024733617\
                                   9769789423065727343008115773267580550562\
                                   0686985379449212982959585501387537164015\
                                   7101398586478337786069255834975410851965\
                                   9161512805757594075263500747593528871082\
                                   3649949940771895617054361149474865046711\
                                   0151015639406805275400715845608785776637\
                                   43040086340742855278549092581",
            10,
        )
        .unwrap();
        let (p, q) = challenge1(n);
        println!("p: {:?}, q: {:?}", p.to_string(), q.to_string());

        assert_eq!(p, BigUint::parse_bytes(b"13407807929942597099574024998205846127479365820592393377723561443721764030073662768891111614362326998675040546094339320838419523375986027530441562135724301", 10).unwrap());
    }
}
