mod context;
mod imp;

use super::parser::{parse, Parser};
use super::types::{ErrKind, RootNd, Type};
use context::Context;
pub use context::Semantic;

pub trait Analyzer: Parser {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind>;
}

pub fn analyze(code: &str) -> Result<(RootNd, Semantic), ErrKind> {
    let nd_res = parse::<RootNd>(code);
    if nd_res.is_err() {
        return Err(ErrKind::ParseErr);
    }
    let nd = nd_res.unwrap();
    let mut cxt = Context::new();
    nd.analyze(&mut cxt)?;
    Ok((nd, Semantic::new(cxt)?))
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use crate::utils::load_code_from_file;

    #[test]
    fn test_redeclare() {
        let code = load_code_from_file("test_cfiles/analyzer/redec_0.c");
        let res = analyze(&code);
        assert_eq!(String::from("ReDeclare"), format!("{}", res.unwrap_err()));
        let code = load_code_from_file("test_cfiles/analyzer/redec_1.c");
        assert!(analyze(&code).is_ok());
    }

    #[test]
    fn test_mainfunc() {
        let code = load_code_from_file("test_cfiles/analyzer/main_0.c");
        let res = analyze(&code);
        assert_eq!(String::from("NoMainFunc"), format!("{}", res.unwrap_err()));
        let code = load_code_from_file("test_cfiles/analyzer/main_1.c");
        let res = analyze(&code);
        // main func type error
        assert_eq!(String::from("TypeErr"), format!("{}", res.unwrap_err()));
    }

    #[test]
    fn test_var() {
        let code = load_code_from_file("test_cfiles/analyzer/var_0.c");
        assert!(analyze(&code).is_ok());
        let code = load_code_from_file("test_cfiles/analyzer/var_1.c-");
        let res = analyze(&code);
        assert_eq!(String::from("NoDeclare"), format!("{}", res.unwrap_err()));
        let code = load_code_from_file("test_cfiles/analyzer/var_2.c-");
        let res = analyze(&code);
        assert_eq!(String::from("GlobalNeedConst"), format!("{}", res.unwrap_err()));
    }

    #[test]
    fn test_func() {
        // good func 0 direct impl
        let code = load_code_from_file("test_cfiles/analyzer/func_0.c");
        assert!(analyze(&code).is_ok());
        // good func 1 declare then impl
        let code = load_code_from_file("test_cfiles/analyzer/func_1.c");
        assert!(analyze(&code).is_ok());
        // double declare
        let code = load_code_from_file("test_cfiles/analyzer/func_2.c");
        let res = analyze(&code);
        assert_eq!(String::from("ReDeclare"), format!("{}", res.unwrap_err()));
        // double impl
        let code = load_code_from_file("test_cfiles/analyzer/func_3.c");
        let res = analyze(&code);
        assert_eq!(String::from("ReImpl"), format!("{}", res.unwrap_err()));
        // No impl
        let code = load_code_from_file("test_cfiles/analyzer/func_4.c");
        let res = analyze(&code);
        assert_eq!(String::from("FuncNoImpl"), format!("{}", res.unwrap_err()));
        // declare & impl param mismatch
        let code = load_code_from_file("test_cfiles/analyzer/func_5.c");
        let res = analyze(&code);
        assert_eq!(String::from("TypeErr"), format!("{}", res.unwrap_err()));
        // declare & call param mismatch
        let code = load_code_from_file("test_cfiles/analyzer/func_6.c-");
        let res = analyze(&code);
        assert_eq!(String::from("TypeErr"), format!("{}", res.unwrap_err()));
    }
}
