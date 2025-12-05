use super::LIST_GRAPH;
use std::collections::{HashSet, VecDeque};

/// Implementa el algoritmo de Búsqueda en Anchura (BFS - Breadth-First Search)
/// sobre el grafo estático LIST_GRAPH definido en el módulo padre.
///
/// BFS es un algoritmo de recorrido que explora los nodos por niveles,
/// visitando primero todos los vecinos directos de un nodo antes de pasar
/// a los vecinos de esos vecinos.
///
/// # Características:
/// - Evita ciclos usando un conjunto de nodos visitados
/// - Utiliza una cola (FIFO) para mantener el orden de exploración
/// - Comienza desde el nodo 0
/// - Imprime el progreso del recorrido en consola
pub fn bfs() {
    unsafe {
        // Accedemos de forma segura a la variable estática mutable LIST_GRAPH
        if let Some(ref graph) = LIST_GRAPH {
            // HashSet para almacenar los nodos que ya hemos visitado
            // Esto previene que revisitemos el mismo nodo múltiples veces
            // y evitamos caer en ciclos infinitos
            let mut visited: HashSet<usize> = HashSet::new();

            // VecDeque implementa una cola FIFO (First In, First Out)
            // Usamos push_back() para añadir elementos al final
            // y pop_front() para extraer elementos del principio
            // Esto garantiza que exploramos los nodos en orden de nivel (anchura)
            let mut queue: VecDeque<usize> = VecDeque::new();

            // Definimos el nodo inicial desde donde comenzará el BFS
            let start_node = 0;

            // Añadimos el nodo inicial a la cola para comenzar la exploración
            queue.push_back(start_node);

            // Marcamos el nodo inicial como visitado para no procesarlo nuevamente
            visited.insert(start_node);

            // Imprimimos un mensaje indicando el inicio del algoritmo BFS
            println!("BFS comenzando desde nodo {}: ", start_node);

            // Bucle principal: continúa mientras haya nodos en la cola
            // pop_front() extrae y retorna Some(nodo) si la cola no está vacía
            // o None si la cola está vacía, terminando así el bucle
            while let Some(node) = queue.pop_front() {
                // Imprimimos el nodo actual que estamos visitando
                println!("Visitando nodo: {}", node);

                // Obtenemos la lista de vecinos (nodos adyacentes) del nodo actual
                // graph.get(&node) retorna Option<&Vec<usize>>
                if let Some(neighbors) = graph.get(&node) {
                    // Iteramos sobre cada vecino del nodo actual
                    for &neighbor in neighbors {
                        // Verificamos si el vecino ya ha sido visitado
                        // Si ya fue visitado, lo ignoramos para evitar ciclos
                        if !visited.contains(&neighbor) {
                            // Marcamos el vecino como visitado
                            // Esto asegura que no lo procesaremos más de una vez
                            visited.insert(neighbor);

                            // Añadimos el vecino a la cola para explorarlo después
                            // Esto mantiene el orden FIFO del BFS
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            // Imprimimos un mensaje de finalización con todos los nodos visitados
            // El conjunto visited contiene exactamente los nodos alcanzables desde el nodo inicial
            println!("BFS completado. Nodos visitados: {:?}", visited);
        } else {
            // Si la variable estática LIST_GRAPH es None, significa que no fue inicializada
            // Esto indica un error en la configuración previa del grafo
            println!("El grafo no ha sido inicializado");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::graphs::basics::fill_list_graph;

    use super::*;

    /// Test que verifica el comportamiento de BFS cuando el grafo NO ha sido inicializado.
    ///
    /// # Propósito:
    /// Validar que la función bfs() maneje correctamente el caso de error donde
    /// la variable estática LIST_GRAPH es None (no inicializada).
    ///
    /// # Comportamiento esperado:
    /// - La función debe detectar que LIST_GRAPH es None
    /// - Debe imprimir un mensaje de error: "El grafo no ha sido inicializado"
    /// - No debe entrar en pánico (panic) ni causar un comportamiento indefinido
    /// - Debe terminar de forma segura sin ejecutar el algoritmo BFS
    ///
    /// # Caso de uso:
    /// Asegura que si alguien olvida llamar a fill_list_graph() antes de bfs(),
    /// el programa informará correctamente del problema en lugar de fallar silenciosamente
    #[test]
    fn test_sin_iniciar() {
        // Llamamos a bfs() sin haber inicializado el grafo previamente
        // Esto simula un error de configuración o un uso incorrecto de la API
        // Esperamos que imprima un mensaje de error en la consola
        bfs();
        // Nota: Este test simplemente verifica que no hay pánico
        // En una versión mejorada, podríamos capturar la salida estándar
        // para verificar que el mensaje de error se imprimió correctamente
    }

    /// Test que verifica el funcionamiento completo de BFS con un grafo inicializado correctamente.
    ///
    /// # Propósito:
    /// Validar que la función bfs() ejecuta correctamente el algoritmo BFS
    /// cuando el grafo ha sido inicializado previamente con fill_list_graph().
    ///
    /// # Flujo del test:
    /// 1. Llama a fill_list_graph() para inicializar la variable estática LIST_GRAPH
    ///    con un grafo que contiene 5 nodos (0, 1, 2, 3, 4) y múltiples ciclos
    /// 2. Llama a bfs() para ejecutar el algoritmo de búsqueda en anchura
    /// 3. El algoritmo debe recorrer todos los nodos alcanzables desde el nodo 0
    ///
    /// # Comportamiento esperado:
    /// - La función bfs() debe inicializarse correctamente
    /// - Debe imprimir: "BFS comenzando desde nodo 0: "
    /// - Debe visitar cada nodo exactamente una vez (evitando ciclos)
    /// - Debe explorar los nodos en orden de anchura (nivel por nivel)
    /// - Debe imprimir: "Visitando nodo: X" para cada nodo visitado
    /// - Debe terminar con: "BFS completado. Nodos visitados: {0, 1, 2, 3, 4}"
    ///   (el orden exacto puede variar según la implementación del HashSet)
    ///
    /// # Caso de uso:
    /// Valida que el algoritmo BFS funciona correctamente en el caso de uso normal
    /// donde el grafo está correctamente inicializado y listo para ser explorado
    #[test]
    fn test_con_fill() {
        // Inicializamos el grafo estático con una estructura predefinida
        // fill_list_graph() inserta 5 nodos con aristas que crean múltiples ciclos:
        // - 0 -> [1, 2]
        // - 1 -> [0, 3]
        // - 2 -> [1, 3]
        // - 3 -> [2, 4]
        // - 4 -> [0, 3]
        fill_list_graph();

        // Ejecutamos el algoritmo BFS sobre el grafo inicializado
        // La función debe:
        // 1. Comenzar desde el nodo 0
        // 2. Explorar todos los nodos conectados en orden FIFO
        // 3. Evitar revisitar nodos (usar el HashSet de visitados)
        // 4. Manejar correctamente los ciclos del grafo
        bfs();

        // Nota: Este test verifica principalmente que:
        // - No hay pánico durante la ejecución
        // - El algoritmo termina correctamente
        // - La salida se imprime en la consola (observable en la salida del test)
        //
        // Para un test más robusto, podríamos:
        // - Capturar la salida estándar
        // - Verificar que exactamente 5 nodos fueron visitados
        // - Confirmar el orden de exploración
        // - Validar que no hay duplicados en los nodos visitados
    }
}
