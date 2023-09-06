use lmvs_rust::root::lmvs::math::*;


fn main() {

    let sum = unsafe{
        //lmvs_rust::add(55.5, 5566.5)
        add(1.0, 1.0)
    };
    println!("lmvs_rust::sum: {}", sum);


    let r = unsafe{
        div(55.5, 5566.5)
    };
    println!("lmvs_rust::div: {}", r);

}
