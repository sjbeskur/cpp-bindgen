
use lmvs_rust::root::lmvs::math::*;

#[test]
fn test_add() {

    let sum = unsafe{
        //lmvs_rust::add(55.5, 5566.5)
        add(1.0, 1.0)
    };
    assert_eq!(sum,2.0);

    let r = unsafe{
        div(55.5, 5566.5)
    };
    println!("lmvs_rust::div: {}", r);

}


#[test]
fn test_div() {

    let r = unsafe{
        div(100.0, 10.0)
    };
    assert_eq!(r,10.0);

}
