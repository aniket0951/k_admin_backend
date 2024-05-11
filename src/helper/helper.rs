use rand::RngCore;

pub struct Helper{}


impl Helper {
    pub fn generate_unique_number() -> u32 {
        let mut rng = rand::thread_rng();
        loop {
            
            let number: u32 = rng.next_u32();
            // Check if the number is unique
            if Self::is_unique(number) {
                return number;
            }
        }
    }
    
    pub fn is_unique(number: u32) -> bool {
        static mut USED_NUMBERS: Vec<u32> = Vec::new();
        unsafe {
            if USED_NUMBERS.contains(&number) {
                false
            } else {
                USED_NUMBERS.push(number);
                true
            }
        }
    }
}

