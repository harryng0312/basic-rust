use bigdecimal::BigDecimal;
use chrono::{Local, NaiveDateTime};
use num::Zero;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Debug, str::FromStr};

use utils::format::bigdecimal_format;
use utils::format::date_format;

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    #[serde(rename = "id")]
    id: u64,
    // #[serde(rename = "orderDate", serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    #[serde(rename = "orderDate", with = "date_format")]
    order_date: NaiveDateTime,
    #[serde(rename = "totalAmount", with = "bigdecimal_format")]
    total_amount: BigDecimal,
    #[serde(rename = "orderDetails")]
    order_details: Vec<OrderDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderDetail {
    #[serde(rename = "id")]
    id: u64,
    #[serde(rename = "productId")]
    product_id: String,
    quantity: u32,
    #[serde(with = "bigdecimal_format")]
    price: BigDecimal,
    #[serde(rename = "order_id")]
    order_id: u64,
}

#[test]
fn test_json_struct() {
    let mut order = Order {
        id: 1,
        order_date: Local::now().naive_local(),
        total_amount: BigDecimal::zero(),
        order_details: Vec::<OrderDetail>::new(),
    };
    let order_detail = OrderDetail {
        id: 1,
        product_id: "".to_string(),
        quantity: 2,
        price: Default::default(),
        order_id: 1,
    };
    order.order_details.push(order_detail);
    // order.order_details.borrow_mut().push(order_detail);
    let json = serde_json::to_string(&order).unwrap_or("{}".to_string());
    println!("Json:{}", json);
    let obj: Order = serde_json::from_str(&json).unwrap();
    println!("Object:{:?}", obj);

    let od = &mut order.order_details[0];
    od.price = BigDecimal::from_str("3.141592").unwrap();
    println!("1st obj:{:?}", od);
}
