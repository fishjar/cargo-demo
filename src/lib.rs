#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn say() {
    println!("Hello, world!");

    #[cfg(feature = "gpu")]
    println!("Hello, world! feature: gpu");


    #[cfg(feature = "opencl")]
    println!("Hello, world! feature: opencl");
}
