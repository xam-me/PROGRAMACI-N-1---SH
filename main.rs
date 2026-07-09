const NF: usize = 120;
const NC: usize = 120;

struct Matriz {
    nro_filas:    usize,
    nro_columnas: usize,
    elemento: [[u64; NC]; NF],
}

impl Matriz {
    fn new(nro_filas: usize, nro_columnas: usize) -> Matriz {
        Matriz {
            nro_filas,
            nro_columnas,
            elemento: [[0; NC]; NF],
        }
    }

    fn obt_nro_filas(&self) -> usize {
        self.nro_filas
    }

    fn obt_nro_columnas(&self) -> usize {
        self.nro_columnas
    }

    // leer celda
    fn get_celda(&self, f: usize, c: usize) -> u64 {
        self.elemento[f][c]
    }

    // escribir celda
    fn set_celda(&mut self, f: usize, c: usize, valor: u64) {
        self.elemento[f][c] = valor;
    }
    
    fn summar_elementos(&self) -> u64 {
        let mut suma=0;
        for i in 0..self.obt_nro_filas() {
            for j in 0..self.obt_nro_columnas(){
                suma += self.elemento[i][j];
            }
        }
        suma
    }

    // ── Mostrar ───────────────────────────────────────────────────

    fn mostrar(&self) {
        println!();
        for f in 0..self.nro_filas {
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("┌─────────┐");
            }
            println!();
            print!("  ");
            for c in 0..self.nro_columnas {
                print!("│{:^9}│", self.elemento[f][c]);
            }
            println!();
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("└─────────┘");
            }
            println!();
        }
        print!("  ");
        for c in 0..self.nro_columnas {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }

    fn sumar_pares(&self) -> u64 {
        let mut suma = 0;
        for i in 0..self.obt_nro_filas() {
            for j in 0..self.obt_nro_columnas() {
                if self.elemento[i][j] % 2 == 0 {
                    suma += self.elemento[i][j];
                }
            }
        }
        suma
    }

    fn sumar_impares(&self) -> u64 {
        let mut suma = 0;
        for i in 0..self.obt_nro_filas() {
            for j in 0..self.obt_nro_columnas() {
                if self.elemento[i][j] % 2 != 0 {
                    suma += self.elemento[i][j];
                }
            }
        }
        suma
    }

    fn elemento_mayor(&self) -> u64 {
        let mut mayor = self.elemento[0][0];
        for i in 0..self.obt_nro_filas() {
            for j in 0..self.obt_nro_columnas() {
                if self.elemento[i][j] > mayor {
                    mayor = self.elemento[i][j];
                }
            }
        }
        mayor
    }

    fn sumar_columna(&self, columna : usize) -> u64 {
        let mut suma = 0;
        for fila in 0..self.obt_nro_filas() {
            suma += self.elemento[fila][columna];
        }
        suma
    }

    fn sumar_fila(&self, fila:usize ) -> u64 {
        let mut suma = 0;
        for columna in 0..self.obt_nro_columnas() {
            suma += self.elemento[fila][columna];    
        }
        suma
    }

    

    fn esta_ordenada(&self) -> bool {
        let filas = self.nro_filas;
        let columnas = self.nro_columnas;
        
        // Recorrer toda la matriz
        for i in 0..filas {
            for j in 0..columnas {
                let actual = self.elemento[i][j];
                
                // Verificar siguiente elemento (si existe)
                if j + 1 < columnas {
                    // Misma fila, siguiente columna
                    if actual > self.elemento[i][j + 1] {
                        return false;
                    }
                } else if i + 1 < filas {
                    // Última columna, pasar a siguiente fila
                    if actual > self.elemento[i + 1][0] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn sumar_borde(&self) -> u64 {
    let mut suma = 0;
    let filas = self.nro_filas;
    let columnas = self.nro_columnas;
    for i in 0..filas {
        for j in 0..columnas {
            if i == 0 || i == filas - 1 || j == 0 || j == columnas - 1 {
                suma += self.elemento[i][j];
            }
        }
    }
    suma
}

    pub fn busqueda_binaria(&self, ele:u64)->Option<(usize,usize)>{
        println!("Buscando {} en la matriz...", ele);
        if !self.esta_ordenada() {
        println!("La matriz no está ordenada, El resultado podría ser incorrecto");
        }
        let mut ini:usize = 0;
        let mut fin:usize = self.nro_columnas * self.nro_filas -1;
        while ini <= fin {
            let medio = (ini + fin) / 2;
            let fila = medio / self.nro_columnas;
            let col = medio % self.nro_columnas;
            let valor = self.elemento[fila][col];
            if valor == ele{
                return Some((fila, col));
            }
            if ele < valor {
                if medio == 0{
                    break;
                }
                fin  = medio-1;
            }else{
                ini = medio +1;
            }

        }
        None
    }

        fn ordenar_espiral(&mut self) {
        let filas = self.nro_filas;
        let columnas = self.nro_columnas;
        let total_elementos = filas * columnas;
        let mut elementos: Vec<u64> = Vec::with_capacity(total_elementos);
        for i in 0..filas {
            for j in 0..columnas {
                elementos.push(self.elemento[i][j]);
            }
        }
        
        elementos.sort();
        
        let posiciones = self.generar_posiciones_espiral();
        
        for (idx, (fila, col)) in posiciones.iter().enumerate() {
            self.elemento[*fila][*col] = elementos[idx];
        }
    }

    fn generar_posiciones_espiral(&self) -> Vec<(usize, usize)> {
        let filas = self.nro_filas;
        let columnas = self.nro_columnas;
        let mut posiciones: Vec<(usize, usize)> = Vec::new();
        
        let centro_fila = filas / 2;
        let centro_col = columnas / 2;
        
        let mut visitado = vec![vec![false; columnas]; filas];
        
        let direcciones = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0;
        
        let mut fila = centro_fila;
        let mut col = centro_col;
        let mut paso = 1;
        
        visitado[fila][col] = true;
        posiciones.push((fila, col));
        
        while posiciones.len() < filas * columnas {
            for _ in 0..paso {
                let nueva_fila = fila as isize + direcciones[dir_idx].0;
                let nueva_col = col as isize + direcciones[dir_idx].1;
                
                if nueva_fila >= 0 && nueva_fila < filas as isize && 
                   nueva_col >= 0 && nueva_col < columnas as isize {
                    
                    let f = nueva_fila as usize;
                    let c = nueva_col as usize;
                    
                    if !visitado[f][c] {
                        visitado[f][c] = true;
                        posiciones.push((f, c));
                        fila = f;
                        col = c;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            dir_idx = (dir_idx + 1) % 4;
            
            if dir_idx % 2 == 0 {
                paso += 1;
            }
        }
        
        posiciones
    }

    fn suma_diagonal_mayor(&self)->u64{
    let mut mayor = 0;
    
    for n in 0..(self.nro_filas + self.nro_columnas - 1){
        let mut suma = 0;
        let mut i;
        let mut j;
        if n < self.nro_filas {
            i = n;
            j = 0;
        }
        else {
            i = self.nro_filas-1;
            j = n - self.nro_filas + 1;
        }
        while i < self.nro_filas && j < self.nro_columnas {
            suma += self.elemento[i][j];
            if i == 0 {
                break;
            }
            i -= 1;
            j += 1;
        }
        if suma > mayor{
            mayor = suma;
        }
    }
    return mayor;
}

    fn prom_diagonal_mayor_der(&self) -> f64 {
    let mut mayor = 0;
    let mut cantidad_mayor = 0;

    for n in 0..(self.nro_filas + self.nro_columnas - 1) {
        let mut suma = 0;
        let mut cantidad = 0;
        let mut i;
        let mut j;
        if n < self.nro_filas {
            i = n;
            j = 0;
        }
        else {
            i = self.nro_filas - 1;
            j = n - self.nro_filas + 1;
        }
        while i < self.nro_filas && j < self.nro_columnas {
            suma += self.elemento[i][j];
            cantidad += 1;
            if i == 0 {
                break;
            }
            i -= 1;
            j += 1;
        }
        if suma > mayor {
            mayor = suma;
            cantidad_mayor = cantidad;
        }
    }

    return mayor as f64 / cantidad_mayor as f64;
}

    fn prom_diagonal_mayor_izq(&self) -> f64 {
    let mut mayor = 0;
    let mut cantidad_mayor = 0;

    for n in 0..(self.nro_filas + self.nro_columnas - 1) {
        let mut suma = 0;
        let mut cantidad = 0;
        let mut i;
        let mut j;
        if n < self.nro_filas {
            i = n;
            j = self.nro_columnas - 1;
        }
        else {
            i = self.nro_filas - 1;
            j = self.nro_columnas - 1 - (n - self.nro_filas + 1);
        }
        while i < self.nro_filas && j < self.nro_columnas {
            suma += self.elemento[i][j];
            cantidad += 1;
            if i == 0 || j == 0 {
                break;
            }
            i -= 1;
            j -= 1;
        }
        if suma > mayor {
            mayor = suma;
            cantidad_mayor = cantidad;
        }
    }

    return mayor as f64 / cantidad_mayor as f64;
}

fn rotar_borde(&mut self) {
    let rows = self.nro_filas;
    let cols = self.nro_columnas;
    let mut coordenadas = Vec::new();

    for c in 0..cols {
        coordenadas.push((0, c));
    }
    for f in 1..rows {
        coordenadas.push((f, cols - 1));
    }
    for c in (0..cols - 1).rev() {
        coordenadas.push((rows - 1, c));
    }
    for f in (1..rows - 1).rev() {
        coordenadas.push((f, 0));
    }
    let mut valores: Vec<u64> = coordenadas
        .iter()
        .map(|&(f, c)| self.elemento[f][c])
        .collect();
    valores.rotate_right(8);
    for (i, &(f, c)) in coordenadas.iter().enumerate() {
        self.elemento[f][c] = valores[i];
    }
}

    fn rotar_anillo_interno(&mut self) {
        let fil = self.nro_filas;
        let cols = self.nro_columnas;
        let mut coordenadas = Vec::new();

        for c in 1..cols - 1 {
        coordenadas.push((1, c));
        }
        for f in 2..fil - 1 {
        coordenadas.push((f, cols - 2));
        }
        for c in (1..cols-2).rev() {
        coordenadas.push((fil-2,c))    
        }
        for f in (2..fil - 2).rev() {
        coordenadas.push((f, 1));
        }
        let mut valores: Vec<u64> = coordenadas
          .iter()
          .map(|&(f, c)| self.elemento[f][c])
          .collect();
        valores.rotate_right(3);
        for (i , &(f,c)) in coordenadas.iter().enumerate() {
        self.elemento[f][c] = valores[i];
    }
}

    fn transponer(&self) -> Matriz {
        let mut nueva = Matriz::new(self.nro_columnas, self.nro_filas);
        for i in 0..self.nro_filas {
            for j in 0..self.nro_columnas {
                nueva.set_celda(j, i, self.elemento[i][j]);
            }
        }
        nueva
    }

    // Función auxiliar para verificar si un número es primo
fn es_primo(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

    // Cuenta cuántos números primos hay en la matriz
fn contar_primos(&self) -> usize {
    let mut contador = 0;
    for i in 0..self.nro_filas {
        for j in 0..self.nro_columnas {
            if Self::es_primo(self.elemento[i][j]) {
                contador += 1;
            }
        }
    }
    contador
}

    // Gira la matriz 90 grados a la derecha
    fn girar_90_grados(&mut self) {
        let n = self.nro_filas;
        let mut nueva = Matriz::new(n, n);
    
        for i in 0..n {
            for j in 0..n {
                nueva.set_celda(j, n - 1 - i, self.elemento[i][j]);
            }
        }
        *self = nueva;
    }

    // Cuenta cuántas veces aparece cada número
    fn frecuencias(&self) -> std::collections::HashMap<u64, usize> {
        let mut frecuencias = std::collections::HashMap::new();
        for i in 0..self.nro_filas {
            for j in 0..self.nro_columnas {
                *frecuencias.entry(self.elemento[i][j]).or_insert(0) += 1;
            }
        }
        frecuencias
    }

    // Verifica si la matriz es mágica (todas las filas, columnas y diagonales suman lo mismo)
fn es_magica(&self) -> bool {
    if self.nro_filas != self.nro_columnas {
        return false;
    }
    
    let n = self.nro_filas;
    let suma_esperada: u64 = (0..n).map(|i| self.elemento[i][i]).sum();
    
    // Verificar filas
    for i in 0..n {
        let suma_fila: u64 = (0..n).map(|j| self.elemento[i][j]).sum();
        if suma_fila != suma_esperada {
            return false;
        }
    }
    
    // Verificar columnas
    for j in 0..n {
        let suma_col: u64 = (0..n).map(|i| self.elemento[i][j]).sum();
        if suma_col != suma_esperada {
            return false;
        }
    }
    
    // Verificar diagonal secundaria
    let suma_secundaria: u64 = (0..n).map(|i| self.elemento[i][n - 1 - i]).sum();
    if suma_secundaria != suma_esperada {
        return false;
    }
    
    true
}

}

// ═══════════════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════════════

fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    // ── crear la matriz 3x3 ──────────────────────────────────────
    let mut m = Matriz::new(5, 5);

    // ── cargar datos directamente ────────────────────────────────
    //        fila  col  valor
    m.set_celda(0, 0, 1);   m.set_celda(0, 1, 1);   m.set_celda(0, 2, 2);    m.set_celda(0, 3, 4);    m.set_celda(0, 4, 6);
    m.set_celda(1, 0, 4);   m.set_celda(1, 1, 1);   m.set_celda(1, 2, 2);    m.set_celda(1, 3, 2);    m.set_celda(1, 4, 2);
    m.set_celda(2, 0, 7);   m.set_celda(2, 1, 8);   m.set_celda(2, 2, 10);   m.set_celda(2, 3, 4);    m.set_celda(2, 4, 1);
    m.set_celda(3, 0, 3);   m.set_celda(3, 1, 7);   m.set_celda(3, 2, 6);    m.set_celda(3, 3, 5);    m.set_celda(3, 4, 10);
    m.set_celda(4, 0, 6);   m.set_celda(4, 1, 1);   m.set_celda(4, 2, 8);    m.set_celda(4, 3, 9);    m.set_celda(4, 4, 0);

    // ── mostrar ──────────────────────────────────────────────────
    println!("Matriz original:");
    m.mostrar();

    // ── sumar elementos ──────────────────────────────────────────
    println!("Suma de los elementos de cada posicion del vector es : {}", m.summar_elementos());    
    println!("Suma de los elementos pares de cada posicion del vector es : {}", m.sumar_pares());
    println!("Suma de los elementos impares de cada posicion del vector es : {}", m.sumar_impares());
    println!("Elemento mayor de la matriz es : {}", m.elemento_mayor());
    println!("Suma de los elementos de la columna 0 es : {}", m.sumar_columna(0));    
    println!("Suma de los elementos de la fila 0 es : {}", m.sumar_fila(0));
    println!("Celda [0][0] usando get_celda: {}", m.get_celda(0, 0));
    println!("Busqueda binaria del elemento 8: {:?}", m.busqueda_binaria(8));
    println!("La suma de los elementos que están en los bordes de esta matriz es de {}", m.sumar_borde());
    println!("La matriz está ordenada: {}", m.esta_ordenada());
    //println!("La matriz ordenada en espiral es:");
    //m.ordenar_espiral();
    //m.mostrar();
    println!("La suma de la diagonal mayor es: {}", m.suma_diagonal_mayor());


    //println!("Rotando el borde de la matriz...");
    //m.rotar_borde();
    //m.mostrar();

    println!("Rotando el anillo interno de la matriz...");
    m.rotar_anillo_interno();
    m.mostrar();
    //println!("El promedio de la diagonal mayor derecha es: {}", m.prom_diagonal_mayor_der());
    //println!("El promedio de la diagonal menor izquierda es: {}", m.prom_diagonal_mayor_izq());
    //println!("La matriz transpuesta es:");
    //let transpuesta = m.transponer();
    //transpuesta.mostrar();
    //println!("La cantidad de números primos en la matriz es: {}", m.contar_primos());
    //println!("Girando la matriz 90 grados a la derecha...");
    //m.girar_90_grados();
    //m.mostrar();
    //println!("Frecuencia de cada número en la matriz:");
    //let frecuencias = m.frecuencias();
    //for (numero, frecuencia) in frecuencias {
    //    println!("Número: {}, Frecuencia: {}", numero, frecuencia); 
    //}
    //println!("La matriz es mágica: {}", m.es_magica()); 
    
}
