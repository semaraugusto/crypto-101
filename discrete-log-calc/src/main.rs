use num_bigint::BigUint;
use num_bigint::ToBigUint;
use std::collections::HashMap;

#[derive(Debug)]
struct FieldParams {
    p: BigUint,
    g: BigUint,
    h: BigUint,
}
#[derive(Debug)]
struct MeetInTheMiddle {
    fp: FieldParams,
    table: HashMap<BigUint, u32>,
    size: u32,
}

impl FieldParams {
    fn new() -> FieldParams {
        let p = BigUint::parse_bytes(
            b"1340780792994259709957402499820584612747\
                                       9365820592393377723561443721764030073546\
                                       9768018742981669034276900318581864860508\
                                       53753882811946569946433649006084171",
            10,
        )
        .unwrap();

        let g = BigUint::parse_bytes(
            b"1171782988036620700951611759633536708855\
                                       8084999998952205599979459063929499736583\
                                       7466705721764714603129285948296754282794\
                                       66566527115212748467589894601965568",
            10,
        )
        .unwrap();

        let h = BigUint::parse_bytes(
            b"3239475104050450443565264378728065788649\
                                       0975209524495278347924529719819761432925\
                                       5807385693795855318053287892800149470609\
                                       7394108577585732452307673444020333",
            10,
        )
        .unwrap();

        FieldParams { p, g, h }
    }
}

impl MeetInTheMiddle {
    fn new() -> MeetInTheMiddle {
        let mut table: HashMap<BigUint, u32> = HashMap::with_capacity(2usize.pow(20));

        let fp = FieldParams::new();
        let inv_coef = &fp.p - BigUint::parse_bytes(b"2", 10).unwrap();
        // let mut curr_g_inv = BigUint::parse_bytes(b"1", 10).unwrap();
        let g_inv = fp.g.modpow(&inv_coef, &fp.p);

        // let two = BigUint::new(vec![2]);
        // let g_inv = fp.g.modpow(&(&fp.p - &two), &fp.p);

        let mut val = fp.h.clone();
        table.insert(val.clone(), 0);
        for idx in 1u32..2u32.pow(20) {
            // let val = (&fp.h * &curr_g_inv) % &fp.p;
            // println!("{}, {:?} table size: {:?}", idx, val, table.len());
            //
            val = (&val * &g_inv) % &fp.p;
            table.insert(val.clone(), idx);
            // if idx % 1_000 == 0 {
            //     println!("{}, table size: {:?}", idx, table.len());
            // }
        }

        MeetInTheMiddle {
            fp,
            table,
            size: 2u32.pow(20),
        }
    }

    fn attack(&self) -> Option<(u32, u32)> {
        let big_g = self
            .fp
            .g
            .modpow(&BigUint::new(vec![self.size as u32]), &self.fp.p);

        let mut val = BigUint::new(vec![1]);
        for idx in 0u32..self.size {
            if let Some(val) = self.table.get(&val) {
                return Some((idx, *val));
            }

            val = &val * &big_g % &self.fp.p;
            // if idx % 1_000 == 0 {
            //     println!("{}, table size: {:?}", idx, self.table.len());
            // }
        }

        None
    }
}

fn main() {
    println!("Meet-in-the-Middle Attack (MITM)");
    let mitm = MeetInTheMiddle::new();
    // println!("{:?}", mitm);
    let attack = mitm.attack().unwrap();
    println!("{:?}", attack);
}
