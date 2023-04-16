module.exports = grammar({
    name: 'Quanta',

    rules: {
        source_file: $ => $.block,
    
        block: $ => repeat1($.statement),
    
        bracket_block: $ => seq(
            '{',
            $.block,
            '}'
        ),

        statement: $ => choice(
            $.command,
            $.initialization_statement,
            $.if_statement,
            $.for_statement,
            $.while_statement
        ),
    
        command: $ => seq(
            $.identifier,
            '(',
            optional(seq(
                $._expression,
                repeat(seq(',', $._expression)))),
            ')',
            ';'
        ),
    
        initialization_statement: $ => seq(
            $._type,
            $._initialization,
            repeat(seq(',', $._initialization)),
            ';',
        ),
    
        _initialization: $ => seq(
            $.identifier,
            '=',
            $._expression
        ),
    
        if_statement: $ => seq(
            "if",
            "(",
            $._expression,
            ")",
            $.bracket_block
        ),
    
        for_statement: $ => seq(
            "for",
            "(",
            $.initialization_statement,
            $._expression,
            ';',
            $._expression,
            ')',
            $.bracket_block
        ),
    
        while_statement: $ => seq(
            "while",
            "(", $._expression, ")",
            $.bracket_block
        ),
    
        _type: $ => choice(
        $.array_type,
        $.primitive_type,
      ),
    
      primitive_type: $ => choice(
        'bool',
        'int',
        'color',
        'float',
      ),
    
      array_type: $ => seq(
        $._type,
        '[',
        choice($.nat_number, $.identifier),
        ']',
      ),
    
        _expression: $ => choice(
          $.identifier,
          $.number
          // TODO: other kinds of _expressions
        ),
    
        identifier: $ => /[a-z][a-zA-Z0-9]*/,
    
        number: $ => choice(
            $.float_number,
            $.nat_number,
        ),
    
        float_number: $ => /\d+\.\d+/,
        nat_number: $ => /\d+/        
    } 
});

/*
rules: {
    
   

    
*/