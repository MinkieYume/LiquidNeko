fn main() {
    let mut env = Env::default();
    loop {
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0{
            println!("\nByeNyan");
            break;
        }
        let mut strs = reader::pre_read_str(&input,env.clone());
        let mut results:Vec<String> = Vec::new();
        //println!("{:?}",&strs);
        for st in strs {
            let s = rep(st.as_str(),env.clone());
            results.push(st);
            println!("{}",&s);
        }
	io::stdout().flush().unwrap();
    }
}
