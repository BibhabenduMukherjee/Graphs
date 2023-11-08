#[cfg(target_os = "linux")] 
fn checking_on_linux() { 
    println!("Currently on linux!"); 
} 
  
#[cfg(not(target_os = "linux"))] 
fn checking_on_linux() { 
    println!("Not on linux!"); 
} 
  
fn main() { 
   checking_on_linux(); 
    if cfg!(target_os = "linux") { 
        println!("Running on linux"); 
    } else { 
        println!("Not Running on linux"); 
    } 
} 