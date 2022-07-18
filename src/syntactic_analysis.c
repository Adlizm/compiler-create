#include <stdlib.h>
#include <stdio.h>

typedef enum {
    NT_S, NT_BLOCK, NT_VAR_DECLARATIONS,
    NT_COMMAND_SEQUENCE, NT_LIST_IDS, NT_LIST_IDS_,
    NT_COMMAND, NT_SELECTION, NT_SELECTION_,
    NT_REPETITION, NT_ATTRIBUTION, NT_CONDITION,
    NT_EXPRESSION, NT_EXPRESSION_, NT_TERM, NT_TERM_,
    NT_UNITARY, NT_FACTOR
} Non_Terminal;


//------------- Declara��o de procedimentos -------------

int procedureS(SyntacticTree** tree);
int procedureBlock(SyntacticTree** tree);
int procedureVarDeclaratios(SyntacticTree** tree);
int procedureCommandSequence(SyntacticTree** tree);
int procedureListIds(SyntacticTree** tree);
int procedureListIds_(SyntacticTree** tree);
int procedureCommand(SyntacticTree** tree);
int procedureSelection(SyntacticTree** tree);
int procedureSelection_(SyntacticTree** tree);
int procedureRepetition(SyntacticTree** tree);
int procedureAttribution(SyntacticTree** tree);
int procedureCondition(SyntacticTree** tree);
int procedureExpression(SyntacticTree** tree);
int procedureExpression_(SyntacticTree** tree);
int procedureTerm(SyntacticTree** tree);
int procedureTerm_(SyntacticTree** tree);
int procedureUnitary(SyntacticTree** tree);
int procedureFactor(SyntacticTree** tree);

//------------------ Fun��es Auxiliares ------------------
struct Token currentToken;
int row, col;

void error(){
    struct symbol sym;
    switch(currentToken.type){
        case TK_ID: case TK_NUMBER:
            getSymbol(SYMBOLS_TABLE, currentToken.attr, &sym);
            break;
        case TK_EOF:
            sprintf(sym.lexema, "EOF" );
            break;
        case TK_CHAR:
            sprintf(sym.lexema, "%c", currentToken.attr);
            break;
        case TK_RELATIONAL:
            switch(currentToken.attr){
                case OP_EQ: sprintf(sym.lexema, "="); break;
                case OP_NE: sprintf(sym.lexema, "<>"); break;
                case OP_LT: sprintf(sym.lexema, "<"); break;
                case OP_LE: sprintf(sym.lexema, "<="); break;
                case OP_GT: sprintf(sym.lexema, ">"); break;
                case OP_GE: sprintf(sym.lexema, ">="); break;
            }
            break;
        case TK_ARITHMETIC:
            switch(currentToken.attr){
                case OP_ADD: sprintf(sym.lexema, "+"); break;
                case OP_SUB: sprintf(sym.lexema, "-"); break;
                case OP_MUL: sprintf(sym.lexema, "*"); break;
                case OP_DIV: sprintf(sym.lexema, "/"); break;
            }
            break;
        default:
            getTokenString(currentToken, sym.lexema);
            break;
    }
    printf("\nError(%d,%d): '%s' nao esperado", row, col, sym.lexema);
}
int verifyToken(SyntacticTree** tree, int pos, int type){
    /*
    char tokenName[MAX_SIZE_LEXEME + 30];
    getTokenString(currentToken, tokenName);
    printf("\n%s", tokenName);
    */
    if(currentToken.type != type){
        error();
        return 0;
    }
    (*tree)->childs[pos] = createSyntacticTree(currentToken, 0);
    if((*tree)->childs[pos] == NULL)
       return 0;
    return nextToken(&currentToken, &row, &col);
}

//------------------ Fun��es Principais ------------------

SyntacticTree* SYNTACTIC_TREE;

int initSyntaticAnalysis(char *filepath){
    if(!initLexicalAnalysis(filepath))
        return 0;
    if(nextToken(&currentToken, &row, &col)){
        if(!procedureS(&SYNTACTIC_TREE)){
            deleteSyntacticTree(SYNTACTIC_TREE);
            SYNTACTIC_TREE = NULL;
        }
        endAnalysis();
        if(currentToken.type == TK_EOF)
            return 1;
        error();
    }
    return 0;
}
int writeSyntaticTreeFile(char *filepath){
    if(SYNTACTIC_TREE == NULL){
        printf("Error: Nao foi criada uma arvore sintatica");
        return 0;
    }
    return exportSyntacticTree(SYNTACTIC_TREE, filepath);
}

//--------------------- Procedimentos --------------------

