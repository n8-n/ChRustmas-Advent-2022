use std::collections::HashMap;
use crate::common::io;
use slab_tree::*;

const SIZE_LIMIT: usize = 100000;

pub fn get_sum_of_large_directories(filename: &str) -> usize {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    let mut directory_tree: Tree<File> = init_tree();

    let mut current_node = directory_tree.root_id().expect("Should contain root node");
    lines[1..].iter().for_each(|line| {
        let tree_info = TreeInfo {
            current_node_id: &current_node,
            tree: &mut directory_tree,
        };
        current_node = parse_line_add_to_tree(line, tree_info)
    });

    let dir_sizes = get_size_of_dirs(&directory_tree);

    dir_sizes.values().fold(0, |total, size| {
        total + (if *size <= SIZE_LIMIT { *size } else { 0 })
    })
}

fn parse_line_add_to_tree(line: &str, tree_info: TreeInfo) -> NodeId {
    let tokens = line.split(" ").collect::<Vec<_>>();
    let current_node_id = *tree_info.current_node_id;

    match tokens[0] {
        "$" => {
            let command = tokens[1];
            if command == "cd" {
                let location = tokens[2];
                if location == ".." {
                    return get_parent_node_id(tree_info);
                } else {
                    return create_node_if_not_exist(location, FileType::Directory, tree_info);
                }
            }
        }
        "dir" => {
            create_node_if_not_exist(tokens[1], FileType::Directory, tree_info);
        }
        _ => {
            let size = tokens[0].parse().expect("Should parse int");
            create_node_if_not_exist(tokens[1], FileType::DataFile(size), tree_info);
        }
    }

    current_node_id
}

fn create_node_if_not_exist(name: &str, file_type: FileType, tree_info: TreeInfo) -> NodeId {
    let current_node = tree_info
        .tree
        .get(*tree_info.current_node_id)
        .expect("Should be a node");

    let child_node = current_node.children().find(|n| n.data().name == name);

    match child_node {
        Some(child_node) => child_node.node_id(),
        None => {
            let new_file = File {
                name: name.to_string(),
                file_type,
            };
            let mut current_node = tree_info
                .tree
                .get_mut(*tree_info.current_node_id)
                .expect("Should be a node");
            current_node.append(new_file).node_id()
        }
    }
}

fn get_parent_node_id(tree_info: TreeInfo) -> NodeId {
    let current_node = tree_info.tree.get(*tree_info.current_node_id).unwrap();
    let parent_node = current_node
        .ancestors()
        .next()
        .expect("Should have ancestor");
    parent_node.node_id()
}

fn init_tree() -> Tree<File> {
    let root_file = File {
        name: "/".to_string(),
        file_type: FileType::Directory,
    };
    TreeBuilder::new().with_root(root_file).build()
}

fn get_size_of_dirs(tree: &Tree<File>) -> HashMap<DirId, usize> {
    let mut sizes = HashMap::new();

    tree.root()
        .expect("Should not be empty")
        .traverse_post_order()
        .for_each(|node| {
            let file = node.data();
            match file.file_type {
                FileType::DataFile(_) => (),
                FileType::Directory => {
                    let id = DirId::new(file.name.to_string(), node.node_id());
                    let size = sum_files_in_dir(node, &sizes);
                    sizes.insert(id, size);
                }
            }
        });

    sizes
}

fn sum_files_in_dir(dir_node: NodeRef<File>, dir_sizes: &HashMap<DirId, usize>) -> usize {
    dir_node
        .children()
        .fold(0, |total, file| match file.data().file_type {
            FileType::DataFile(size) => total + size,
            FileType::Directory => {
                let id = DirId::new(file.data().name.to_string(), file.node_id());
                let child_dir_size = dir_sizes.get(&id).unwrap_or(&0);
                total + *child_dir_size
            }
        })
}

struct TreeInfo<'a> {
    current_node_id: &'a NodeId,
    tree: &'a mut Tree<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    file_type: FileType,
}

#[derive(Debug)]
enum FileType {
    Directory,
    DataFile(usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct DirId {
    name: String,
    id: NodeId
}

impl DirId {
    fn new(name: String, id: NodeId) -> DirId {
        DirId {
            name, id
        }
    }
}

//
//
//
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_create_child_node() {
        let mut directory_tree: Tree<File> = init_tree();
        let root_id = &directory_tree.root_id().unwrap();
        
        // first time create new file node, second time skip
        for _ in 1..=2 {
            let tree_info = TreeInfo {current_node_id: root_id, tree: &mut directory_tree};
            let new_node_id = create_node_if_not_exist("test1", FileType::Directory, tree_info);
            let root_children: Vec<_> = directory_tree.root().unwrap().children().collect();

            assert_eq!(1, root_children.len());
            assert_eq!(new_node_id, root_children[0].node_id());
        }
    }

    
    #[test]
    fn test_parse_lines_to_tree() {
        let mut directory_tree: Tree<File> = init_tree();
        let root_id = &directory_tree.root_id().unwrap();

        let tree_info = TreeInfo { current_node_id: root_id, tree: &mut directory_tree };
        let result = parse_line_add_to_tree("$ ls", tree_info);
        assert_eq!(*root_id, result);

        let tree_info = TreeInfo { current_node_id: root_id, tree: &mut directory_tree };
        let result = parse_line_add_to_tree("$ cd blah", tree_info);
        assert_eq!(directory_tree.root().unwrap().first_child().unwrap().node_id(), result);

        let node_id = result;
        let tree_info = TreeInfo { current_node_id: &node_id, tree: &mut directory_tree };
        let result = parse_line_add_to_tree("2344 tst", tree_info);
        assert_eq!(node_id, result);
    }

    #[test]
    fn test_get_sum_of_large_directories() {
        let sum = get_sum_of_large_directories("resources/test/07_directories.txt");
        assert_eq!(95437, sum);
    }
}
