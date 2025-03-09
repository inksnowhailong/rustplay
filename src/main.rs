use std::fmt;
use std::fmt::{write, Binary};

fn main() {
    // struct SomeStruct(i32);
    // impl fmt::Display for SomeStruct {

    //     fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result {
    //         println!("SomeStruct");
    //         write!(f,"{}",self.0)
    //     }
    // }
    // println!("{}",SomeStruct(3));

    #[derive(Debug)]
    struct NumberTuple(i32, i32);

    impl fmt::Display for NumberTuple {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({},{})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct TestDifference {
        x: f32,
        y: f32,
    }
    impl fmt::Display for TestDifference {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x:{}y:{}", self.x, self.y)
        }
    }

    let numbertuple = NumberTuple(10, 2);
    println!("Display{}", numbertuple);
    println!("Debug:{:?}", numbertuple);

    let testDiffrence = TestDifference { x: 1.0, y: 2.0 };
    println!("Display{}", testDiffrence);
    println!("Debug:{:?}", testDiffrence);

    impl Binary for NumberTuple {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{:b}", self.0)
        }
    }
    impl Binary for TestDifference {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "x={:b}.y={:b}", self.x.to_bits(), self.y.to_bits())
        }
    }
    println!(
        "what does my struct look like in binary?: {n:b},{t:b}",
        n = numbertuple,
        t = testDiffrence,
    );

    #[derive(Debug)]
    struct Wc {
        real: f32,
        imag: f32,
    };
    impl fmt::Display for Wc {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.2} + {:.2}i", self.real, self.imag)
        }
    }

    println!("Display: {}",Wc {real:3.3,imag:7.2});
    println!("Complex: {:?}",Wc {real:3.3,imag:7.2} );
}
