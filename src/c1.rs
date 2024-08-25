/*
Convert hex to base64

hex: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d

base64: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing. 
*/

/*
hex = 4 bits

base64 = 6 bits

2 hex chars = 1 byte
4 b64 chars = 3 bytes


suppose 1234567 (same as 12345670)
is what we are trying to convert

12 34 56 70 00 00

last two bytes turn to ==

number of bytes divisible by

*/
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

fn unit_tests(){
    assert!(hex_to_dec('f')==15);
	assert!(hex_to_dec('a')==10);
	assert!(hex_to_dec('9')==9);
	assert!(hex_to_dec('0')==0);
	assert!(dec_to_binary(& 15_u8,4)==[true,true,true,true].to_vec());
	assert!(dec_to_binary(& 1_u8,4)==[false,false,false,true].to_vec());
	assert!(hex_to_binary(&mut "fff".to_string()) == [true;12].to_vec());
	assert!(hex_to_binary(&mut "000".to_string()) == [false;12].to_vec());
	assert!(hex_to_binary(&mut "f0".to_string()) == [true,true,true,true,false,false,false,false,false,false,false,false].to_vec());
	assert!(binary_to_base64_bytes(&[true,true,true,true,false,false])=='8' );
	assert!(binary_to_base64_bytes(&[false,false,false,false,false,false])=='A' );
	
	assert!(binary_to_base64([true,true,true,true,false,false,false,false,false,false,false,false].to_vec())=="8A" );

}

fn main(){
	unit_tests();
    //assert!(binary_print(&to_binary(&14,4)[..]) =="1111");
	//assert!(binary_print(&to_binary(&0,4)[..])=="0000");
	
	//assert!(hex_to_base64("12345670")=="EjRWcA")
	//assert!(hex_to_base64()==);

	let mut hex:String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

	let ans:String = hex_to_base64(&mut hex);
	println!("{}",ans);
	
	
	assert!(hex_to_base64(&mut hex)==base64);
    assert!(hex_to_base64(&mut "1234567".to_string())=="EjRWcA==");
}
fn hex_to_base64(hex:&mut String)->String{
	let mut num_char = hex.len();

	if num_char%2==1 {
		hex.push_str("0");
		num_char +=1;
	}
	let num_bytes = num_char/2;
	
	
	let bit_vec:Vec<bool> = hex_to_binary(hex);
	
    binary_print(&bit_vec[..]);

    let mut ans:String= binary_to_base64(bit_vec);
    let mut padding_count = 3 - (num_bytes%3);
    if padding_count != 3{
        ans.push_str(& ("=".repeat(padding_count)));
        
    }
    
    return ans;
}

fn _print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

fn hex_to_binary(hex: &mut String)-> Vec<bool>{
    let mut bit_vec:Vec<bool> = Vec::new();
	
    for c in hex.chars(){
        let mut a = dec_to_binary(&(hex_to_dec(c)),4);
        bit_vec.append(&mut a);
    }
    while bit_vec.len() % 6 != 0{
        bit_vec.push(false);
    }
    return bit_vec;
}

fn binary_to_base64(bit_vec:Vec<bool>) -> String{
    let six_ind_groups = bit_vec.len()/6;
    let mut ans:String = String::new();

    for i in 0..six_ind_groups{
        let temp = binary_to_base64_bytes(&bit_vec[i*6..i*6+6]);
        ans = ans+&temp.to_string();
    }
    return ans;
}
/*

&[u8] is a slice
&[u8;4] is a reference to an array

*/

fn hex_to_dec(c:char)->u8{
	let mut ans:u8 = 0;
	if c > '9'{
		ans = c as u8 -'a' as u8 +10;
	}else{
		ans = c as u8 -'0' as u8;
	}
	return ans;
}


fn binary_to_base64_bytes(bitarray:&[bool])->char{
    let mut ans: u8 = 0;
    for (i,b) in bitarray.iter().rev().enumerate(){
        ans += (*b as u8) * u8::pow(2,i as u32);
    }
    let base64_key = ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").as_bytes();
    
    return base64_key[ans as usize] as char;
}

fn binary_print(bitarray:&[bool]){
    let mut ans: String = String::new();
    for (i,b) in bitarray.iter().enumerate(){
        if i%6==0{
        	//println!();
        }
        //print!("{}",*b as u8);

    }
    //println!();
}

fn dec_to_binary(k: &u8, bits: usize)->Vec<bool>{
	let mut n: u8= *k;
	let mut ans:Vec<bool> = Vec::new();
	while n > 0{
		let rem = n % 2;
		ans.push(rem == 1);
		n = n/2;
	}

	//add 0 bits until full
	while ans.len() < bits{
	    ans.push(false);
	}
	let right_cutoff = ans.len()-bits;
	//reverse the bitarray and truncate extra bits
	return reverse_move(&ans)[right_cutoff..].to_vec();
}

fn reverse_move(arr: &Vec<bool>)->Vec<bool>{
	arr.iter().rev().copied().collect()
}
