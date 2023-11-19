se std::collections::HashMap;

/* PRIMERA SOLUCIÓN */
fn primera_solucion(frase: &str) {
    let mut diccionario: HashMap<&str, i32> = HashMap::new();
    let mut orden: Vec<&str> = Vec::new();

    let palabras = frase.split_whitespace();
    for palabra in palabras {
        if diccionario.contains_key(palabra) {
            diccionario.insert(palabra, diccionario[palabra] + 1);
            continue;
        }
        orden.push(palabra);
        diccionario.insert(palabra, 1);
    }

    let mut salida = "".to_string();
    for o in orden {
        let coincidencias = diccionario.get(o).unwrap();
        salida += &format!("{}{}", o, coincidencias);
    }

    println!("{salida}")
}

/* SEGUNDA SOLUCIÓN */
#[derive(Debug)]
struct PalabraEncontrada {
    palabra: String,
    coincidencias: u16,
    orden: u16,
}

fn segunda_solucion(frase: &str) {
    let palabras = frase.split_whitespace();
    let mut mapa_palabras: HashMap<String, PalabraEncontrada> = HashMap::new();

    for p in palabras {
        let palabra = p.to_ascii_lowercase();

        if mapa_palabras.contains_key(&palabra) {
            let mapa = mapa_palabras.get_mut(&palabra).unwrap();
            mapa.coincidencias = mapa.coincidencias + 1;
            continue;
        }

        let nueva_palabra = PalabraEncontrada {
            palabra: p.to_string(),
            coincidencias: 1,
            orden: mapa_palabras.len() as u16,
        };

        mapa_palabras.insert(palabra, nueva_palabra);
    }

    let mut coleccion_palabras = Vec::new();
    for (_, v) in mapa_palabras {
        coleccion_palabras.push(v);
    }

    coleccion_palabras.sort_by_key(|p| p.orden);

    let mut salida = "".to_string();

    for v in coleccion_palabras {
        let texto = format!("{}{}", v.palabra, v.coincidencias);
        salida += &texto;
    }

    println!("{salida}");
}


fn main() {
    let frase = "murcielago leon jirafa cebra elefante rinoceronte hipopotamo ardilla mapache zorro lobo oso puma jaguar tigre leopardo gato perro caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco tigre jaguar leopardo oso lobo zorro mapache ardilla elefante rinoceronte hipopotamo cebra jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago tucan loro canario colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago flamenco pinguino tucan loro canario colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo leon jirafa murcielago caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo leon jirafa murcielago caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco murcielago leon jirafa cebra elefante rinoceronte hipopotamo ardilla mapache zorro lobo oso puma jaguar tigre leopardo gato perro caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo jirafa leon murcielago pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon paloma colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon paloma colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon";
    primera_solucion(frase);
    segunda_solucion(frase);
}
