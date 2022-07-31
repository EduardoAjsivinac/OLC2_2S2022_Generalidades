// base de datos biblioteca
mod biblioteca {

    // tabla libro
    pub mod libro {

        // definición de la estructura libro
        pub struct Libro {
            pub titulo  :String,
            pub autor   :String,
            pub genero  :String
        }

        /**
         * Lenguaje de definición de datos (DDL)
         * 
         * En la simulación solo contará con la función de 
         * crear una tabla y es porque en un struct no se puede
         * añadir o eliminar sus campos.
         * 
         * create_table: Permite crear una tabla de un tamaño fijo
         * según la estructura definida.
         */
        pub fn create_table(tamanio: i64) -> Vec<Libro> {
            let tabla: Vec<Libro> = Vec::with_capacity(tamanio as usize);
            println!("La tabla Libro ha sido creada");
            return tabla;
        }


        /**
         * Lenguaje de manipulación de datos (DML)
         * 
         * Se simulará las siguientes sentencias de una base de datos:
         *  SELECT
         *  INSERT
         *  UPDATE
         *  DELETE
         */

        /**
         * SELECT
         * 
         * La función imprimira los registros encontrados, se debe proporcionar
         * la tabla de referencia después indicar la posición o indice del 
         * registro que se desea seleccionar.
         * 
         *      Por ejemplo: select_by_id(tabla, id)
         */
        pub fn select_by_id(tabla: Vec<Libro>, id: usize) -> Libro {
            println!("Se seleccionó un registro de la tabla Libro");

            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "titulo", "autor", "genero");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "------------------", "------------------", "------------------");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                tabla[id].titulo, tabla[id].autor, tabla[id].genero);

            return Libro {
                titulo: tabla[id].titulo.clone(),
                autor: tabla[id].autor.clone(),
                genero: tabla[id].genero.clone()
            };
        }

        /**
         * SELECT
         * 
         * La función imprimira todos los registros, se debe proporcionar
         * la tabla de referencia.
         * 
         *      Por ejemplo: select(tabla, id)
         */
        pub fn select(tabla: Vec<Libro>) {
            println!("Se seleccionó todos los registros de la tabla Libro");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "titulo", "autor", "genero");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "------------------", "------------------", "------------------");
            for reg in tabla {
                println!(
                    "| {0: <50} | {1: <50} | {2: <50}",
                    reg.titulo, reg.autor, reg.genero);
            }
        }

        /**
         * INSERT
         * 
         * La función permite insertar un nuevo registro, se debe proporcionar 
         * la tabla de referencia luego se debe de agregar los valores del registro. 
         * Esta función retornara la referencia de la tabla.
         * 
         *      Por ejemplo: insert(tabla, valor_campo1, valor_campo2, ..., valor_campoN)
         */
        pub fn insert(mut tabla: Vec<Libro>, titulo: String, autor: String, genero: String) -> Vec<Libro> {
            if tabla.len() < tabla.capacity() {
                let registro = Libro {
                    titulo: titulo,
                    autor: autor,
                    genero: genero
                };

                tabla.push(registro);
                println!("Se insertó un registro en la tabla Libro");
            }
            else {
                println!("La tabla Libro ha llegado a su maxima capacidad");
            }
            return tabla;
        }

        /**
         * UPDATE
         * 
         * La función modificará los valores de un registro, se debe proporcionar 
         * la tabla para actualizar el registro luego se envía una estructura del mismo tipo para 
         * actualizar los campos.
         * 
         * Si el identificador es negativo se actualizara todos los campos
         *      Por ejemplo: update(tabla, id, estructura)
         * 
         * De esta función también puede nacer funciones derivadas tales como update_by_filter
         */
        pub fn update(mut tabla: Vec<Libro>, id: i64, libro_update :Libro) -> Vec<Libro> {
            if id >= 0 {
                tabla[id as usize].titulo = libro_update.titulo;
                tabla[id as usize].autor = libro_update.autor;
                tabla[id as usize].genero = libro_update.genero;
                println!("Se actualizó un registro de la tabla Libro");
            } else {
                let mut i = 0;
                while i < tabla.len() {
                    tabla[i].titulo = libro_update.titulo.clone();
                    tabla[i].autor = libro_update.autor.clone();
                    tabla[i].genero = libro_update.genero.clone();
                    i = i + 1;
                }
                println!("Se actualizó {} registro(s) de la tabla Libro", i.to_string());
            }
            return tabla;
        }

        /**
         * UPDATE
         * 
         * Se envia como parametro la tabla de referencia, el libro que simulará el filtro y
         * y el libro que tiene los campos de actualización.
         * 
         * Esta función actualizará todos los registros según el filtro.
         */
        pub fn update_by_filter(mut tabla: Vec<Libro>, libro_filtro: Libro, libro_update :Libro) -> Vec<Libro> {
            let mut i = 0;
            let mut cont = 0;

            // println!("{}", libro_filtro.titulo);
            // println!("{}", libro_filtro.autor);
            // println!("{}", libro_filtro.genero);

            while i < tabla.len() {
                if tabla[i].titulo == libro_filtro.titulo && 
                    tabla[i].autor == libro_filtro.autor &&
                        tabla[i].genero == libro_filtro.genero {
                    tabla[i].titulo = libro_update.titulo.clone();
                    tabla[i].autor = libro_update.autor.clone();
                    tabla[i].genero = libro_update.genero.clone();
                    cont = cont + 1;
                }
                i = i + 1;
            }
            println!("Se actualizó {} registro(s) de la tabla Libro", cont.to_string());
            return tabla;
        }

        /**
         * DELETE
         * 
         * La función eliminará un registro de la tabla de libro, se debe proporcionar 
         * la tabla de referencia después indicar la posición o indice del registro que se desea eliminar.
         * 
         * Este metodo no podrá eliminar varios registros
         * 
         *      Por ejemplo: delete_by_id(tabla, id)
         */
        pub fn delete_by_id(mut tabla: Vec<Libro>, id: i64) -> Vec<Libro> {
            if (id as usize) < tabla.len() {
                tabla.remove(id as usize);
                println!("Se eliminó un registro de la tabla Libro");
            } else {
                println!("El índice sobrepasa al tamaño de la tabla Libro");
            }
            return tabla;
        }
    }

    // tabla lector
    pub mod lector {

        // definición de la estructura lector
        pub struct Lector {
            pub nombre      :String,
            pub direccion   :String,
            pub telefono    :String
        }

        /**
         * Lenguaje de definición de datos (DDL)
         * 
         * En la simulación solo contará con la función de 
         * crear una tabla y es porque en un struct no se puede
         * añadir o eliminar sus campos.
         * 
         * create_table: Permite crear una tabla de un tamaño fijo
         * según la estructura definida.
         */
        pub fn create_table(tamanio: i64) -> Vec<Lector> {
            let tabla: Vec<Lector> = Vec::with_capacity(tamanio as usize);
            println!("La tabla Lector ha sido creada");
            return tabla;
        }


        /**
         * Lenguaje de manipulación de datos (DML)
         * 
         * Se simulará las siguientes sentencias de una base de datos:
         *  SELECT
         *  INSERT
         *  UPDATE
         *  DELETE
         */

        /**
         * SELECT
         * 
         * La función imprimira los registros encontrados, se debe proporcionar
         * la tabla de referencia después indicar la posición o indice del 
         * registro que se desea seleccionar.
         * 
         *      Por ejemplo: select_by_id(tabla, id)
         */
        pub fn select_by_id(tabla: Vec<Lector>, id: usize) -> Lector {
            println!("Se seleccionó un registro de la tabla Lector");

            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "nombre", "direccion", "telefono");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "------------------", "------------------", "------------------");
            
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                tabla[id].nombre, tabla[id].direccion, tabla[id].telefono);

            return Lector {
                nombre: tabla[id].nombre.clone(),
                direccion: tabla[id].direccion.clone(),
                telefono: tabla[id].telefono.clone()
            };
        }

        /**
         * SELECT
         * 
         * La función imprimira todos los registros, se debe proporcionar
         * la tabla de referencia.
         * 
         *      Por ejemplo: select(tabla, id)
         */
        pub fn select(tabla: Vec<Lector>) {
            println!("Se seleccionó todos los registros de la tabla Lector");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "nombre", "direccion", "telefono");
            println!(
                "| {0: <50} | {1: <50} | {2: <50}",
                "------------------", "------------------", "------------------");
            for reg in tabla {
                println!(
                    "| {0: <50} | {1: <50} | {2: <50}",
                    reg.nombre, reg.direccion, reg.telefono);
            }
        }

        /**
         * INSERT
         * 
         * La función permite insertar un nuevo registro, se debe proporcionar 
         * la tabla de referencia luego se debe de agregar los valores del registro. 
         * Esta función retornara la referencia de la tabla.
         * 
         *      Por ejemplo: insert(tabla, valor_campo1, valor_campo2, ..., valor_campoN)
         */
        pub fn insert(mut tabla: Vec<Lector>, nombre: String, direccion: String, telefono: String) -> Vec<Lector> {
            if tabla.len() < tabla.capacity() {
                let registro = Lector {
                    nombre: nombre,
                    direccion: direccion,
                    telefono: telefono
                };

                tabla.push(registro);
                println!("Se insertó un registro en la tabla Lector");
            }
            else {
                println!("La tabla Lector ha llegado a su maxima capacidad");
            }
            return tabla;
        }

        /**
         * UPDATE
         * 
         * La función modificará los valores de un registro, se debe proporcionar 
         * la tabla para actualizar el registro luego se envía una estructura del mismo tipo para 
         * actualizar los campos.
         * 
         * Si el identificador es negativo se actualizara todos los campos
         *      Por ejemplo: update(tabla, id, estructura)
         * 
         * De esta función también puede nacer funciones derivadas tales como update_by_filter
         */
        pub fn update(mut tabla: Vec<Lector>, id: i64, lector_update :Lector) -> Vec<Lector> {
            if id >= 0 {
                tabla[id as usize].nombre = lector_update.nombre;
                tabla[id as usize].direccion = lector_update.direccion;
                tabla[id as usize].telefono = lector_update.telefono;
                println!("Se actualizó un registro de la tabla Lector");
            } else {
                let mut i = 0;
                while i < tabla.len() {
                    tabla[i].nombre = lector_update.nombre.clone();
                    tabla[i].direccion = lector_update.direccion.clone();
                    tabla[i].telefono = lector_update.telefono.clone();
                    i = i + 1;
                }
                println!("Se actualizó {} registro(s) de la tabla Lector", i.to_string());
            }
            return tabla;
        }

        /**
         * UPDATE
         * 
         * Se envia como parametro la tabla de referencia, el libro que simulará el filtro y
         * y el libro que tiene los campos de actualización.
         * 
         * Esta función actualizará todos los registros según el filtro.
         */
        pub fn update_by_filter(mut tabla: Vec<Lector>, lector_filtro: Lector, lector_update :Lector) -> Vec<Lector> {
            let mut i = 0;
            let mut cont = 0;
            while i < tabla.len() {
                if tabla[i].nombre == lector_filtro.nombre &&
                    tabla[i].direccion == lector_filtro.direccion &&
                        tabla[i].telefono == lector_filtro.telefono {
                    tabla[i].nombre = lector_update.nombre.clone();
                    tabla[i].direccion = lector_update.direccion.clone();
                    tabla[i].telefono = lector_update.telefono.clone();
                    cont = cont + 1;
                }
                i = i + 1;
            }
            println!("Se actualizó {} registro(s) de la tabla Lector", cont.to_string());
            return tabla;
        }

        /**
         * DELETE
         * 
         * La función eliminará un registro de la tabla de libro, se debe proporcionar 
         * la tabla de referencia después indicar la posición o indice del registro que se desea eliminar.
         * 
         * Este metodo no podrá eliminar varios registros
         * 
         *      Por ejemplo: delete_by_id(tabla, id)
         */
        pub fn delete_by_id(mut tabla: Vec<Lector>, id: i64) -> Vec<Lector> {
            if (id as usize) < tabla.len(){
                tabla.remove(id as usize);
                println!("Se eliminó un registro de la tabla Lector");
            } else {
                println!("El indice sobrepasa al tamanio de la tabla Lector");
            }
            return tabla;
        }
    }
}

