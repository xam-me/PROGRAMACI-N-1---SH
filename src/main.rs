    use std::io::{self, Write};  // Para leer entrada y mostrar el menú sin salto de línea
struct Numero {
    valor: u64
}

impl Numero {
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool { //opción 1, devuelve true si el número es par, false en caso contrario
        self.valor % 2 == 0
    }

    fn es_primo(&self) -> bool { //opción 2, devuelve true si el número es primo, false en caso contrario
        let num: u64 = self.valor;

        if num < 2 {
            return false;
        }
        if num == 2 {
           return true;
        }

        if num % 2 == 0 {
            return false;
        }

        let t: u64 = (num as f64).sqrt() as u64 + 1;
        let mut d: u64 = 3;

        while d <= t {
            if num % d == 0 {
                return false;
            }
            d += 2;
        };

        true
    }

    fn cantidad_digitos(&self) -> u64 { //opción 3, devuelve la cantidad de dígitos que tiene el número, ej: 123 → 3, 400 → 3, 9 → 1
        let mut count: u64 = 0;
        let mut num: u64 = self.valor;

        while num > 0 {
            num /= 10;
            count += 1;
        }

        count
    }
    fn elevado(&self, base: u64, exp: u64) -> u64 { //función auxiliar para calcular la potencia, se utiliza en la función es_armstrong
        let mut resultado: u64 = 1;
        for _ in 0..exp {
            resultado = resultado * base;
        }
        resultado
    }
    fn invertir(&self) -> u64 { //opción 4, devuelve el número invertido, ej: 123 → 321, 400 → 4, 9 → 9
        if self.valor < 10 {
            return self.valor;
        }
        else {
            let mut num = self.valor;
            let mut invertido = 0;

            while num > 0 {
                let digito = num % 10;
                invertido = invertido * 10 + digito;
                num /= 10;
            }
            invertido
        }
    }

    fn es_capicua(&self) -> bool { //opción 5, devuelve true si el número es capicúa, false en caso contrario
        self.valor == self.invertir()
    }

    fn es_elevado(&self, base: u64, exp: u64) -> u64 { //función auxiliar para calcular la potencia, se utiliza en la función es_armstrong
        let mut resultado: u64 = 1;
        for _ in 0..exp {
            resultado *= base;
        }
        resultado
    }

    fn es_armstrong(&self) -> bool { //opción 6, devuelve true si el número es un número de Armstrong, false en caso contrario
        let mut n = self.valor;
        let mut suma = 0;
        let expo: u64 = self.cantidad_digitos();

        while n > 0 {
            let digito = n % 10;
            suma = suma + self.elevado(digito, expo);
            n /= 10; // esto es igual a: n = n / 10;
        }
        suma == self.valor
    } 

    //funcion que devuelva la cantidad de digitos pares que contiene un numero, ej:
    //341 = 1 digito par - 379 = 0 digitos pares - 482 = 3 digitos pares.
    fn cant_dig_pares(&self) -> u64 { //opción 7, devuelve la cantidad de dígitos pares que tiene el número
        let mut num = self.valor;
        let mut cont = 0;

        while num > 0 {
            let digito = num % 10;
            if digito % 2 == 0 {
                cont += 1;
            }
            num /= 10;
        }
        cont
    }

    fn raiz_digital(&self) -> u64 { //opción 8, devuelve la raíz digital del número
        let mut n: u64 = self.valor;
        while n >= 10 {
            let mut suma = 0;
            let mut temp = n;
            while temp > 0 {
                suma = suma + (temp % 10);
                temp = temp / 10;
            }
            n = suma;
        }
        n
    }

    //1.- La conjetura de Collatz: Si el número es par, divídelo entre 2; si es impar, multiplícalo por 3
    //y súmale 1. Repetir hasta llegar a 1. Implementar un método que cuente los pasos necesarios y otro que
    //encuentre el valor máximo alcanzado durante la secuencia.
    
