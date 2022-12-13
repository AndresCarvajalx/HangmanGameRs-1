use std::{io::{stdin, Write}, time::Duration};


// Juego basico de ahorcado, por consola, proximamente con interfaz grafica y mas funciones

fn main(){
    let intro:String = String::from("El juego del ahorcado, por consola, by Andres Carvajal \n\n");
    for letras in intro.chars(){
        print!("{}", letras);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }

    //Creamos las variables en las que se guardaran las palabras y se sumaran los intentos
    let mut palabra_adivinar: String = String::new();
    let mut palabra_usuario: String = String::new(); 
    let mut pista:String = String::new();  
    let mut intentos: i32 = 6; // Mutable porque se le restara 1 cada vez que no adivine la palabra
    let mut ayuda: i8 = 1;
    println!("Voltea la pantalla que no vean la palabra a adivinar \n");
    println!("Ingresa la palabra que tendran que adivinar: ");

    /*  
        Leemos la palabra a adivinar y ademas le quitamos los espacios en blanco que tenga al inicio o
        final de la palabra con .trim() luego la convertimos a String con .to_string(), por ejemplo " Hola  ", se convertira en "Hola"
        y .to_lowercase() convierte la palabra a minusculas
    */
    stdin().read_line(&mut palabra_adivinar).expect("Error al leer la palabra");
    palabra_adivinar = palabra_adivinar.trim().to_string().to_lowercase(); 
    
    while palabra_adivinar.is_empty() {
        println!("No has ingresado ninguna palabra, ingresa una palabra: ");
        stdin().read_line(&mut palabra_adivinar).expect("Error al leer la palabra");
        palabra_adivinar = palabra_adivinar.trim().to_string().to_lowercase();
    }
    
    //limpiamos la pantalla
    println!("Voltea la pantalla para empezar a jugar");
    std::thread::sleep(Duration::from_millis(1500));
    print!("{}[2J", 27 as char);

    // Pedimos el ingreso de la pista
    println!("Ingresa una pista para la palabra a adivinar: ");
    stdin().read_line(&mut pista).expect("Error al leer la palabra");

    for letras in intro.chars(){
        print!("{}", letras);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }

    // Convertimos la palabra a adivinar en un vector de caracteres, en el caso de "hola" seria ['h', 'o', 'l', 'a']
    let palabra_adivinar_list: Vec<char> = palabra_adivinar.chars().collect();
    
    // Comienzo del ciclo "infinito" loop que se rompera cuando se acaben los intentos o se adivine la palabra
    loop {
        println!("\nPista: {pista}\n");
        // let mut palabra_usuario: String = String::new();  <---| Tambien se puede declarar la variable aqui |---> Lo cual seria lo mejor
        palabra_usuario = String::new(); // Reseteamos la palabra del usuario   <---|nos ahorrariamos esta linea de codigo ↺ 
        println!("Trata de adivinar la palabra, tienes {} intentos :)", intentos);
        stdin().read_line(&mut palabra_usuario).expect("Error al leer la palabra");
        palabra_usuario = palabra_usuario.trim().to_string().to_lowercase();

        if palabra_usuario == palabra_adivinar {
            println!("Has ganado");
            break;
        }

        // comprobar si una letra de la palabra a adivinar esta en la palabra del usuario
        for letra in palabra_adivinar_list.iter(){
            if palabra_usuario.contains(*letra) {
                print!("{} ", letra);
            }else{
                print!("_ ");
            }
        }

        if intentos == 1 {
            println!("Has perdido, la palabra era: {}", palabra_adivinar);
            break;
        }

        intentos = intentos - 1;


        match intentos {
            3 =>  {
                println!("\n
                -----------------------------------------------------------
                | PISTA EXTRA: La primera letra es: {} y la ultima es: {} |
                -----------------------------------------------------------
                \n Ten en cuenta lo siguiente: {}", palabra_adivinar_list[0], palabra_adivinar_list[palabra_adivinar_list.len()-1], pista);
                continue;
                }

            1 => {
                println!("\n
                --------------------------------------------------------------------------
                | AYUDA: tienes un total de {} intentos, te dare uno mas :) ahora tienes {}|
                --------------------------------------------------------------------------
                \n", intentos, intentos + 1);
                ayuda = ayuda - 1;
                if ayuda == 0 {
                    intentos = intentos + 1;
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