use crate::common::io;
use slab_tree::*;

fn get_answer(filename: &str, marker_size: usize) {
    let mut lines = io::read_file_as_vector(filename).expect("Could not read file");

    let root_file = File { name: "/".to_string(), file_type: FileType::Directory };
    let mut directory_tree: Tree<File> = TreeBuilder::new().with_root(root_file).build();

    let mut current_node = directory_tree.root_id().expect("Should contain root node");
    lines[1..].iter().for_each(|line| -> () {
        // current_node = parse_line(line, &current_node, &mut directory_tree)
        parse_line(line, &current_node, &mut directory_tree)
    });
}

fn parse_line(line: &str, current_node: &NodeId, tree: &mut Tree<File>) -> () /*NodeId*/ {
    let tokens = line.split(" ").collect::<Vec<_>>();

    match tokens[0] {
        "$" => {
            // cd: move to other node
            // ls: return current nodeId
        },
        "dir" => {
            // new directory node
        },
        _ => {
            // new file node
        },
    }
}

enum Command {
    Cd(String),
    Ls,
}

struct File {
    name: String,
    file_type: FileType,
}

enum FileType {
    Directory,
    DataFile(usize),
}

impl File {
    fn dir_from(name: &str) -> File {
        File {
            name: name.to_string(),
            file_type: FileType::Directory,
        }
    }

    fn datafile_from(name: &str, size: usize) -> File {
        File {
            name: name.to_string(),
            file_type: FileType::DataFile(size),
        }
    }
}


//
//
//
#[cfg(test)]

#[test]
fn tree_test() {
    let root = File { name: "/".to_string(), file_type: FileType::Directory };
    let directory_tree: Tree<File> = TreeBuilder::new().with_root(root).build();

    // let root_mut = directory_tree.root_mut().unwrap();
    // root_mut.node_id().
    // root_mut.append(data)

    println!("{:?}", directory_tree.root_id().unwrap());

}
