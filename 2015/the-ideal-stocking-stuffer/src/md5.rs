const T: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const A: u32 = 0x67452301u32;
const B: u32 = 0xefcdab89u32;
const C: u32 = 0x98badcfeu32;
const D: u32 = 0x10325476u32;

pub fn hash(mut msg: Vec<u8>) -> u128 {
    let bitcount = msg.len().saturating_mul(8) as u64;
    msg.push(0b10000000);
    while (msg.len() * 8) % 512 != 448 {
        msg.push(0u8);
    }
    msg.extend(bitcount.to_le_bytes());

    let mut reg_a = A;
    let mut reg_b = B;
    let mut reg_c = C;
    let mut reg_d = D;

    let f = |x: u32, y: u32, z: u32| -> u32 { x & y | !x & z };
    let g = |x: u32, y: u32, z: u32| -> u32 { x & z | y & !z };
    let h = |x: u32, y: u32, z: u32| -> u32 { x ^ y ^ z };
    let i = |x: u32, y: u32, z: u32| -> u32 { y ^ (x | !z) };

    for block in msg.chunks_exact_mut(64) {
        let mut x = [0u32; 16];
        for j in 0..16 {
            x[j] = u32::from_le_bytes([
                block[j * 4 + 0],
                block[j * 4 + 1],
                block[j * 4 + 2],
                block[j * 4 + 3],
            ]);
        }

        let saved_reg_a = reg_a;
        let saved_reg_b = reg_b;
        let saved_reg_c = reg_c;
        let saved_reg_d = reg_d;

        macro_rules! op1 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(f($b, $c, $d))
                        .wrapping_add(x[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        op1!(reg_a, reg_b, reg_c, reg_d, 0, 7, 0);
        op1!(reg_d, reg_a, reg_b, reg_c, 1, 12, 1);
        op1!(reg_c, reg_d, reg_a, reg_b, 2, 17, 2);
        op1!(reg_b, reg_c, reg_d, reg_a, 3, 22, 3);

        op1!(reg_a, reg_b, reg_c, reg_d, 4, 7, 4);
        op1!(reg_d, reg_a, reg_b, reg_c, 5, 12, 5);
        op1!(reg_c, reg_d, reg_a, reg_b, 6, 17, 6);
        op1!(reg_b, reg_c, reg_d, reg_a, 7, 22, 7);

        op1!(reg_a, reg_b, reg_c, reg_d, 8, 7, 8);
        op1!(reg_d, reg_a, reg_b, reg_c, 9, 12, 9);
        op1!(reg_c, reg_d, reg_a, reg_b, 10, 17, 10);
        op1!(reg_b, reg_c, reg_d, reg_a, 11, 22, 11);

        op1!(reg_a, reg_b, reg_c, reg_d, 12, 7, 12);
        op1!(reg_d, reg_a, reg_b, reg_c, 13, 12, 13);
        op1!(reg_c, reg_d, reg_a, reg_b, 14, 17, 14);
        op1!(reg_b, reg_c, reg_d, reg_a, 15, 22, 15);

        macro_rules! op2 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(g($b, $c, $d))
                        .wrapping_add(x[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        op2!(reg_a, reg_b, reg_c, reg_d, 1, 5, 16);
        op2!(reg_d, reg_a, reg_b, reg_c, 6, 9, 17);
        op2!(reg_c, reg_d, reg_a, reg_b, 11, 14, 18);
        op2!(reg_b, reg_c, reg_d, reg_a, 0, 20, 19);

        op2!(reg_a, reg_b, reg_c, reg_d, 5, 5, 20);
        op2!(reg_d, reg_a, reg_b, reg_c, 10, 9, 21);
        op2!(reg_c, reg_d, reg_a, reg_b, 15, 14, 22);
        op2!(reg_b, reg_c, reg_d, reg_a, 4, 20, 23);

        op2!(reg_a, reg_b, reg_c, reg_d, 9, 5, 24);
        op2!(reg_d, reg_a, reg_b, reg_c, 14, 9, 25);
        op2!(reg_c, reg_d, reg_a, reg_b, 3, 14, 26);
        op2!(reg_b, reg_c, reg_d, reg_a, 8, 20, 27);

        op2!(reg_a, reg_b, reg_c, reg_d, 13, 5, 28);
        op2!(reg_d, reg_a, reg_b, reg_c, 2, 9, 29);
        op2!(reg_c, reg_d, reg_a, reg_b, 7, 14, 30);
        op2!(reg_b, reg_c, reg_d, reg_a, 12, 20, 31);

        macro_rules! op3 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(h($b, $c, $d))
                        .wrapping_add(x[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        op3!(reg_a, reg_b, reg_c, reg_d, 5, 4, 32);
        op3!(reg_d, reg_a, reg_b, reg_c, 8, 11, 33);
        op3!(reg_c, reg_d, reg_a, reg_b, 11, 16, 34);
        op3!(reg_b, reg_c, reg_d, reg_a, 14, 23, 35);

        op3!(reg_a, reg_b, reg_c, reg_d, 1, 4, 36);
        op3!(reg_d, reg_a, reg_b, reg_c, 4, 11, 37);
        op3!(reg_c, reg_d, reg_a, reg_b, 7, 16, 38);
        op3!(reg_b, reg_c, reg_d, reg_a, 10, 23, 39);

        op3!(reg_a, reg_b, reg_c, reg_d, 13, 4, 40);
        op3!(reg_d, reg_a, reg_b, reg_c, 0, 11, 41);
        op3!(reg_c, reg_d, reg_a, reg_b, 3, 16, 42);
        op3!(reg_b, reg_c, reg_d, reg_a, 6, 23, 43);

        op3!(reg_a, reg_b, reg_c, reg_d, 9, 4, 44);
        op3!(reg_d, reg_a, reg_b, reg_c, 12, 11, 45);
        op3!(reg_c, reg_d, reg_a, reg_b, 15, 16, 46);
        op3!(reg_b, reg_c, reg_d, reg_a, 2, 23, 47);

        macro_rules! op4 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(i($b, $c, $d))
                        .wrapping_add(x[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        op4!(reg_a, reg_b, reg_c, reg_d, 0, 6, 48);
        op4!(reg_d, reg_a, reg_b, reg_c, 7, 10, 49);
        op4!(reg_c, reg_d, reg_a, reg_b, 14, 15, 50);
        op4!(reg_b, reg_c, reg_d, reg_a, 5, 21, 51);

        op4!(reg_a, reg_b, reg_c, reg_d, 12, 6, 52);
        op4!(reg_d, reg_a, reg_b, reg_c, 3, 10, 53);
        op4!(reg_c, reg_d, reg_a, reg_b, 10, 15, 54);
        op4!(reg_b, reg_c, reg_d, reg_a, 1, 21, 55);

        op4!(reg_a, reg_b, reg_c, reg_d, 8, 6, 56);
        op4!(reg_d, reg_a, reg_b, reg_c, 15, 10, 57);
        op4!(reg_c, reg_d, reg_a, reg_b, 6, 15, 58);
        op4!(reg_b, reg_c, reg_d, reg_a, 13, 21, 59);

        op4!(reg_a, reg_b, reg_c, reg_d, 4, 6, 60);
        op4!(reg_d, reg_a, reg_b, reg_c, 11, 10, 61);
        op4!(reg_c, reg_d, reg_a, reg_b, 2, 15, 62);
        op4!(reg_b, reg_c, reg_d, reg_a, 9, 21, 63);

        reg_a = reg_a.wrapping_add(saved_reg_a);
        reg_b = reg_b.wrapping_add(saved_reg_b);
        reg_c = reg_c.wrapping_add(saved_reg_c);
        reg_d = reg_d.wrapping_add(saved_reg_d);
    }
    finalize(reg_a, reg_b, reg_c, reg_d)
}

fn md5_utf8(msg: &str) -> u128 {
    hash(msg.as_bytes().to_vec())
}

const fn finalize(a: u32, b: u32, c: u32, d: u32) -> u128 {
    ((a.swap_bytes() as u128) << 96)
        + ((b.swap_bytes() as u128) << 64)
        + ((c.swap_bytes() as u128) << 32)
        + (d.swap_bytes() as u128)
}

#[cfg(test)]
mod tests {
    use crate::md5::md5_utf8;

    #[test]
    fn empty_string_hash() {
        assert_eq!(md5_utf8(""), 0xd41d8cd98f00b204e9800998ecf8427e);
    }

    #[test]
    fn single_char_string_hash() {
        assert_eq!(md5_utf8("a"), 0x0cc175b9c0f1b6a831c399e269772661);
    }

    #[test]
    fn short_string_hash() {
        assert_eq!(md5_utf8("abc"), 0x900150983cd24fb0d6963f7d28e17f72);
    }

    #[test]
    fn sample_string_hash() {
        assert_eq!(
            md5_utf8("message digest"),
            0xf96b697d7cb7938d525a2f31aaf161d0
        );
    }

    #[test]
    fn abecedary_string_hash() {
        assert_eq!(
            md5_utf8("abcdefghijklmnopqrstuvwxyz"),
            0xc3fcd3d76192e4007dfb496cca67e13b
        );
    }

    #[test]
    fn uppercase_abecedary_string_hash() {
        assert_eq!(
            md5_utf8("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            0xd174ab98d277d9f5a5611c2c9f419d9f
        );
    }

    #[test]
    fn number_string_hash() {
        assert_eq!(
            md5_utf8(
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            ),
            0x57edf4a22be3c955ac49da2e2107b67a
        );
    }
}
