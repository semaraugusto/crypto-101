#![allow(dead_code)]
use num_bigint::BigInt;
use num_traits::{One, Zero};

fn challenge1(n: &BigInt) -> (BigInt, BigInt) {
    let one: BigInt = One::one();
    let a: BigInt = n.sqrt() + one;
    let x = (&a * &a - n).sqrt();
    let p = &a - &x;
    let q = a + x;
    (p, q)
}

fn challenge2(n: BigInt) -> (BigInt, BigInt) {
    let one: BigInt = One::one();
    let a: BigInt = n.sqrt() + one;

    for i in 0..100_000_000u64 {
        let i: BigInt = i.into();
        let avg = &a + i;
        let x = (&avg * &avg - &n).sqrt();
        let p = &avg - &x;
        let q = avg + x;
        if &p * &q == n {
            return (p, q);
        }
    }
    unreachable!()
}
fn challenge3(n: BigInt) -> (BigInt, BigInt) {
    let one: BigInt = One::one();
    let b: BigInt = (BigInt::from(24u32) * &n).sqrt() + &one;
    let y = (&b * &b - BigInt::from(24u32) * &n).sqrt();
    let p = (&b + &y) / BigInt::from(4u32);
    let q = (&b - &y) / BigInt::from(6u32);
    if &p * &q == n {
        // Print the smaller factor
        println!("3. Prime: {:?}", &p.clone().min(q.clone()).to_string());
    }
    (p, q)
}
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), Zero::zero(), One::one())
    } else {
        let (g, s, t) = extended_gcd(&(b % a), a);
        (g, t - (b / a) * &s, s)
    }
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (gcd, s, _) = extended_gcd(a, m);
    if gcd == One::one() {
        return Some((s % m + m) % m);
    }
    None
}

fn decrypt(cipher: BigInt, n: BigInt) -> Vec<u8> {
    let one: BigInt = One::one();
    let (p, q) = challenge1(&n);
    let phi_n = (&p - &one) * (&q - &one);
    println!("phi_n: {:?}", phi_n);
    let e = BigInt::from(65_537u32);
    // let d = e.modpow(&cipher, &phi_n);
    let d = mod_inverse(&e, &phi_n).unwrap();
    let msg = cipher.modpow(&d, &n);
    let (_, msg_bytes) = msg.to_bytes_be();
    msg_bytes
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge1() {
        let n: BigInt = BigInt::parse_bytes(
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
        let (p, q) = challenge1(&n);
        println!("p: {:?}, q: {:?}", p.to_string(), q.to_string());

        assert_eq!(p, BigInt::parse_bytes(b"13407807929942597099574024998205846127479365820592393377723561443721764030073662768891111614362326998675040546094339320838419523375986027530441562135724301", 10).unwrap());
    }
    #[test]
    fn test_challenge2() {
        let n: BigInt = BigInt::parse_bytes(
            b"6484558428080716696628242653467722787263437207069762630604390703787\
9730861808111646271401527606141756919558732184025452065542490671989\
2428844841839353281972988531310511738648965962582821502504990264452\
1008852816733037111422964210278402893076574586452336833570778346897\
15838646088239640236866252211790085787877",
            10,
        )
        .unwrap();
        let (p, q) = challenge2(n);
        println!("p: {:?}, q: {:?}", p.to_string(), q.to_string());

        assert_eq!(p, BigInt::parse_bytes(b"25464796146996183438008816563973942229341454268524157846328581927885777969985222835143851073249573454107384461557193173304497244814071505790566593206419759", 10).unwrap());
    }

    #[test]
    fn test_challenge3() {
        let n: BigInt = BigInt::parse_bytes(
            b"72006226374735042527956443552558373833808445147399984182665305798191\
63556901883377904234086641876639384851752649940178970835240791356868\
77441155132015188279331812309091996246361896836573643119174094961348\
52463970788523879939683923036467667022162701835329944324119217381272\
9276147530748597302192751375739387929",
            10,
        )
        .unwrap();
        let (p, q) = challenge3(n);
        let min = p.clone().min(q.clone());
        println!("p: {:?}, q: {:?}", p.to_string(), q.to_string());

        assert_eq!(min, BigInt::parse_bytes(b"21909849592475533092273988531583955898982176093344929030099423584127212078126150044721102570957812665127475051465088833555993294644190955293613411658629209", 10).unwrap());
    }
    #[test]
    fn test_challenge4() {
        let cipher: BigInt = BigInt::parse_bytes(
            b"22096451867410381776306561134883418017410069787892831071731839143676135600120538004282329650473509424343946219751512256465839967942889460764542040581564748988013734864120452325229320176487916666402997509188729971690526083222067771600019329260870009579993724077458967773697817571267229951148662959627934791540",
            10,
        )
        .unwrap();

        let n: BigInt = BigInt::parse_bytes(
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

        let result = decrypt(cipher, n);
        let msg: Vec<u8>;
        if result[0] == 2 {
            msg = result.into_iter().skip_while(|&b| b != 0).skip(1).collect();
            println!("res: {:?}", String::from_utf8_lossy(&msg));
            assert_eq!(
                "Factoring lets us break RSA.",
                String::from_utf8_lossy(&msg)
            )
        }
    }
}
