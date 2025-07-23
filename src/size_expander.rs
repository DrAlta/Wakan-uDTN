/// this seems to work thou need to put the test into actual unit tests
#[allow(dead_code)]
pub fn size_expander(v:u8) -> u16 {
    let u = v as u16;
    if u < 92 {
        u + 21
    } else if u < 140 {
        (u * 2) - 70
    } else if u < 172 {
        (u * 4) - 348
    } else if u < 188 {
        (u * 8) - 1032
    } else if u < 207 {
        (u * 16) - 2528
    } else if u < 225 {
        (u * 32) - 5824
    } else if u < 244 {
        (u * 64) - 12992
    } else {
        (u * 128) - 28544
    }
}
#[cfg(test)]
mod test_u8 {
    use super::*;
    #[test]
    fn max(){
        assert_eq!(
            4096,
            size_expander(u8::MAX)
        );
    }

    #[test]
    fn t_64_to_128(){ 
        let e = 2432;
        let t = 241;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 64,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 64 + 64,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 64 + 64 + 128,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 64 + 64 + 128 + 128,
            size_expander(t + 4)
        );
    }


    #[test]
    fn t_32_to_64(){
        let e = 1280;
        let t = 222;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 32,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 32 + 32,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 32 + 32 + 64,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 32 + 32 + 64 + 64,
            size_expander(t + 4)
        );

    }

    #[test]
    fn t_16_to_32(){
        let e = 736;
        let t = 204;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 16,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 16 + 16,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 16 + 16 + 32,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 16 + 16 + 32 + 32,
            size_expander(t + 4)
        );
    }
    #[test]
    fn t_8_to_16(){
        let e = 448;
        let t = 185;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 8,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 8 + 8,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 8 + 8 + 16,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 8 + 8 + 16+ 16,
            size_expander(t + 4)
        );
    }
    #[test]
    fn t_4_to_8(){
        let e = 328;
        let t = 169;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 4,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 4 + 4,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 4 + 4 + 8,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 4 + 4 + 8 + 8,
            size_expander(t + 4)
        );
    }
    #[test]
    fn t_2_to_4(){
        let e = 204;
        let t = 137;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 2,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 2 + 2,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 2 + 2 + 4,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 2 + 2 + 4 + 4,
            size_expander(t + 4)
        );
    }
    #[test]
    fn t_1_to_2(){
        let e = 110;
        let t = 89;
        assert_eq!(
            e,
            size_expander(t)
        );
        assert_eq!(
            e + 1,
            size_expander(t + 1)
        );
        assert_eq!(
            e + 1 + 1,
            size_expander(t + 2)
        );
        assert_eq!(
            e + 1 + 1 + 2,
            size_expander(t + 3)
        );
        assert_eq!(
            e + 1 + 1 + 2 + 2,
            size_expander(t +4 )
        );
                
    }
}
///////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn size_expander_u16(v:u16)->u32 {
    let u = v as u32;
    if u < 2_u32.pow(12)-1 {
        u
    } else if u < 2_u32.pow(13)-1 {
        (u * 2) - (2_u32.pow(12)-2)
    } else if u < 2_u32.pow(14)-1{
        (u * 4) - 20474
    } else if u < 2_u32.pow(15)-1{
        (u * 8) - 86002
    } else {
        (u * 16) -348130
    }
}
#[cfg(test)]
mod test_u16 {
    use super::*;

#[test]
fn test_1_to_2(){
    let t = 4092;
    let e = 4092;
    assert_eq!(
    e,
    size_expander_u16(t)
    );
    assert_eq!(
    e+1,
    size_expander_u16(t+1)
    );
    assert_eq!(
    e+1+1,
    size_expander_u16(t+2)
    );
    assert_eq!(
    e+1+1+2,
    size_expander_u16(t+3)
    );
    assert_eq!(
    e+1+1+2+2,
    size_expander_u16(t+4)
    );
}

#[test]
fn test_2_to_4(){
    let t = 8188;
    let e = 12282;
    assert_eq!(
    e,
    size_expander_u16(t)
    );
    assert_eq!(
    e+2,
    size_expander_u16(t+1)
    );
    assert_eq!(
    e+2+2,
    size_expander_u16(t+2)
    );
    assert_eq!(
    e+2+2+4,
    size_expander_u16(t+3)
    );
    assert_eq!(
    e+2+2+4+4,
    size_expander_u16(t+4)
    );
}
#[test]
fn test_4_to_8(){
    let t = 16380;
    let e = 45046;
    assert_eq!(
    e,
    size_expander_u16(t)
    );
    assert_eq!(
    e+4,
    size_expander_u16(t+1)
    );
    assert_eq!(
    e+4+4,
    size_expander_u16(t+2)
    );
    assert_eq!(
    e+4+4+8,
    size_expander_u16(t+3)
    );
    assert_eq!(
    e+4+4+8+8,
    size_expander_u16(t+4)
    );
}
#[test]
fn test_8_to_16(){
    let t = 32764;
    let e = 176110;
    assert_eq!(
    e,
    size_expander_u16(t)
    );
    assert_eq!(
    e+8,
    size_expander_u16(t+1)
    );
    assert_eq!(
    e+8+8,
    size_expander_u16(t+2)
    );
    assert_eq!(
    e+8+8+16,
    size_expander_u16(t+3)
    );
    assert_eq!(
    e+8+8+16+16,
    size_expander_u16(t+4)
    );
}
}

