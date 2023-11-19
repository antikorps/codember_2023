fn primera_solucion(entrada: &str) {
    let mut valor = 0;
    let mut salida = "".to_string(); 

    for c in entrada.chars() {
        match c {
            '#' => valor += 1,
            '@' => valor -= 1,
            '*' => valor = valor * valor,
            '&' => salida += &format!("{valor}"),
            _ => panic!("dis make me panic"),
        }
    }
    println!("{salida}")
}


fn segunda_solucion(entrada: &str) {
    let mut valor = 0;
    let mut salida = "".to_string();

    for caracter in entrada.chars() {
        if caracter == '#' {
            valor += 1;
            continue
        }
        if caracter == '@' {
            valor -= 1;
            continue
        }
        if caracter == '*' {
            valor = valor * valor;
            continue
        }
        if caracter == '&' {
            salida += &valor.to_string();
        }
    }

    println!("{}", salida);
}

struct Decodificador {
    valor: i32,
    entrada: String,
    salida: String,
}

impl Decodificador {
    fn new(entrada: &str, valor: i32 )-> Decodificador {
        return Decodificador { valor, entrada: entrada.to_string(), salida: "".to_string() }
    }
    fn sumar(&mut self) {
        self.valor += 1;
    }
    fn restar(&mut self) {
        self.valor -= 1;
    }
    fn multiplicar(&mut self) {
        self.valor = self.valor * self.valor
    }
    fn pintar(&mut self) {
        self.salida += &self.valor.to_string()
    }
    fn decodificar(&mut self) {
        for c in self.entrada.clone().chars() {
            if c == '#' {
                self.sumar()
            }
            if c == '@' {
                self.restar()
            }
            if c == '*' {
                self.multiplicar()
            }
            if c == '&' {
                self.pintar()
            }
        }
    }
    fn resolver(&self) {
        println!("{}", self.salida)
    }
}

fn tercera_solucion(entrada: &str) {
    let mut dec = Decodificador::new(entrada, 0);
    dec.decodificar();
    dec.resolver();
}



fn main() {
    let entrada = "&###@&*&###@@##@##&######@@#####@#@#@#@##@@@@@@@@@@@@@@@*&&@@@@@@@@@####@@@@@@@@@#########&#&##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@&";

   primera_solucion(entrada);
   segunda_solucion(entrada);  
   tercera_solucion(entrada);
}
