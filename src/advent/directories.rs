use crate::common::io;
use slab_tree::*;

fn get_answer(filename: &str) {
    let mut lines = io::read_file_as_vector(filename).expect("Could not read file");

    let mut directory_tree: Tree<File> = init_tree();

    let mut current_node = directory_tree.root_id().expect("Should contain root node");
    lines[1..].iter().for_each(|line| -> () {
        let tree_info = TreeInfo {
            current_node_id: &current_node,
            tree: &mut directory_tree,
        };
        current_node = parse_line_add_to_tree(line, tree_info)
    });

    // directory_tree.root().unwrap().children().for_each(f)
}

fn parse_line_add_to_tree(line: &str, tree_info: TreeInfo) -> NodeId {
    let tokens = line.split(" ").collect::<Vec<_>>();

    match tokens[0] {
        "$" => {
            let command = tokens[1];
            if command == "cd" {
                create_child_node(tokens[2], FileType::Directory, tree_info)
            } else {
                *tree_info.current_node_id
            }
        }
        "dir" => create_child_node(tokens[1], FileType::Directory, tree_info),
        _ => {
            let size = tokens[0].parse().expect("Should parse int");
            create_child_node(tokens[1], FileType::DataFile(size), tree_info)
        }
    }
}

fn create_child_node(name: &str, file_type: FileType, tree_info: TreeInfo) -> NodeId {
    let current_node = tree_info
        .tree
        .get(*tree_info.current_node_id)
        .expect("Should be a node");
    
    let child_node = current_node.children().find(|n| n.data().name == name);

    match child_node {
        Some(child_node) => { 
            println!("Child node exists, no need to create");
            child_node.node_id() 
        }
        None => {
            println!("No child node, creating");
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

fn init_tree() -> Tree<File> {
    let root_file = File {
        name: "/".to_string(),
        file_type: FileType::Directory,
    };
    TreeBuilder::new().with_root(root_file).build()
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

//
//
//
#[rustfmt::skip]
#[cfg(test)]
#[test]
fn test_create_child_node() {
    let mut directory_tree: Tree<File> = init_tree();
    let root_id = &directory_tree.root_id().unwrap();
    
    // first time create new file node, second time skip
    for _ in 1..=2 {
        let tree_info = TreeInfo {current_node_id: root_id, tree: &mut directory_tree};
        let new_node_id = create_child_node("test1", FileType::Directory, tree_info);
        let root_children: Vec<_> = directory_tree.root().unwrap().children().collect();

        assert_eq!(1, root_children.len());
        assert_eq!(new_node_id, root_children[0].node_id());
    }
}

#[test]
fn test_parse_lines_to_tree() {
    let mut directory_tree: Tree<File> = init_tree();
    let root_id = &directory_tree.root_id().unwrap();

    let tree_info = TreeInfo {current_node_id: root_id, tree: &mut directory_tree};
    let result = parse_line_add_to_tree("$ ls", tree_info);
    assert_eq!(*root_id, result);

    let tree_info = TreeInfo {current_node_id: root_id,tree: &mut directory_tree};
    let result = parse_line_add_to_tree("$ cd blah", tree_info);
    assert_eq!(directory_tree.root().unwrap().first_child().unwrap().node_id(), result);

    // check same dir
    let tree_info = TreeInfo {current_node_id: root_id,tree: &mut directory_tree};
    let result = parse_line_add_to_tree("dir blah", tree_info);
    assert_eq!(directory_tree.root().unwrap().first_child().unwrap().node_id(), result);

    let tree_info = TreeInfo {current_node_id: root_id,tree: &mut directory_tree};
    let result = parse_line_add_to_tree("2344 tst", tree_info);
    assert_eq!(directory_tree.root().unwrap().last_child().unwrap().node_id(), result);

    // create child of root child
    let node_id = result;
    let tree_info = TreeInfo {current_node_id: &node_id, tree: &mut directory_tree};
    let result2 = parse_line_add_to_tree("1111 test2", tree_info);
    assert_eq!(directory_tree.get(node_id).unwrap().first_child().unwrap().node_id(), result2);
}
