
fn test_result_enum(tf: u8) -> Result<bool, std::io::ErrorKind> {
    if tf == 0 {
        return Ok(true);
    } else if tf == 1 {
        return Ok(false);
    } 
    return Err(std::io::ErrorKind::NotFound);
}
fn test_std_result(tf: u8) -> Result<i32, String> {
    match tf {
        0 => return Ok(0),
        1 => return Ok(100),
        2 => return Ok(-100),
        _ => return Err("General error message".to_string())
    };
}
fn main() {

    match test_result_enum(2) {
        Ok(true) => println!("True"),
        Ok(false) => println!("False"),
        Err(ek) => match ek {
            std::io::ErrorKind::NotFound => println!("Not found"),
            _ => println!("Unknown error kind!")
        }
    };
    let mut result = test_std_result(0).unwrap();
    println!("result is {}", result);

    result = test_std_result(3).expect("Input GT 2");
    println!("result is {}", result);

    //let v = vec![1, 2, 3];
    //let v1 = v[99];

}
