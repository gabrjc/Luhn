/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    //println!("Is the Luhn checksum for {} valid?", code);
    let mut res=0;
    let mut i =0;

if code.len()<=1 {return false; }
else {
    for c in code.chars().rev(){
        let n = match c {
            '0'..='9' if i%2==1 => {i+=1; c.to_digit(10).unwrap()*2 },
            '0'..='9' if i%2==0 => {i+=1; c.to_digit(10).unwrap()},
            ' '     => continue,
            _      => return false,

        };

       if n<10 {res+=n;}
        else{
            res=res+n-9;}

    }
    if res%10==0 && i>1 {return true;}
    else {return false;}
}
}
