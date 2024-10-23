
/*Example program
 *
 */
fn main() {
    //Question 6.8b
    let  mut a: u32 = 21;
    let  mut b: u32 = 15;
    println!("Eucleadian Algorithim Answer");
    println!("{}\n",  gcd(&mut a, &mut b).to_string()); 

    let  a: u32 = 21;
    let  b: u32 = 15;
    println!("Extended Eucleadian Algorithim Steps");
    let (gcd, n, m) = extended_gcd(a, b);
    println!("\ngcd({}, {}) = {}", a, b, gcd);
    println!("Integers = {}({}) + {}({}) = {}", n, a, m, b, a as i32 * n + b as i32 * m);
}

/*
 * Implementation of the Euclidean algorithim to find gcd(a,b)
 * workds for x,y <= 2^{32}-1
 */
fn gcd(x: &mut u32, y: &mut u32) -> u32 {
    //if one element is zero, return the other
    if *x==0{ return *y}
    if *y == 0 { return *x;}
    
    //ensure that y=max{x,y}
    if *x > *y {
        let temp = *x;
        *x = *y;
        *y = temp;
    }
    *y = *y % *x; //update y to be the remainder of y/x
    gcd(x, y) // rinse and repeat
}

/*
 *Extended Eucledian algorithim: finds 2 integers 𝑥
 *  and 𝑦 such that 𝑎𝑥+𝑏𝑦=gcd(𝑎,𝑏) when 𝐺𝐶𝐷(𝑎,𝑏)=1
 *  Note: This function prints all of the steps
*/
fn extended_gcd(a: u32, b: u32) -> (u32, i32,i32) {
    //Base case
    if a ==0 { 
    println!("gcd found");
    return (b,0,1)
    }
    
    //showing work for hw purposes
    println!("{} =\t {}({}) + {}", b, b/a, a , b%a);
    
    //recursive step(traversing down)
    let (gcd,x1,y1) = extended_gcd(b%a, a);
    
    
    //calculating n and m at each step back up
    let n = y1 - (b/a) as i32 * x1;
    let m = x1;
    
    //show work
    println!("{} =\t {}({}) + {}({})", gcd, n,a, m, b);
    return (gcd,n,m);
}
