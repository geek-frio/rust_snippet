struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self)->bool {
        self.width == self.height
    }
}

pub fn run() -> i32 {
    println!("outer method!");
    return 1;
}

// 表示只有当我们运行test用例的时候才会编译这个mod里的内容
#[cfg(test)]
mod dcode_tests{
    // should panic表示这个方法本来就应该panic,是预计的情况
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1);
        panic!("Oh no!");
    }

    #[test]
    fn test_equals() {
        assert_eq!(2,1+1);
    }

    #[test]
    fn test_not_equals(){
        assert_eq!(2, 2);
    }

    // 如果一个test花费太长时间,我们不希望其执行就可以这样使用
    #[test]
    #[ignore]
    fn test_ignore(){
        assert_eq!(2, 1+1);
        assert_ne!(2, 1+2);
    }

    // 调用外部的方法
    #[test]
    fn test_use_outer_fn(){
        // run 方法相对于这个mod是外部的方法,所以使用super
        assert_eq!(1, super::run());
    }

    #[test]
    #[should_panic]
    fn test_structs(){
        let r = super::Rectangle{
            width: 50,
            height: 25
        };
        assert!(r.is_square());
    }
}