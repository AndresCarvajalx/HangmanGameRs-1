use std::{io::{stdin, Write}, time::Duration};

// Juego basico de ahorcado, por consola, proximamente con interfaz grafica y mas funciones

fn main(){
    let intro:String = String::from("El juego del ahorcado, por consola, by Andres Carvajal \n\n");
    for words in intro.chars(){
        print!("{}", words);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }

    //Creamos las variables en las que se guardaran las palabras y se sumaran los attemps
    let mut word_to_guess: String = String::new();
    let mut word_user: String = String::new(); 
    let mut clue:String = String::new();  
    let mut attemps: i32 = 6; // Mutable porque se le restara 1 cada vez que no adivine la palabra
    let mut help: i8 = 1;
    println!("Voltea la pantalla que no vean la palabra a adivinar \n");
    println!("Ingresa la palabra que tendran que adivinar: ");

    /*  
        Leemos la palabra a adivinar y ademas le quitamos los espacios en blanco que tenga al inicio o
        final de la palabra con .trim() luego la convertimos a String con .to_string(), por ejemplo " Hola  ", se convertira en "Hola"
        y .to_lowercase() convierte la palabra a minusculas
    */
    stdin().read_line(&mut word_to_guess).expect("Error al leer la palabra");
    word_to_guess = word_to_guess.trim().to_string().to_lowercase(); 
    
    while word_to_guess.is_empty() {
        println!("No has ingresado ninguna palabra, ingresa una palabra: ");
        stdin().read_line(&mut word_to_guess).expect("Error al leer la palabra");
        word_to_guess = word_to_guess.trim().to_string().to_lowercase();
    }
    
    //limpiamos la pantalla
    println!("Voltea la pantalla para empezar a jugar");
    std::thread::sleep(Duration::from_millis(1500));
    print!("{}[2J", 27 as char);

    // Pedimos el ingreso de la clue
    println!("Ingresa una clue para la palabra a adivinar: ");
    stdin().read_line(&mut clue).expect("Error al leer la palabra");

    for words in intro.chars(){
        print!("{}", words);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }

    // Convertimos la palabra a adivinar en un vector de caracteres, en el caso de "hola" seria ['h', 'o', 'l', 'a']
    let word_to_guess_list: Vec<char> = word_to_guess.chars().collect();
    
    // Comienzo del ciclo "infinito" loop que se rompera cuando se acaben los attemps o se adivine la palabra
    loop {
        println!("\nclue: {clue}\n");
        // let mut word_user: String = String::new();  <---| Tambien se puede declarar la variable aqui |---> Lo cual seria lo mejor
        word_user = String::new(); // Reseteamos la palabra del usuario   <---|nos ahorrariamos esta linea de codigo ↺ 
        println!("Trata de adivinar la palabra, tienes {} attemps :)", attemps);
        stdin().read_line(&mut word_user).expect("Error al leer la palabra");
        word_user = word_user.trim().to_string().to_lowercase();

        if word_user == word_to_guess {
            println!("Has ganado");
            break;
        }

        // comprobar si una letra de la palabra a adivinar esta en la palabra del usuario
        for letra in word_to_guess_list.iter(){
            if word_user.contains(*letra) {
                print!("{} ", letra);
            }else{
                print!("_ ");
            }
        }

        if attemps == 1 {
            println!("Has perdido, la palabra era: {}", word_to_guess);
            break;
        }

        attemps = attemps - 1;


        match attemps {
            3 =>  {
                println!("\n
                -----------------------------------------------------------
                | clue EXTRA: La primera letra es: {} y la ultima es: {} |
                -----------------------------------------------------------
                \n Ten en cuenta lo siguiente: {}", word_to_guess_list[0], word_to_guess_list[word_to_guess_list.len()-1], clue);
                continue;
                }

            1 => {
                println!("\n
                --------------------------------------------------------------------------
                | help: tienes un total de {} attemps, te dare uno mas :) ahora tienes {}|
                --------------------------------------------------------------------------
                \n", attemps, attemps + 1);
                help = help - 1;
                if help == 0 {
                    attemps = attemps + 1;
                }
                continue;
            }
            _ =>{
                continue;
            }         
        }
    }
    // hacer que no se cierre la consola
    println!("Presiona enter para salir");
    let mut salir = String::new();
    stdin().read_line(&mut salir).expect("Error al leer la palabra");
}