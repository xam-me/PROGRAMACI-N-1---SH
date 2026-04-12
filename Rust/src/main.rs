use std::io::{self, Write};

const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    fn new() -> Cadena {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    fn obt_longitud(&self) -> usize {
        self.longitud
    }

    fn add_char(&mut self, c: char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    fn obt_char(&self, posicion: usize) -> char {
        if posicion >= 1 && posicion <= self.longitud {
            self.caracteres[posicion - 1]
        } else {
        '\0'// caracter nulo para indicar error
        }
    }
   fn cant_apar_char(&self, c: char) -> u32 {
        let mut contador: u32 = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }
        contador
    }    
    fn car_max_repetido(&self) -> Option<char> {
        let mut max_char = '\0';
        let mut max_count = 0;
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            let count = self.cant_apar_char(c);
            if count > max_count {
                max_char = c;
                max_count = count;
            }
        }
        if max_count > 0 {
            Some (max_char)
    
        } else {
            None
        }
    }
    fn conv_min_may (&mut self) {
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            if c.is_ascii_lowercase() {
                self.caracteres[i] = c.to_ascii_uppercase();
            }
        }
    }
    fn inv_cadena(&mut self) {
        for i in 0..self.longitud / 2 {
            let temp = self.caracteres[i];
            self.caracteres[i] = self.caracteres[self.longitud - 1 - i];
            self.caracteres[self.longitud - 1 - i] = temp;
        }
    }
    fn palindrome(&self) -> bool {
        for i in 0..self.longitud / 2 {
            if self.caracteres[i] != self.caracteres[self.longitud - 1 - i] {
                return false;
            }
        }
         return true;
    } 
    // limpia la cadena para poder ingresar una nueva
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    // muestra la cadena completa carácter por carácter
    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
}

// ── helpers de entrada ──────────────────────────────────────────────
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

// ── menú ────────────────────────────────────────────────────────────
fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠══════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena        ║");
    println!("║  2. Mostrar cadena               ║");
    println!("║  3. Longitud                     ║");
    println!("║  4. Obtener carácter (posición)  ║");
    println!("║  5. Contar apariciones           ║");
    println!("║  6. Caracter mas repetido        ║");
    println!("║  7. Convertir a mayúsculas       ║");
    println!("║  8. Invertir cadena              ║");
    println!("╠══════════════════════════════════╣");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c = Cadena::new(); //definiendo la instancia de tipo Cadena

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada = leer_linea();

                c.limpiar(); // reiniciamos antes de cargar la nueva

                // ── proceso artesanal: carácter por carácter ──
                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obt_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obt_longitud()),

            "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obt_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obt_longitud()),
                    None    => println!("  Posición inválida."),
                }
            } 
            
             "5" => {
                println!("  Ingresa el carácter a buscar:");
                let entrada = leer_linea();
                match entrada.chars().next() {
                   Some(car) => {
                      let resultado:u32 = c.cant_apar_char(car);
                      if resultado > 1 {
                           println !(" '{}' aparece {} veces", car, resultado);
                           }else if resultado == 1 {
                              println!(" aparece 1 vez");
                               } else {
                                  println!(" '{}' no aparece en la cadena", car);
                               }
                    }
                       None => {
                          println!("  No ingresaste ningún carácter.")
                        }
                }
             } 
                "6" => {
                    match c.car_max_repetido() {
                        Some(car) => println!(" El caracter mas repetido es '{}' ", car),
                        None => println! (" No hay caracteres en la cadena.")
                    }
                }
                "7" => {
                    c.conv_min_may();
                    println!(" Cadena convertida a mayusculas.");
                }
                "8" => {
                    c.inv_cadena();
                    println!(" Cadena invertida.");
                }
                "9" => {
                    if c.palindrome() {
                        println!(" La cadena es un palíndromo.");
                    } else {
                        println!(" La cadena no es un palíndromo.");
                    }
                }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}