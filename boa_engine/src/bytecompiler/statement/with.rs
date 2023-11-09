use crate::{bytecompiler::ByteCompiler, vm::Opcode};
use boa_ast::statement::With;

impl ByteCompiler<'_> {
    /// Compile a [`With`] `boa_ast` node
    pub(crate) fn compile_with(&mut self, with: &With, use_expr: bool) {
        self.compile_expr(with.expression(), true);

        let old_lex_env = self.lexical_environment.clone();
        let _ = self.push_compile_environment(false);
        self.emit_opcode(Opcode::PushObjectEnvironment);

        self.compile_stmt(with.statement(), use_expr, true);

        self.pop_compile_environment();
        self.lexical_environment = old_lex_env;
        self.emit_opcode(Opcode::PopEnvironment);
    }
}
