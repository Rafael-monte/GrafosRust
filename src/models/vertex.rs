pub struct Vertex {
    value: String,
    adj: Vec<Box<Vertex>>
}


impl Vertex {
    pub fn from(line: &String) -> Vertex {
        let (vertex_value_index, adj_value_index): (usize, usize) = (0, 1);
        let line_formatted = line.replace("\n", "");
        let elements: Vec<&str> = line_formatted.split(" ").collect();
            if elements.len() != 2 {
            panic!("{}", format!("ERRO: A linha informada não possui o numero de parâmetros experado\n Esperado 2, encontrado {}", line.len()));
        }
        return Vertex {
            value: String::from(elements[vertex_value_index]),
            adj: vec![Box::from(Vertex {
                value: String::from(elements[adj_value_index]),
                adj: vec![]
            })]
        }
    }

    pub fn add_vertex_connection(&mut self, other_vertex: Vertex) {
        self.adj.push(Box::from(other_vertex));
    }

    pub fn get_value(&self) -> &String {
        return &self.value;
    }
}