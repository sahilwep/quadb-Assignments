#include<bits/stdc++.h>
using namespace std;
typedef long long ll;

struct Node{
    int key;
    Node *left, *right;
    Node(int k){
        key = k;
        left = right = NULL;
    }
};

// depth of binary Tree: 
int depth(Node *root){
    if(root == NULL) return 0;
    return max(depth(root->left), depth(root->right)) + 1;
}

// preOrder traversal:
void preOrder(Node *root){
    if(root!=NULL){
        cout << (root->key) << " ";
        preOrder(root->left);
        preOrder(root->right);
    }
}

int main(void){
    
    Node *root = new Node(10);
    root->left = new Node(8);
    root->right = new Node(30);
    root->right->left = new Node(40);
    root->right->right = new Node(50);
    
    preOrder(root); // preorder traversal:
    cout << endl;
    cout << depth(root) << endl; // depth of binary tree

    return 0;
}