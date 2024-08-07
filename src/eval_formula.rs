/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/10 12:30:20 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/12 14:13:41 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::bool_eval::{ast::ASTBoolEval, lexer::lex_rpn};

pub fn eval_formula(formula: &str) -> bool {
    match lex_rpn(formula) {
        Ok(tkns) => match ASTBoolEval::try_from(tkns.as_ref()) {
            Ok(ast) => ast.eval(),
            Err(message) => {
                println!("parsing error: {}.", message);
                false
            }
        },
        Err(message) => {
            println!("lexing error: {}.", message);
            false
        }
    }
}
