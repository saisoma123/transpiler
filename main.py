import sys
from antlr4 import *
from Python3Lexer import Python3Lexer
from Python3Parser import Python3Parser
from Py2RsCodeGenListener import Py2RsCodeGenListener


def main(argv):
    input = FileStream("stat.py")
    lexer = Python3Lexer(input)
    stream = CommonTokenStream(lexer)
    parser = Python3Parser(stream)
    tree = parser.file_input()

    with open("output.rs","w") as output:
        js2pyListen = Py2RsCodeGenListener(output)
        walker = ParseTreeWalker()
        walker.walk(js2pyListen, tree)
        
    output.close()      

if __name__ == '__main__':
    main(sys.argv)