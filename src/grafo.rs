use petgraph::graph::UnGraph;

// Definición técnica del Grafo para el equipo
pub type RedSocial = UnGraph<&'static str, u32>;

/// Inicializa el grafo vacío. Se eligió UnGraph porque la amistad es mutua.
pub fn crear_grafo() -> RedSocial {
    println!("Iniciando infraestructura de la Red Social..."); 
    // Usamos default() o new() para inicializar el grafo correctamente
    UnGraph::default()
}

/// Función auxiliar para que el Compañero 5 pueda imprimir el grafo fácilmente
pub fn obtener_descripcion_tecnica() -> &'static str {
    "Tipo de Grafo: No dirigido (UnGraph) | Librería: Petgraph"
}