    fn collatz(&self) -> (u64, u64) { //opción 9, devuelve una tupla con el número de pasos y el máximo alcanzado
    let mut n = self.valor;
    let mut pasos: u64 = 0;
    let mut maximo: u64 = 0;

      while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        pasos += 1;
        if n > maximo {
            maximo = n;
        }
      }
      (pasos,  maximo)
    }


    fn insertar_digito(&self, digito: u64, posicion: u64) -> u64 {//opción 10
    let total_cifras = self.cantidad_digitos();

    let mut divisor: u64 = 1;

    for _ in 0..(total_cifras - posicion + 1) {  // ← +1 para que la posición empiece en 1
        divisor = divisor * 10;
    }

    let parte_izquierda = self.valor / divisor;
    let parte_derecha   = self.valor % divisor;

    parte_izquierda * divisor * 10 + digito * divisor + parte_derecha
}

    fn obtener_digito_posicion(&self, posicion: u64) -> u64 { //opción 11
        let total = self.cantidad_digitos();
        if posicion < 1 || posicion > total {
            return 0;
        }
        (self.valor / 10u64.pow((total - posicion) as u32)) % 10
    }

    fn buscar_digito(&self, digito: u64) -> u64 { //opción 12
        let total = self.cantidad_digitos();
        for i in 1..=total {
            let digit = (self.valor / 10u64.pow((total - i) as u32)) % 10;
            if digit == digito {
                return i;
            }
        }
        0
    }
    fn rotar_todos_los_numeros_n_veces_izquierda(&self, n: u64) -> u64 { //opción 13 eje-> 12345 "rotar 3" -> 45123
        let mut valor: u64 = self.valor;
        for _ in 0..n {
        let dig: u64 = self.cantidad_digitos();
        let k :u64 = self.es_elevado(10 ,dig-1);
        let div : u64 = valor / k;
        valor = (valor % k) * 10 + div;     
        }
        valor 
    }

    fn resetear(&mut self, nuevo: u64) {
        self.valor = nuevo;
    }
}
// Función para leer una línea de entrada del usuario
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
// Función para leer un número u64 del usuario, devuelve None si no es válido
fn leer_numero() -> Option<u64> {
    leer_linea().parse::<u64>().ok()
}

