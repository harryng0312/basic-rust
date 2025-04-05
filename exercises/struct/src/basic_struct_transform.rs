use std::cell::RefCell;
use std::fmt::Debug;
// use std::rc::{Rc, Weak};
use chrono::NaiveDateTime;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Weak;

use utils::format::date_format;

// fn serialize_naive_datetime<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let s = dt.to_string(); // Serialize thành một chuỗi
//     serializer.serialize_str(&s)
// }
// fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
// }

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    // #[serde(rename = "orderDate", serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    #[serde(rename = "orderDate", with = "date_format")]
    order_date: NaiveDateTime,
    #[serde(rename = "totalAmount")]
    total_amount: Decimal,
    #[serde(rename = "orderDetails")]
    order_details: RefCell<Vec<OrderDetail>>,
    // order_details: Vec<OrderDetail>,
}
// impl<T: Debug> Debug for OrderDetail {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "OrderDetail{{data:{:?}}}", self)
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
struct OrderDetail {
    #[serde(rename = "productId")]
    product_id: String,
    quantity: u32,
    price: Decimal,
    #[serde(skip)]
    order: Weak<Order>,
}

#[cfg(test)]
mod tests {
    use crate::basic_struct_transform::{Order, OrderDetail};
    use chrono::Local;
    use rust_decimal::prelude::Zero;
    use rust_decimal::Decimal;
    use std::cell::RefCell;
    use std::sync::Arc;

    #[test]
    fn test_json_struct() {
        let /*mut*/ order = Arc::new(Order {
        order_date: Local::now().naive_local(),
        total_amount: Decimal::zero(),
        order_details: RefCell::new(vec![]),
    });
        let order_detail = OrderDetail {
            product_id: "".to_string(),
            quantity: 2,
            price: Default::default(),
            order: Arc::downgrade(&order),
        };
        order.order_details.borrow_mut().push(order_detail);
        // order.order_details.borrow_mut().push(order_detail);
        let json = serde_json::to_string(&(*order)).unwrap();
        println!("Json:{}", json);
        let obj: Order = serde_json::from_str(&json).unwrap();
        println!("Object:{:?}", obj);
        let od = &mut order.order_details.borrow_mut()[0];
        od.price = Decimal::new(3141, 3);
        println!("1st obj:{:?}", od);
    }
}
