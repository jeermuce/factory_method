Proyecto: Patrón de Diseño Método de Fábrica

# Descripción del Proyecto

Este proyecto implementa el patrón de diseño "Método de Fábrica" en Rust. El objetivo es demostrar cómo se puede utilizar este patrón para crear objetos sin especificar la clase exacta del objeto que se creará. En este caso, el proyecto se centra en la creación de diferentes tipos de bebidas (Café y Té) mediante el uso de baristas (BaristaCafé y BaristaTé).

# Compilar el program

Requisitos:
[Rust](https://www.rust-lang.org/tools/install])

Navegar en la consola/terminal al directorio raiz del projecto y usar este comando.

```shell
cargo build --release
```

# Correr el programa

doble click en `./executables/factory_method.exe` para windows

o en la terminal
linux

```shell
./factory_method
```

windows

```shell
./factory_method.exe
```

o, si se tiene rust instalado y se quiere informacion de depuracion

```shell
cargo run
```

## Estructura del Código

### Paso 1: Definir la Interfaz del Producto

Se define una interfaz (trait) llamada `Bebida` que declara un método `descripcion` que debe ser implementado por todas las bebidas concretas.

```rust
use std::sync::Arc;

trait Bebida {
    fn descripcion(&self) -> String;
}
```

### Paso 2: Implementar Bebidas Concretas

Se implementan dos estructuras concretas (`struct`), `Café` y `Té`, que representan dos tipos de bebidas. Ambas estructuras implementan la interfaz `Bebida`.
struct Cafe;

```rust
impl Bebida for Cafe {
    fn descripcion(&self) -> String {
        "Cafe: bebida que se obtiene mediante el percolado de agua caliente a través de los granos tostados y molidos de los frutos de la planta del café (cafeto); es altamente estimulante por su contenido de cafeína, una sustancia psicoactiva.".to_string()
    }
}

struct Te;

impl Bebida for Te {
    fn descripcion(&self) -> String {
        "Te: infusión de las hojas y brotes de la planta del té (Camellia sinensis).".to_string()
    }
}
```

### Paso 3: Crear el Método de Fábrica

Se define una interfaz (trait) llamada `Barista` que declara un método `preparar_bebida` que debe ser implementado por todos los baristas concretos.

```rust
trait Barista {
    fn preparar_bebida(&self) -> Arc<dyn Bebida>;
}
```

Se implementan dos estructuras concretas (`struct`), `BaristaCafé` y `BaristaTé`, que representan dos tipos de baristas. Ambas estructuras implementan la interfaz `Barista`.

```rust
struct BaristaCafe;

impl Barista for BaristaCafe {
    fn preparar_bebida(&self) -> Arc<dyn Bebida> {
        Arc::new(Cafe)
    }
}

struct BaristaTe;

impl Barista for BaristaTe {
    fn preparar_bebida(&self) -> Arc<dyn Bebida> {
        Arc::new(Te)
    }
}
```

## Uso del Patrón

En la función `main`, se crea una lista de baristas y se utiliza el método `preparar_bebida` para obtener y describir las bebidas preparadas por cada barista.

```rust
fn main() {
    let baristas: Vec<Arc<dyn Barista>> = vec![
        Arc::new(BaristaCafe),
        Arc::new(BaristaTe),
    ];

    for barista in baristas {
        let bebida = barista.preparar_bebida();
        println!("{}", bebida.descripcion());
    }
}
```

## Retroalimentación

### Funcionalidad

El código implementa correctamente el patrón de diseño "Método de Fábrica". La interfaz `Bebida` y las implementaciones concretas `Café` y `Té` están bien definidas. Los baristas `BaristaCafé` y `BaristaTé` utilizan el método de fábrica `preparar_bebida` para crear instancias de `Café` y `Té`, respectivamente.

### Situaciones de Uso

Este patrón es útil en situaciones donde se necesita crear objetos de diferentes tipos sin especificar la clase exacta del objeto que se creará. Por ejemplo, en una aplicación de cafetería, se podría utilizar este patrón para crear diferentes tipos de bebidas (café, té, chocolate caliente) sin cambiar el código del cliente.

### Situaciones en las que No se Utilizaría

No es recomendable utilizar este patrón cuando el número de tipos de productos es fijo y conocido de antemano, ya que puede añadir complejidad innecesaria. En tales casos, una simple creación de objetos podría ser suficiente.
