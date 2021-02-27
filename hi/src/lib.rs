#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub fn say() {
    println!("Hi!");

    #[cfg(feature = "gpu")]
    println!("Hi! feature: gpu");


    #[cfg(feature = "opencl")]
    println!("Hi! feature: opencl");
}
