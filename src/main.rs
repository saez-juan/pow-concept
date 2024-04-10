use std::env;
use std::time::Instant;

use hmac_sha512::Hash;

fn main() {
    // == Parsear argumentos

    let args: Vec<String> = env::args().collect();

    let message = args
        .get(1)
        .expect("el primer argumento debe ser el mensaje a encriptar");

    let zeros = args
        .get(2)
        .expect("el segundo parámetro debe ser la cantidad de ceros iniciales");

    let zeros: usize = zeros
        .parse()
        .expect("el segundo parámetro debe ser un número válido");

    // == Validar cantidad de ceros

    if zeros <= 0 || zeros >= 32 {
        panic!("cantidad de ceros inválida. fuera de límites");
    }

    // == Buscar nonce

    let mut nonce: u32 = 0;

    let perf = Instant::now();

    'main: loop {
        let fixed_msg = format!("{}:{}", message, nonce);
        let msg_hash = Hash::hash(&fixed_msg);

        // Comprobar que las posiciones iniciales sean 0.
        for i in 0..zeros {
            if msg_hash[i] != 0 {
                nonce += 1;
                continue 'main;
            }
        }

        let perf = perf.elapsed().as_millis();

        println!("\nCeros: {}", zeros);
        println!("Nonce: {}", nonce);

        println!("\nMensaje\n{}:{}", &message, nonce);
        println!("\nHex bytes\n{:02x?}\n", &msg_hash);
        // :02x -> Se traduce como: imprimí el hash en hexadecimal (x) y
        //         que cada byte ocupe 2 (02) caracteres.

        println!("Encontrado en {}ms\n", perf);

        break;
    }
}
