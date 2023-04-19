import sys
from antlr4 import *
from Python3Lexer import Python3Lexer
from Python3Parser import Python3Parser
from antlr4.tree.Trees import Trees
 

def print_tree(tree, lev):
    print (" " * lev) + "` " + str(tree)
    for c in tree.getChildren():
        print_tree(c, lev + 1)

def main(argv):
    input_stream = FileStream(argv[1])
    lexer = Python3Lexer(input_stream)
    stream = CommonTokenStream(lexer)
    parser = Python3Parser(stream)
    tree = parser.file_input()
    print(tree.toStringTree(recog=parser))
 
if __name__ == '__main__':
    main(sys.argv)