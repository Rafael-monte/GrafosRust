use std::fs::File;
use std::{fs};
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use crate::models::graph::GraphType::{Directed, Undirected};
use crate::models::vertex::Vertex;

pub enum GraphType {
    Directed,
    Undirected
}

pub struct Graph {
    vertexes: Vec<Box<Vertex>>,
    graph_type: GraphType
}



impl Graph {
    pub fn get_graph_type(self) -> GraphType {
        return self.graph_type;
    }

    pub fn new() -> Graph {
        return Graph{
            vertexes: vec![],
            graph_type: Undirected
        }
    }

    pub fn from(file_path: &Path) -> Graph {
        let file = File::open(file_path).expect("Erro ao abrir arquivo de grafos");
        return Graph::proccess_file(&file);
    }

    fn proccess_file(file: &File) -> Graph {
        let buffer = BufReader::new(file);
        return Graph::build_graph_by_buffer(buffer);
    }

    fn build_graph_by_buffer(buffer: BufReader<&File>) -> Graph {
        const FIRST_ELEMENT_INDEX: usize = 0;
        let mut graph = Graph::new();
        let mut lines = buffer.lines().into_iter().map(|el| {
            el.expect("Não foi possivel converter a linha para string")
        }).collect::<Vec<String>>();
        let first_line = lines.get(FIRST_ELEMENT_INDEX).expect("Arquivo de grafos não possui nenhuma linha");
        graph.graph_type = Graph::process_graph_type_by_line(first_line);
        lines.remove(FIRST_ELEMENT_INDEX);
        lines.iter().for_each(|line|{
            graph.vertexes.push(Box::from(Vertex::from(line)))
        });
        return graph;
    }

    fn process_graph_type_by_line(line: &String) -> GraphType {
        match line.replace("\n", "").as_str() {
            "directed" => return Directed,
            "undirected" => return Undirected,
            _ => panic!("{}", format!("Opcao de grafo desconhecida"))
        }
    }

    pub fn contains_vertex(&self, vertex: Box<Vertex>) -> bool {
        return self.vertexes.iter().any(|el| el.get_value() == vertex.get_value());
    }
}