fn registrar_libros(mut tabla: Vec<biblioteca::libro::Libro>) -> Vec<biblioteca::libro::Libro> {
    // Libros almacenados en array
    let libros = [
        ["The Big Data Agenda: Data Ethics and Critical Data Studies", "Annika Richterich", "Algoritmos"],
        ["Computacion Distribuida basada en Objetivos", "Javier Palanca Cámara", "Arquitectura"],
        ["Computational Biology and Applied Bioinformatics", "Prof. Heitor Lopes", "Robotica"],
        ["Implementing a One Address CPU in Logisim", "Charles W. Kann", "Electronica"]
    ];

    // Registrar libros en la base de datos
    for libro in libros {
        tabla = biblioteca::libro::insert(tabla, libro[0].to_string(), libro[1].to_string(), libro[2].to_string());
    }

    return tabla
}

fn registrar_lectores(mut tabla: Vec<biblioteca::lector::Lector>) -> Vec<biblioteca::lector::Lector> {
    // Lectores almacenados en array
    let lectores = [
        ["Jhonatan López", "zona 1", "40000000"],
        ["Alex Jerónimo", "zona 2", "40000001"],
        ["Pablo Roca", "zona 3", "4000003"]
    ];

    // Registrar lectores en la base de datos
    for lector in lectores {
        tabla = biblioteca::lector::insert(tabla, lector[0].to_string(), lector[1].to_string(), lector[2].to_string());
    }
    return tabla
}

