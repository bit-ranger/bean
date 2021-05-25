#[cfg(test)]
mod test{
    use bean::component::{HasComponent, Arc};

    struct A{}
    struct B{}

    bean::container!(Test {A, B});

    #[test]
    fn test(){
        Test::init()
            .put("a", Arc::new(A{}))
            .put("b", Arc::new(B{}));

        let _:Arc<A>  = Test::borrow().get("a").unwrap();
    }

}