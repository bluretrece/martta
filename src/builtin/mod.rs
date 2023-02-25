#![allow(dead_code)]
use crate::value::*;

pub(crate) fn std_print(
    vals: Vec<crate::value::Value>,
) -> Result<crate::value::Value, crate::error::Error> {
    println!("{:?}", &vals);

    Ok(vals[0].clone())
}

// #![allow(dead_code)]
pub(crate) fn reduce(
    elems: Vec<crate::value::Value>,
) -> Result<crate::value::Value, crate::error::Error> {
    let el = match elems[0].clone() {
        Value::List(els) => els.into_iter().reduce(|a, b| a + b),
        _ => unreachable!(),
    };

    Ok(el.unwrap())
}
