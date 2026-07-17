use std::io::{self, Write};

const N: usize = 100;
const VOCALES: &str = "aeiouAEIOUáéíóúÁÉÍÓÚüÜ";
const ALFABETO: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZñÑáéíóúÁÉÍÓÚüÜ";
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

    fn obt_longitud(&self) -> usize {  //opcion 3
        self.longitud
    }

    fn add_char(&mut self, c: char) {  
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    fn obt_char(&self, posicion: usize) -> char {  //opcion 4
        if posicion >= 1 && posicion <= self.longitud {
            self.caracteres[posicion - 1]
        } else {
        '\0'// caracter nulo para indicar error
        }
    }
   fn cant_apar_char(&self, c: char) -> u32 {  //opcion 5
        let mut contador: u32 = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }
        contador
    }    
    fn car_max_repetido(&self) -> Option<char> {  //opcion 6
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
    fn conv_min_may (&mut self) {  //opcion 7
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            if c >= 'a' && c <= 'z' {
                self.caracteres[i] = ((c as u8) - 32) as char;
            }
        }
    }
    
    fn inv_cadena(&mut self) {  //opcion 8
        for i in 0..self.longitud / 2 {
            let temp = self.caracteres[i];
            self.caracteres[i] = self.caracteres[self.longitud - 1 - i];
            self.caracteres[self.longitud - 1 - i] = temp;
        }
    }
    fn palindromo(&self) -> bool { //opcion 9
        let mut texto: [char; N] = ['\0'; N];
        let mut len: usize = 0;
        for i in 0..self.longitud {
            let mut c = self.caracteres[i];
            if c == ' ' {
                continue;
            }
            if c >= 'A' && c <= 'Z' {
                c = ((c as u8) + 32) as char;
            }
            texto[len] = c;
            len += 1;
        }
        let mut i: usize = 0;
        while i < len / 2 {
            if texto[i] != texto[len - 1 - i] {
                return false;
            }
            i += 1;
        }
        true
    }      
    fn reemplazar_pos_car(&mut self, pos:usize, c: char) {  //opcion 10
        if pos >= 1 && pos <= self.longitud {
            self.caracteres[pos - 1] = c;
        }
    }

    fn reemplazar_car(&mut self, c: char, x: char)  {  //opcion 11
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                self.caracteres[i] = x;
            }
        }
    }
    fn cont_voc_cons(&self) -> (u32, u32) {  //opcion 12
        let mut vocales: u32 = 0;
        let mut consonantes: u32 = 0;

        for i in 0..self.longitud {
            let c = self.caracteres[i];

            if VOCALES.contains(c) {
                vocales += 1;
            } else if ALFABETO.contains(c) {
                consonantes += 1;
            }
        }
        (vocales, consonantes)
    }
       fn eliminar_pos(& mut self, pos:usize) { //opción 13
            if pos >= 1 && pos <= self.longitud {
                let index = pos - 1; // ajustar a índice 0
                for i in index..self.longitud - 1 {
                    self.caracteres[i] = self.caracteres[i + 1];
                }
                self.caracteres[self.longitud - 1] = '\0'; // limpiar el último carácter
                self.longitud -= 1; // reducir la longitud
            }
    }

    fn obtener_subcadena(&self, inicio: usize, fin: usize) -> Cadena { //opción 14
            let mut subcadena = Cadena::new();
            if inicio >= 1 && fin <= self.longitud && inicio <= fin {
                for i in (inicio - 1)..fin {
                    subcadena.add_char(self.caracteres[i]);
                }
            }
            subcadena
    }
        
    fn inv_cadena_mas_la_primer_letra(&mut self) { //opción 15 
       if self.longitud == 0 {return;}
       let ult_char: char = self.caracteres[self.longitud - 1];
       self.inv_cadena();
       self.longitud +=1;
       self.caracteres[self.longitud - 1] = ult_char;        
    }
    fn verMAY(&self, c: char)->bool{
        if c>='A' && c<='Z' || c=='Ñ'{
         true
        }
        else{
            false
        }
    }

    fn verminu(&self, c: char)->bool{
        if c>='a' && c<='z' || c== 'ñ'{
         true
        }
        else{
            false
        }
    }
    
    fn encriptador_cesar(&mut self, c : usize) { //opción 16
    let minus = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','ñ','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mayus = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','Ñ','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    
    for i in 0..self.longitud {
    //mayuscula
        if self.verMAY(self.caracteres[i]) {
            for j in 0..27 {
                if self.caracteres[i] == mayus[j] {
                    if j > 23 { 
                        self.caracteres[i] = mayus[j + c - 24];
                    } else {
                        self.caracteres[i] = mayus[j + c];
                    }
                    break;
                }
            }

    //minuscula
        } else if self.verminu(self.caracteres[i]) {

            for j in 0..27 {
                if self.caracteres[i] == minus[j] {
                    if j > 23 { 
                        self.caracteres[i] = minus[j + c - 24];
                    } else {
                        self.caracteres[i] = minus[j + c];
                    }
                    break;
                }
            }

        }
    }
}


    //    fn insertar_subcadena(&mut self, sub: &Cadena, p: usize) {
    //        if p >0 && p <= self.longitud + 1 && self.longitud + sub.longitud <= N{
    //            for i in (p - 1..self.longitud).rev() {
    //                self.caracteres[i + sub.longitud] = self.caracteres[i];
    //            }
    //            for i in 0..self.longitud {
    //                self.caracteres[p - 1 + i] = sub.caracteres[i];
    //            }
    //            self.longitud += t;
    //        }
    //    }

       //            for i in (p..self.longitud).rev() { self.caracteres[i + t] = self.caracteres[i]; }
    //            for i in 0..sub.longitud { self.caracteres[p + i] = sub.caracteres[i]; }
    //            if e { self.caracteres[p + sub.longitud] = ' '; }
    //            self.longitud += t;
    //        }
    //    }
    //}

    //    fn insertar_subcadena(&mut self, sub: &Cadena, pos: usize) {
    //    if pos > 0 && pos <= self.longitud + 1 {
    //        let e = pos <= self.longitud && self.caracteres[pos - 1] == ' ';
     //       let p = pos - 1 + e as usize;
     //       let t = sub.longitud + e as usize;
      //      if self.longitud + t <= N {
    //            for i in (p..self.longitud).rev() { self.caracteres[i + t] = self.caracteres[i]; }
    //            for i in 0..sub.longitud { self.caracteres[p + i] = sub.caracteres[i]; }
    //            if e { self.caracteres[p + sub.longitud] = ' '; }
    //            self.longitud += t;
    //        }
    //    }
    //}





        fn insertar_subcadena(&mut self, sub:&Cadena, p: usize){
           if p > 0 && p <= self.longitud + 1 {
            let e = p <= self.longitud && self.caracteres[p - 1] ==' ';
            let k = p - 1 + e as usize;
            let t = sub.longitud + e as usize;
            if self.longitud + t <=N {
                for i in (k..self.longitud).rev() {
                    self.caracteres[i + t] = self.caracteres[i];
                }
                for i in 0..sub.longitud {
                    self.caracteres[k + i] = sub.caracteres[i];
                }
                if e {
                    self.caracteres[k + sub.longitud] = ' ';
                }
                self.longitud += t;
            }
           } 
    } 

    // limpia la cadena para poder ingresar una nueva
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }


    //p <= self.longitud && self.longitud[p - 1] = ' ';

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
    println!("║  9. Verificar palíndromo         ║");
    println!("║ 10. Reemplazar carácter          ║");
    println!("║ 11. Reemplazar carácter por otro ║");
    println!("║ 12. Contar vocales y consonantes ║");
    println!("║ 13. Extraer subcadena            ║");
    println!("║ 14. Eliminar carácter en posición║");
    println!("║ 15. Invertir cadena + n posicion ║");
    println!("║ 16. Encriptar con César          ║");
    println!("║ 17. Insertar una subcadena       ║");
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

                println!("  ✓ Cadena cargada ");
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
                    if c.palindromo() {
                        println!(" La cadena es un palíndromo.");
                    } else {
                        println!(" La cadena no es un palíndromo.");
                    }
                }
                "10" => {
                    println!("  Ingresa la posición a reemplazar (1 = izquierda):");
                    match leer_numero() {
                        Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                            println!("  Ingresa el nuevo carácter:");
                            let entrada = leer_linea();
                            match entrada.chars().next() {
                                Some(car) => {
                                    c.reemplazar_pos_car(pos, car);
                                    println!("  Carácter en posición {} reemplazado por '{}'.", pos, car);
                                }
                                None => println!("  No ingresaste ningún carácter."),
                            }
                        }
                        Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obt_longitud()),
                        None    => println!("  Posición inválida."),
                    }
                }
                "11" => {
                    println!("  Ingresa el carácter a reemplazar:");
                    let entrada = leer_linea();
                    match entrada.chars().next() {
                        Some(car) => {
                            println!("  Ingresa el nuevo carácter:");
                            let entrada2 = leer_linea();
                            match entrada2.chars().next() {
                                Some(car2) => {
                                    c.reemplazar_car(car, car2);
                                    println!("  Carácter '{}' reemplazado por '{}'.", car, car2);
                                }
                                None => println!("  No ingresaste ningún carácter."),
                            }
                        }
                        None => println!("  No ingresaste ningún carácter."),
                    }
                }
                "12" => {
                    let (vocales, consonantes) = c.cont_voc_cons();
                    if vocales == 0 && consonantes == 0 {
                        println!(" La cadena no contiene ningún carácter.");
                    } else {
                        println!(" {} Vocales y {} Consonantes", vocales, consonantes);
                    }
                }
                
                "13" => {
                    if c.obt_longitud() == 0 {
                        println!(" La cadena está vacía.");
                    } else {
                        println!(" Ingresa la posición de inicio (1-based):");
                        if let Some(inicio) = leer_numero() {
                            println!(" Ingresa la posición final:");
                            if let Some(fin) = leer_numero() {
                                let sub = c.obtener_subcadena(inicio, fin);
                                print!(" Subcadena extraída: ");
                                sub.mostrar();
                            } else {
                                println!(" Posición inválida.");
                            }
                        } else {
                            println!(" Posición inválida.");
                        }
                    }
                }

                "14" => {
                    if c.obt_longitud() == 0 {
                        println!(" La cadena está vacía.");
                    } else {
                        println!(" Ingresa la posición a eliminar (1-based):");
                        match leer_numero() {
                            Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                                c.eliminar_pos(pos);
                                print!(" Cadena resultante: ");
                                c.mostrar();
                            }
                            Some(_) => println!(" Posición fuera de rango (1 a {}).", c.obt_longitud()),
                            None => println!(" Posición inválida."),
                        }
                    }
                } 
                "15" => {
                    c.inv_cadena_mas_la_primer_letra();
                    println!(" Cadena invertida mas la primer letra");
                } 
                "16" => {
                    println!(" Ingresa el número de posiciones para encriptar (1-26):");
                    match leer_numero() {
                        Some(cifrado) if cifrado >= 1 && cifrado <= 26 => {
                            c.encriptador_cesar(cifrado);
                            println!(" Cadena encriptada con César.");
                        }
                        Some(_) => println!(" Número fuera de rango (1-26)."),
                        None => println!(" Número inválido."),
                    }
                }
                "17" => {
                    if c.obt_longitud() == 0 {
                        println!(" La cadena está vacía.");
                    } else {
                        println!(" Ingresa la subcadena a insertar:");
                        let mut subcadena = Cadena::new();
                        let entrada = leer_linea();
                        for ch in entrada.chars() {
                            subcadena.add_char(ch);
                        }
                        println!(" Ingresa la posición para insertar la subcadena (1 = izquierda):");
                        match leer_numero() {
                            Some(pos) if pos >= 1 && pos <= c.obt_longitud() + 1 => {
                                c.insertar_subcadena(&subcadena, pos);
                                print!(" Cadena resultante: ");
                                c.mostrar();
                            }
                            Some(_) => println!(" Posición fuera de rango (1 a {}).", c.obt_longitud() + 1),
                            None => println!(" Posición inválida."),
                        }
                    }
                }

            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}