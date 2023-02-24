pub(crate) fn std_print(
    vals: Vec<crate::value::Value>,
) -> Result<crate::value::Value, crate::error::Error> {
    println!("{:?}", &vals);

    Ok(vals[0].clone())
}
