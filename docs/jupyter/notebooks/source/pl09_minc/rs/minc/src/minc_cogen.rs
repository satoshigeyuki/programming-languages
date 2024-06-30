use crate::minc_ast;

/* 
two directives below must be remove once you implemented
the function.
*/
#[allow(unreachable_code, unused_variables)]
pub fn ast_to_asm_program(_program : minc_ast::Program) -> String {
    let asm = "# assembly generated by minc compiler ...\n";
    panic!("YOU MUST IMPLEMENT rs/minc/src/minc_cogen.go:ast_to_asm_program");
    asm.to_string()
}