fn mostrar_menu(n: &Numero) {
    println!("\n╔══════════════════════════════════╗");
    println!("║   NÚMERO ACTUAL: {:>14}  ║", n.valor);
    println!("╠══════════════════════════════════╣");
    println!("║  Consulta                        ║");
    println!("║  1. ¿Es par?                     ║");
    println!("║  2. ¿Es primo?                   ║");
    println!("║  3. Cantidad de dígitos          ║");
    println!("║  4. Invertir                     ║");
    println!("║  5. Es capicua?                  ║");    
    println!("║  6. ¿Es Armstrong?               ║");
    println!("║  7. Cantidad Dig Par             ║");
    println!("║  8. Raiz Digital                 ║");
    println!("║  9. Collatz                      ║");
    println!("║  10. Insertar dígito             ║");
    println!("║  11. Obtener dígito en posición  ║");
    println!("║  12. Buscar dígito               ║");
    println!("║  13. Rotar numeros a la izquierda║");
    println!("╠══════════════════════════════════╣");
    println!("║  0. Ingresar nuevo número        ║");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Números - POO — Programación I");
    println!("════════════════════════════════════");
    println!("Ingresa un número para comenzar:");
    // Validar que el usuario ingrese un número válido antes de crear la instancia de Numero
    let valor_inicial: u64 = loop { //loop se repite hasta que encuentre un break
        match leer_numero() {
            Some(num) => break num,
            None    => println!("Número inválido. Intenta de nuevo:"),
        }
    };
    //creando una instancia de Numero con el valor inicial ingresado por el usuario
    //ESTA ES LA INSTANCIA DEL OBJETO NUMERO, A PARTIR DE AQUÍ SE UTILIZARÁ PARA REALIZAR TODAS LAS CONSULTAS
    let mut n =  Numero::new(valor_inicial);

    loop { //el menu se mostrará en un bucle infinito hasta que el usuario decida salir usando la opción 'Q' (break)
        mostrar_menu(&n);
        let opcion = leer_linea();

        match opcion.as_str() {  //usando match, se puede llamar a la función correspondiente.
            // Consultas
            "1" => println!("  ¿Es par?          → {}", n.es_par()),
            "2" => println!("  ¿Es primo?        → {}", n.es_primo()),
            "3" => println!("  Cantidad Digitos: → {}", n.cantidad_digitos()),
            "4" => println!("  Invertir:         → {}", n.invertir()),
            "5" => println!("  Es capicua?:      → {}", n.es_capicua()),
            "6" => println!("  ¿Es Armstrong?    → {}", n.es_armstrong()),
            "7" => println!("  Cantidad de Digitos Pares es    → {}", n.cant_dig_pares()),
            "8" => println!("  La raiz gitital es    → {}", n.raiz_digital()),
            "9" => {
                   let (pasos, maximo) = n.collatz();
                    println!("  Collatz → pasos: {}, máximo: {}", pasos, maximo);
            }
            "10" => {
                println!("  Ingresa el dígito a insertar (0-9):");
                match leer_numero() {
                    Some(digito) if digito <= 9 => {
                        println!("  Ingresa la posición (1 = izquierda):");
                        match leer_numero() {
                            Some(posicion) => {
                                let resultado = n.insertar_digito(digito, posicion);
                                println!("  Insertar dígito {} en posición {}: → {}", digito, posicion, resultado);
                            }
                            None => println!("  Posición inválida."),
                        }
                    }
                    Some(_) => println!("  El dígito debe estar entre 0 y 9."),
                    None    => println!("  Dígito inválido."),
                }
            }
            "11" => {
    println!("  Ingresa la posición (1 = izquierda):");
    match leer_numero() {
        Some(posicion) if posicion >= 1 && posicion <= n.cantidad_digitos() as u64 => {
            let resultado = n.obtener_digito_posicion(posicion);
            println!("  Dígito en posición {}: → {}", posicion, resultado);
        }
        Some(_) => println!("  Posición no encontrada"),
        None    => println!("  Posición inválida."),
    }
}

"12" => {
    println!("  Ingresa el dígito a buscar (0-9):");
    match leer_numero() {
        Some(digito) if digito <= 9 => {
            let resultado = n.buscar_digito(digito);
            if resultado == 0 {
                println!("  El dígito {} no existe en el número.", digito);
            } else {
                println!("  Dígito {} encontrado en posición: → {}", digito, resultado);
            }
        }
        Some(_) => println!("  El dígito debe estar entre 0 y 9."),
        None    => println!("  Dígito inválido."),
    }
}
            "13" => {
                println!("  Ingresa la cantidad de veces que deben rotar los dígitos:");
                match leer_numero() {
                    Some(cant) => {
                        let resultado = n.rotar_todos_los_numeros_n_veces_izquierda(cant);
                        if cant <5 {
                        println!(" Rotar {} posiciones: → {}", cant, resultado);
                        } else {
                        println!(" Rotar {} posiciones es igual a rotar {} posiciones: → {}", cant, cant-5, resultado);    
                        }
                    }
                    None => println!("  Número de rotaciones inválido."),
                }
            }
            "0" => {
                println!("  Ingresa el nuevo número:");
                match leer_numero() {
                    Some(num) => { n.resetear(num); println!("  ✓ Nuevo número: {}", n.valor); }
                    None    => println!("  Número inválido, se mantiene {}", n.valor),
                }
            }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; } //aquí se rompe el ciclo con "q" o "Q"
            _ => println!("  Opción no válida."),
        }
    }
}