fn main() {
    // Tablas de base de datos
    let mut tabla_libro: Vec<biblioteca::libro::Libro> = Vec::new();
    let mut tabla_lector: Vec<biblioteca::lector::Lector> = Vec::new();

    // tabla de libro
    tabla_libro = biblioteca::libro::create_table(10);
    tabla_libro = registrar_libros(tabla_libro);
    tabla_libro = biblioteca::libro::update(tabla_libro, 0,
        biblioteca::libro::Libro { 
            titulo: "Nuevo titulo".to_string(), 
            autor: "Sergio".to_string(), 
            genero: "Literario".to_string()
        });
    tabla_libro = biblioteca::libro::update_by_filter(tabla_libro,
        biblioteca::libro::Libro {
            titulo: "Nuevo titulo".to_string(), 
            autor: "Sergio".to_string(), 
            genero: "Literario".to_string()
        },
        biblioteca::libro::Libro {
            titulo: "Compiladores principios, técnica y herramientas".to_string(),
            autor: "Alfred V. Aho".to_string(),
            genero: "Educación".to_string()
        });
    tabla_libro = biblioteca::libro::delete_by_id(tabla_libro, 1);
    println!("");
    biblioteca::libro::select(tabla_libro);

    // tabla de lector
    println!("");
    println!("");
    println!("");
    tabla_lector = biblioteca::lector::create_table(10);
    tabla_lector = registrar_lectores(tabla_lector);
    tabla_lector = biblioteca::lector::update(tabla_lector, 0,
        biblioteca::lector::Lector { 
            nombre: "Rafa".to_string(), 
            direccion: "zona10".to_string(), 
            telefono: "56523154".to_string()
        });
    tabla_lector = biblioteca::lector::delete_by_id(tabla_lector, 2);
    println!("");
    biblioteca::lector::select(tabla_lector);


    struct Vehiculo {
        modelo: String,
        marca: String
    };
    
    let mut auto: Vehiculo = Vehiculo {
        modelo: "SW42021".to_string(),
        marca: "Toyota".to_string()
    };

    auto.marca = "Mazda".to_string();
    println!("{}", auto.modelo);
}