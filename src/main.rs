fn main() {
    let registers: [u8; 16] = [0; 16];

    let gfx: [u8; 64 * 32] = [0; 64 * 32]; //64 * 32
    let delay_timer: u16 = 0;
    let sound_timet: u16 = 0;

    let stack: [u16; 16] = [0; 16]; // 16 levels of stack
    let keys: [bool; 16] = [false; 16]; // 16 bits for state of key

    while(true){

    }

    ()
}

fn initialize(){
    let opcode: u16 = 0;
    let index: u16 = 0;
    let pc: u16 = 0x200;
    let sp: u16 = 0;

    let mut memory: [u8; 4096] = [0; 4096];

    for i in 0..79 {
        memory[i] = 1;
        //must load fontset later on
    }

    let buffer: [u8; 8192] = [1; 8192];

    for j in 0..buffer.len() {
        memory[j + 0x200] = buffer[j]
    }
}


fn setup_graphics(){

}

fn setup_input(){

}

