use std::sync::Arc;

// Paso 1: Definir la Interfaz del Producto
trait Bebida {
    // Producto -> Bebida
    // Interfaz del Producto
    fn descripcion(&self) -> String; // operation -> descripcion
}

// Paso 2: Implementar Bebidas Concretas
struct Cafe; // Producto Concreto A -> Cafe
impl Bebida for Cafe {
    fn descripcion(&self) -> String {
        "Cafe: bebida que se obtiene mediante el percolado de agua caliente a través de los granos tostados y molidos de los frutos de la planta del café (cafeto); es altamente estimulante por su contenido de cafeína, una sustancia psicoactiva.".to_string()
        // ConcreteProductA -> Cafe
    }
}

struct Te; // Producto Concreto B -> Te
impl Bebida for Te {
    fn descripcion(&self) -> String {
        "Te:  infusión de las hojas y brotes de la planta del té (Camellia sinensis).".to_string()
        // ConcreteProductB -> Te
    }
}

// Paso 3: Crear el Método de Fábrica
trait Barista {
    // Creador -> Barista
    // Interfaz del Creador
    fn preparar_bebida(&self) -> Arc<dyn Bebida>; // factory_method -> preparar_bebida
}

struct BaristaCafe; // Creador Concreto A -> BaristaCafe
impl Barista for BaristaCafe {
    fn preparar_bebida(&self) -> Arc<dyn Bebida> {
        // Implementación del Método de Fábrica para el BaristaCafe
        Arc::new(Cafe) // ConcreteCreatorA -> BaristaCafe
    }
}

struct BaristaTe; // Creador Concreto B -> BaristaTe
impl Barista for BaristaTe {
    fn preparar_bebida(&self) -> Arc<dyn Bebida> {
        // Implementación del Método de Fábrica para el BaristaTe
        Arc::new(Te) // ConcreteCreatorB -> BaristaTe
    }
}

// Uso
fn main() {
    let baristas: Vec<Arc<dyn Barista>> = vec![
        // Usando la Interfaz del Creador
        Arc::new(BaristaCafe),
        Arc::new(BaristaTe),
    ];

    for barista in baristas {
        let bebida = barista.preparar_bebida(); // Llamando al Método de Fábrica
        println!("{}", bebida.descripcion()); // Usando la Bebida
    }
}
