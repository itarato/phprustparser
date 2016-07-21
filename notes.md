PHPCODE := PhpStart + CODE

CODE := STMT*

STMT := EXP | FN_DEF

FN_DEF := NAME ARGS CODE

ARGS := EXP*

EXP := VAR | VAR OP VAR

OP := + | - | * | / | =

-------------------------------------

- code
    - func
        - keyword
        - fname
        - popen
        - args
            - varname
        - pclose
        - bopen
        - code
            - expr
                - fcall
                    - fname
                    - popen
                    - args
                        - expr
                            - expr
                                - string
                            - op
                            - expr
                                - varname
                    - pclose
        - bclose
    - expr
        - expr
            - varname
        - op
        - expr
            - string
    - expr
        - fcall
            - fname
            - popen
            - args
                - expr
                    - varname
            - pclose