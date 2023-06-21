use std::ops::Add;

use derive_new::new;
use rust_decimal::Decimal;

// 振る舞いを持つVO
// 具体的には通貨単位が一致した場合に限り加算が可能
//
// このケースでは通貨単位をフィールドの一部として定義している
// 通貨単位をフィールドではなく、型として表現するケースは`a5_vo_with_phantom`参照
#[derive(Clone, Debug, new, Eq, PartialEq)]
struct Money {
    amount: Decimal,
    currency: String,
}

// Add traitは「+」演算子による加算を表現する
impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Self::Output {
        // 通貨単位のチェック
        // 通貨単位が一致しない場合はpanicを起こす
        // traitのシグネチャ上、Result型として返せないのでこれは仕方ないはず...
        // その意味で、通貨単位を型として表現することでコンパイル時に検査できる方が嬉しいと思われる
        if self.currency != other.currency {
            panic!("Invalid currency")
        }
        let new_amount = self.amount + other.amount;
        Money::new(new_amount, self.currency)
    }
}

/*
    こういうのはありなのかな？
    impl Add for Money {
    type Output = Option<Money>;

    fn add(self, other: Money) -> Self::Output {
        if self.currency != other.currency {
            return None;
        }
        let new_amount = self.amount + other.amount;
        Some(Money::new(new_amount, self.currency))
    }
}
*/

#[test]
fn _2_29() {
    let my_money = Money::new(Decimal::new(1000, 0), "JPY".to_string());
    let allowance = Money::new(Decimal::new(3000, 0), "JPY".to_string());

    let result = my_money + allowance;
    println!("{}", result.amount);
}
