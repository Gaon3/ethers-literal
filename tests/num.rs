use ethers::core::types::*;
use ethers_literal::num;

#[test]
fn test() {
    num! {
        assert_eq!(0x10_U256, "0x10".parse::<U256>().unwrap());
        assert_eq!(10_U256, U256::from(10));

        assert_eq!(0x10U256, "0x10".parse::<U256>().unwrap());
        assert_eq!(10U256, U256::from(10));

        assert_eq!(-0x10_I256, I256::from(-0x10));
        assert_eq!(-10_I256, I256::from(-10));

        assert_eq!(0x10I256, I256::from(0x10));
        assert_eq!(10I256, I256::from(10));


        assert_eq!(0x10_U128, "0x10".parse::<U128>().unwrap());
        assert_eq!(10_U128, U128::from(10));

        assert_eq!(2, 2);
    }
}
