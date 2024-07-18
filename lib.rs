use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]

) -> ProgramResult {
    // Generating multiplicative group mod p (G)
    let p=23; // prime number
    let q: i32=(p-1)/2; // order of group G
    let r: i32=2; // factor (q*r)=p
    let h: i32=7; // any integer number
    let g: i32; // generator of G
    let mut temp: i32;
    let result="";

    g = (h^r) % p;

    for i in 1..=q {
        temp = (g^i) % p; 
        msg!(&(temp.to_string())); // writing to log G set elements
    }  
       
    // gracefully exit the program
    Ok(())
}