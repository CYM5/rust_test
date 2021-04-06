use std::collections::HashMap;
use std::f64;
fn found_div(div:&mut HashMap<i32, i32>, i:i32){
    let square_i = (i as f64).sqrt() as i32;
    for j in 2..square_i {
        if i % j == 0 {
            if div.contains_key(&j) { 
                div.insert(j, 1+div[&j]);
            }
            else{
                div.insert(j, 1);
            }
        }
    }
}

fn main() {
    let mut found = HashMap::new();
    let code = String::from("KQOWEFVJPUJUUNUKGLMEKJINMWUXFQMKJBGWRLFNFGHUDWUUMBSVLPSNCMUEKQCTESWREEKOYSSIWCTUAXYOTAPXPLWPNTCGOJBGFQHTDWXIZAYGFFNSXCSEYNCTSSPNTUJNYTGGWZGRWUUNEJUUQEAPYMEKQHUIDUXFPGUYTSMTFFSHNUOCZGMRUWEYTRGKMEEDCTVRECFBDJQCUSWVBPNLGOYLSKMTEFVJJTWWMFMWPNMEMTMHRSPXFSSKFFSTNUOCZGMDOEOYEEKCPJRGPMURSKHFRSEIUEVGOYCWXIZAYGOSAANYDOEOYJLWUNHAMEBFELXYVLWNOJNSIOFRWUCCESWKVIDGMUCGOCRUWGNMAAFFVNSIUDEKQHCEUCPFCMPVSUDGAVEMNYMAMVLFMAOYFNTQCUAFVFJNXKLNEIWCWODCCULWRIFTWGMUSWOVMATNYBUHTCOCWFYTNMGYTQMKBBNLGFBTWOJFTWGNTEJKNEEDCLDHWTVBUVGFBIJG");
    for i in 5 .. 8 {
        for j in 0..code.len(){
            let substring:&str ;
            if j+i <= code.len() {
                substring = &code[j..j+i];
            }
            else {
                break;
            }
            let test = code.find(&substring);
            match test {
                Some(test) => {
                    if test != j { //rep j - test
                        //println!("{}", &substring);
                        match found.get(&substring[0..substring.len()-1]){
                            Some(_) => { 
                                //let a = found.remove(&substring[0..substring.len()-1]);
                                found.insert(substring, j-test);
                            },
                            None => {
                                found.insert(substring, j-test);
                            }
                        }
                    } 
                },
                None => {}
            }
        }
        
    }
    let mut div: HashMap<i32,i32> = HashMap::new();
    for (key, value) in found.iter() {
        println!("{} - {}", key, value);
        found_div(&mut div, *value as i32);
    }
    let mut key_size:i32 = 0;
    let mut max_size:i32=0;
    for (key, value) in div.iter() {
        println!("{} - {}", key, value);
        if value > &max_size {
            max_size=*value;
            key_size = *key;
        }
    }

    println!("Key size should be -> {}", key_size);
}
