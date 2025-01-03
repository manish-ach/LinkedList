// #include <cstddef>//for 'NULL' though it can be mistaken with int 0, use nullptr(built in) instead
#include<iostream>

//Node class
class Node {
    public:
    int value;
    Node *link;
    static void presetList();
    static void insertBeginning(int);
    static void insertEnd(int);
    static void insertAfter(int, int);
    void displayList();
};

//setting head node as NULL at start as the linked list is empty
Node *head = nullptr;

//default set of filled linked list with some nodes
void Node::presetList() {
    //initializing the nodes
    Node *node1 = nullptr;
    Node *node2 = nullptr;
    Node *node3 = nullptr;

    //allocating them heap memory
    node1 = new Node();
    node2 = new Node();
    node3 = new Node();

    //assigning value to the nodes
    node1 -> value = 11;
    node2 -> value = 22;
    node3 -> value = 33;

    //connecting nodes
    node1 -> link = node2;
    node2 -> link = node3;
    node3 -> link = nullptr;

    //setting head
    head = node1;
    Node *temp = head;
    while (temp != nullptr) {
        std::cout << temp->value << " ";
        temp = temp -> link;
    }
}

//add a node to the beginning of the list
void Node::insertBeginning(int value) {
    Node *newNode;
    newNode = new Node();
    newNode -> value = value;
    if (head == nullptr) {
        newNode -> link = nullptr;
        head = newNode;
    }
    else {
        newNode -> link = head;
        head = newNode;
    }
}

//add a node at the end of the list
//similar to the insertBeginning();
void Node::insertEnd(int value) {
    Node *newNode;
    newNode = new Node();
    newNode -> value = value;
    newNode -> link = nullptr; //as we know node is to be added at en, we can set its link to null
    if (head == nullptr) {
        head = newNode;
    }
    else {
        Node *temp = head;
        while (temp != nullptr) {
            if (temp -> link == nullptr) {
                temp -> link = newNode;
            }
        }
    }
}

//insert after given node
void Node::insertAfter(int value, int location) {
    Node *newNode;
    newNode = new Node();
    newNode -> value = value;
    if ( head == nullptr) {
        newNode -> link = nullptr;
        head = newNode;
    }
    else {
        Node *temp;
        temp = head;
        while (temp -> value != location) {
            temp = temp -> link;
        }
        newNode -> link = temp -> link;
        temp -> link = newNode;
        delete temp;
    }
}

//display the node's value
void Node::displayList() {
    Node *temp = head;
    while (temp -> link != nullptr) {
        std::cout << temp->value << std::endl;
        temp = temp -> link;
    }
}

int main() {
    Node n;
    int choice, value;
    char ch = 'y';
    std::cout << "\t\t***OPTIONS***"<<std::endl;
    std::cout << "1. Have a default preset of linked list available at start \n";
    std::cout << "2. Insert by self(default) \n";
    std::cin >> choice;

    if (choice == 1){
        n.presetList();
    }

    std::cout<<std::endl << "insert into the list now"<<std::endl;
    while (ch == 'y' || ch == 'Y') {
        std::cout << "enter an integer ";
        std::cin >> value;
        n.insertBeginning(value);
        std::cout << "Do you want to continue? (y/n) -> ";
        std::cin >> ch;
    }
    n.displayList();
    return 0;
}
