
// fn change( w: &mut str) -> &mut str {
//     w = "aaskdfjaklsdjfskjfsdkjfasdkfj";
//    w 
// }

pub fn trial_serde() {

   // often confusing when we &str are immutable and we do the following
   let mut x = "ASDFASDFASDF";
   x = "DDD";
   assert_eq!("DDD", x);

   // howevever recall literals are pointers to strings like a slice  
   // *** we are only creating a mutable reference
   // it's difficult with &str (static known size at compile) and String (dynamic no known size)
   // also stack vs heap
   // the ptr, len, capacity are in the stack and value data is on the heap
   // so we can see here that s points to "xxxx..." 
   let mut s = "xxxxxxxxxxxxxxAsfaskdjfaksdfj";

   // s2 Points only s 
   let s2 = s;
   // then we point s to the new value 
   s = "asdkfjsdfj";

   // meanwhile we haven't changed the 
   println!("{} {}", s2, s);
}