int procedureS(SyntacticTree** tree){
    //printf("\nprocedure S");
    struct Token tk;
    tk.attr = -1; tk.type = NT_S;

    *tree = createSyntacticTree(tk, 3);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_PROCEDURE))
    if(verifyToken(tree, 1, TK_PRINCIPAL))
        return procedureBlock(&((*tree)->childs[2]));
    return 0;
}
int procedureBlock(SyntacticTree** tree){
    //printf("\nprocedure Block");
    struct Token tk;
    tk.attr = -1; tk.type = NT_BLOCK;

    *tree = createSyntacticTree(tk, 4);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_INICIO))
    if(procedureVarDeclarations(&((*tree)->childs[1])))
    if(procedureCommandSequence(&((*tree)->childs[2])))
        return verifyToken(tree, 3, TK_FIM);
    return 0;
}
int procedureVarDeclarations(SyntacticTree** tree){
    //printf("\nprocedure VarDeclarations");
    if(currentToken.type != TK_TYPE){
        *tree = NULL;
        return 1;
    }
    struct Token tk;
    tk.attr = -1; tk.type = NT_VAR_DECLARATIONS;
    *tree = createSyntacticTree(tk, 5);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_TYPE))
    if(verifyToken(tree, 1, TK_DOUBLE_POINT))
    if(procedureListIds(&((*tree)->childs[2])))
    if(verifyToken(tree, 3, TK_SEMICOLON))
        return procedureVarDeclarations(&((*tree)->childs[4]));
    return 0;
}
int procedureCommandSequence(SyntacticTree** tree){
    //printf("\nprocedure CommandSequence");
    struct Token tk;
    tk.attr = -1; tk.type = NT_COMMAND_SEQUENCE;
    if(currentToken.type == TK_ID || currentToken.type == TK_IF ||
       currentToken.type == TK_ENQUANTO || currentToken.type == TK_REPITA){
        *tree = createSyntacticTree(tk, 2);
        if(*tree == NULL)
            return 0;

        if(!procedureCommand(&((*tree)->childs[0])))
            return 0;
        return procedureCommandSequence(&((*tree)->childs[1]));
    }
    *tree = NULL;
    return 1;
}
int procedureListIds(SyntacticTree** tree){
    //printf("\nprocedure ListIds");
    struct Token tk;
    tk.attr = -1; tk.type = NT_LIST_IDS;
    *tree = createSyntacticTree(tk, 2);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_ID))
        return procedureListIds_(&((*tree)->childs[1]));
    return 0;
}
int procedureListIds_(SyntacticTree** tree){
    //printf("\nprocedure ListIds'");
    if(currentToken.type != TK_COMMA){
        *tree = NULL;
        return 1;
    }
    struct Token tk;
    tk.attr = -1; tk.type = NT_LIST_IDS_;
    *tree = createSyntacticTree(tk, 2);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_COMMA))
        return procedureListIds(&((*tree)->childs[1]));
    return 0;
}
int procedureCommand(SyntacticTree** tree){
    //printf("\nprocedure Command");
    struct Token tk;
    tk.attr = -1; tk.type = NT_COMMAND;
    *tree = createSyntacticTree(tk, 1);
    if(*tree == NULL)
        return 0;

    switch(currentToken.type){
        case TK_IF:
            return procedureSelection(&((*tree)->childs[0]));
        case TK_ID:
            return procedureAttribution(&((*tree)->childs[0]));
        case TK_ENQUANTO: case TK_REPITA:
            return procedureRepetition(&((*tree)->childs[0]));
        default:
            break;
    }
    error();
    return 0;
}
int procedureSelection(SyntacticTree** tree){
    //printf("\nprocedure Selection");
    struct Token tk;
    tk.attr = -1; tk.type = NT_SELECTION;
    *tree = createSyntacticTree(tk, 7);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_IF))
    if(verifyToken(tree, 1, TK_LEFT_PARENTHESES))
    if(procedureCondition(&((*tree)->childs[2])))
    if(verifyToken(tree, 3, TK_RIGHT_PARENTHESES))
    if(verifyToken(tree, 4, TK_ENTAO))
    if(procedureBlock(&((*tree)->childs[5])))
        return procedureSelection_(&((*tree)->childs[6]));
    return 0;
}
int procedureSelection_(SyntacticTree** tree){
    //printf("\nprocedure Selection'");
    if(currentToken.type != TK_ELSE){
        *tree = NULL;
        return 1;
    }
    struct Token tk;
    tk.attr = -1; tk.type = NT_SELECTION_;
    *tree = createSyntacticTree(tk, 2);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_ELSE))
        return procedureBlock(&((*tree)->childs[1]));
    return 0;
}
int procedureRepetition(SyntacticTree** tree){
    //printf("\nprocedure Repetition");
    struct Token tk;
    tk.attr = -1; tk.type = NT_REPETITION;
    if(currentToken.type == TK_ENQUANTO){
        *tree = createSyntacticTree(tk, 5);
        if(*tree == NULL)
            return 0;

        if(verifyToken(tree, 0, TK_ENQUANTO))
        if(verifyToken(tree, 1, TK_LEFT_PARENTHESES))
        if(procedureCondition(&((*tree)->childs[2])))
        if(verifyToken(tree, 3, TK_RIGHT_PARENTHESES))
            return procedureBlock(&((*tree)->childs[4]));

    }else if(currentToken.type == TK_REPITA){
        *tree = createSyntacticTree(tk, 7);
        if(*tree == NULL)
            return 0;

        if(verifyToken(tree, 0, TK_REPITA))
        if(procedureBlock(&((*tree)->childs[1])))
        if(verifyToken(tree, 2, TK_ENQUANTO))
        if(verifyToken(tree, 3, TK_LEFT_PARENTHESES))
        if(procedureCondition(&((*tree)->childs[4])))
        if(verifyToken(tree, 5, TK_RIGHT_PARENTHESES))
            return verifyToken(tree, 6, TK_SEMICOLON);
    }else{
        error();
    }
    return 0;
}
int procedureAttribution(SyntacticTree** tree){
    //printf("\nprocedure Attribution");
    struct Token tk;
    tk.attr = -1; tk.type = NT_ATTRIBUTION;
    *tree = createSyntacticTree(tk, 4);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_ID))
    if(verifyToken(tree, 1, TK_SET))
    if(procedureExpression(&((*tree)->childs[2])))
        return verifyToken(tree, 3, TK_SEMICOLON);
    return 0;
}
int procedureCondition(SyntacticTree** tree){
    //printf("\nprocedure Condition");
    struct Token tk;
    tk.attr = -1; tk.type = NT_CONDITION;
    *tree = createSyntacticTree(tk, 3);
    if(*tree == NULL)
        return 0;

    if(procedureExpression(&((*tree)->childs[0])))
    if(verifyToken(tree, 1, TK_RELATIONAL))
        return procedureExpression(&((*tree)->childs[2]));
    return 0;
}
int procedureExpression(SyntacticTree** tree){
    //printf("\nprocedure Expression");
    struct Token tk;
    tk.attr = -1; tk.type = NT_EXPRESSION;
    *tree = createSyntacticTree(tk, 2);
    if(*tree == NULL)
        return 0;

    if(procedureTerm(&((*tree)->childs[0])))
        return procedureExpression_(&((*tree)->childs[1]));
    return 0;
}
int procedureExpression_(SyntacticTree** tree){
    //printf("\nprocedure Expression'");
    if(currentToken.type != TK_ARITHMETIC ||
       (currentToken.attr != OP_ADD && currentToken.attr != OP_SUB)){
        *tree = NULL;
        return 1;
    }

    struct Token tk;
    tk.attr = -1; tk.type = NT_EXPRESSION_;
    *tree = createSyntacticTree(tk, 3);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_ARITHMETIC))
    if(procedureTerm(&((*tree)->childs[1])))
       return procedureExpression_(&((*tree)->childs[2]));
    return 0;
}
int procedureTerm(SyntacticTree** tree){
    //printf("\nprocedure Term");
    struct Token tk;
    tk.attr = -1; tk.type = NT_TERM;
    *tree = createSyntacticTree(tk, 2);
    if(*tree == NULL)
        return 0;

    if(procedureUnitary(&((*tree)->childs[0])))
        return procedureTerm_(&((*tree)->childs[1]));
    return 0;
}
int procedureTerm_(SyntacticTree** tree){
    //printf("\nprocedure Term'");
    if(currentToken.type != TK_ARITHMETIC ||
       (currentToken.attr != OP_MUL && currentToken.attr != OP_DIV)){
        *tree = NULL;
        return 1;
    }

    struct Token tk;
    tk.attr = -1; tk.type = NT_TERM_;
    *tree = createSyntacticTree(tk, 3);
    if(*tree == NULL)
        return 0;

    if(verifyToken(tree, 0, TK_ARITHMETIC))
    if(procedureUnitary(&((*tree)->childs[1])))
       return procedureTerm_(&((*tree)->childs[2]));
    return 0;
}
int procedureUnitary(SyntacticTree** tree){
    //printf("\nprocedure Unitary");
    struct Token tk;
    tk.attr = -1; tk.type = NT_UNITARY;

    if(currentToken.type == TK_ARITHMETIC){
        if(currentToken.attr != OP_SUB){
            error();
            return 0;
        }
        *tree = createSyntacticTree(tk, 2);
        if(*tree == NULL)
            return 0;

        if(verifyToken(tree, 0, TK_ARITHMETIC))
            return procedureFactor(&((*tree)->childs[1]));
    }else {
        *tree = createSyntacticTree(tk, 1);
        if(*tree == NULL)
            return 0;
        return procedureFactor(&((*tree)->childs[0]));
    }
    return 0;
}
int procedureFactor(SyntacticTree** tree){
    //printf("\nprocedure Factor");
    struct Token tk;
    tk.attr = -1; tk.type = NT_FACTOR;
    switch(currentToken.type){
        case TK_ID: case TK_CHAR: case TK_NUMBER:
            *tree = createSyntacticTree(tk, 1);
            if(*tree == NULL)
                return 0;
            return verifyToken(tree, 0, currentToken.type);
        case TK_LEFT_PARENTHESES:
            *tree = createSyntacticTree(tk, 3);
            if(*tree == NULL)
                return 0;

            if(verifyToken(tree, 0, TK_LEFT_PARENTHESES))
            if(procedureExpression(&((*tree)->childs[1])))
                return verifyToken(tree, 2, TK_RIGHT_PARENTHESES);
            return 0;
        default:
            break;
    }
    *tree = NULL;
    error();
    return 0;
}
