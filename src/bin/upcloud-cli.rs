use std::env;
use std::error::Error;
use upcloud::UpcloudApi;

fn do_stuff(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let api = UpcloudApi::new(username, password);

    let account = api.get_account_info()?;
    println!("{:?}", account);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Call program with the following:");
        println!("{} UPCLOUD_USERNAME UPCLOUD_PASSWORD", args[0]);
        std::process::exit(1);
    }

    let result = do_stuff(&args[1], &args[2]);
    match result {
        Ok(_) => {
            println!("everything ok");
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
