use mal_core::{self, reader::ParseError};

fn read_print(input: &str) -> Result<String, ParseError> {
    match mal_core::read(input) {
        Ok(ast) => Ok(format!("{:?}", ast)),
        Err(ParseError::EOF) => Ok(String::new()),
        Err(err) => Err(err),
    }
}

#[test]
pub fn testing_read_of_numbers() -> Result<(), ParseError> {
    assert_eq!(read_print("1")?, String::from("1"));
    assert_eq!(read_print("7")?, String::from("7"));
    assert_eq!(read_print("  7   ")?, String::from("7"));
    assert_eq!(read_print("-123")?, String::from("-123"));
    Ok(())
}

#[test]
pub fn testing_read_of_symbols() -> Result<(), ParseError> {
    assert_eq!(read_print("+")?, String::from("+"));
    assert_eq!(read_print("abc")?, String::from("abc"));
    assert_eq!(read_print("   abc   ")?, String::from("abc"));
    assert_eq!(read_print("abc5")?, String::from("abc5"));
    assert_eq!(read_print("abc-def")?, String::from("abc-def"));
    Ok(())
}

#[test]
pub fn testing_non_numbers_starting_with_a_dash() -> Result<(), ParseError> {
    assert_eq!(read_print("-")?, String::from("-"));
    assert_eq!(read_print("-abc")?, String::from("-abc"));
    assert_eq!(read_print("->>")?, String::from("->>"));
    Ok(())
}

#[test]
pub fn testing_read_of_lists() -> Result<(), ParseError> {
    assert_eq!(read_print("(+ 1 2)")?, String::from("(+ 1 2)"));
    assert_eq!(read_print("()")?, String::from("()"));
    assert_eq!(read_print("( )")?, String::from("()"));
    assert_eq!(read_print("(nil)")?, String::from("(nil)"));
    assert_eq!(read_print("((3 4))")?, String::from("((3 4))"));
    assert_eq!(read_print("(+ 1 (+ 2 3))")?, String::from("(+ 1 (+ 2 3))"));
    assert_eq!(
        read_print("  ( +   1   (+   2 3   )   )  ")?,
        String::from("(+ 1 (+ 2 3))")
    );
    assert_eq!(read_print("(* 1 2)")?, String::from("(* 1 2)"));
    assert_eq!(read_print("(** 1 2)")?, String::from("(** 1 2)"));
    assert_eq!(read_print("(* -3 6)")?, String::from("(* -3 6)"));
    assert_eq!(read_print("(()())")?, String::from("(() ())"));
    Ok(())
}

#[test]
pub fn test_commas_as_whitespace() -> Result<(), ParseError> {
    assert_eq!(read_print("(1 2, 3,,,,),,")?, String::from("(1 2 3)"));
    Ok(())
}

#[test]
pub fn testing_read_of_nil_true_false() -> Result<(), ParseError> {
    assert_eq!(read_print("nil")?, String::from("nil"));
    assert_eq!(read_print("true")?, String::from("true"));
    assert_eq!(read_print("false")?, String::from("false"));
    Ok(())
}

#[test]
pub fn testing_read_of_strings() -> Result<(), ParseError> {
    assert_eq!(read_print(r#""abc""#)?, String::from(r#""abc""#));
    assert_eq!(read_print(r#"   "abc"   "#)?, String::from(r#""abc""#));
    assert_eq!(
        read_print(r#""abc (with parens)""#)?,
        String::from(r#""abc (with parens)""#)
    );
    assert_eq!(read_print(r#""abc\"def""#)?, String::from(r#""abc\"def""#));
    assert_eq!(read_print(r#""""#)?, String::from(r#""""#));
    assert_eq!(read_print(r#""\\""#)?, String::from(r#""\\""#));
    assert_eq!(
        read_print(r#""\\\\\\\\\\\\\\\\\\""#)?,
        String::from(r#""\\\\\\\\\\\\\\\\\\""#)
    );
    assert_eq!(read_print(r#""&""#)?, String::from(r#""&""#));
    assert_eq!(read_print(r#""'""#)?, String::from(r#""'""#));
    assert_eq!(read_print(r#""(""#)?, String::from(r#""(""#));
    assert_eq!(read_print(r#"")""#)?, String::from(r#"")""#));
    assert_eq!(read_print(r#""*""#)?, String::from(r#""*""#));
    assert_eq!(read_print(r#""+""#)?, String::from(r#""+""#));
    assert_eq!(read_print(r#"",""#)?, String::from(r#"",""#));
    assert_eq!(read_print(r#""-""#)?, String::from(r#""-""#));
    assert_eq!(read_print(r#""/""#)?, String::from(r#""/""#));
    assert_eq!(read_print(r#"":""#)?, String::from(r#"":""#));
    assert_eq!(read_print(r#"";""#)?, String::from(r#"";""#));
    assert_eq!(read_print(r#""<""#)?, String::from(r#""<""#));
    assert_eq!(read_print(r#""=""#)?, String::from(r#""=""#));
    assert_eq!(read_print(r#"">""#)?, String::from(r#"">""#));
    assert_eq!(read_print(r#""?""#)?, String::from(r#""?""#));
    assert_eq!(read_print(r#""@""#)?, String::from(r#""@""#));
    assert_eq!(read_print(r#""[""#)?, String::from(r#""[""#));
    assert_eq!(read_print(r#""]""#)?, String::from(r#""]""#));
    assert_eq!(read_print(r#""^""#)?, String::from(r#""^""#));
    assert_eq!(read_print(r#""_""#)?, String::from(r#""_""#));
    assert_eq!(read_print(r#""`""#)?, String::from(r#""`""#));
    assert_eq!(read_print(r#""{""#)?, String::from(r#""{""#));
    assert_eq!(read_print(r#""}""#)?, String::from(r#""}""#));
    assert_eq!(read_print(r#""~""#)?, String::from(r#""~""#));
    assert_eq!(read_print(r#""!""#)?, String::from(r#""!""#));
    Ok(())
}

#[test]
pub fn testing_reader_errors() {
    assert!(read_print("(1 2").is_err());
    assert!(read_print("[1 2").is_err());
    assert!(read_print(r#"""#).is_err());
    assert!(read_print(r#""\""#).is_err());
    assert!(read_print(r#""\\\\\\\\\\\\\\\\\\\""#).is_err());
    assert!(read_print(r#"(1 "abc"#).is_err());
    assert!(read_print(r#"(1 "abc""#).is_err());
}

#[test]
pub fn testing_read_of_quoting() -> Result<(), ParseError> {
    assert_eq!(read_print("'1")?, String::from("(quote 1)"));
    assert_eq!(read_print("'(1 2 3)")?, String::from("(quote (1 2 3))"));
    assert_eq!(read_print("`1")?, String::from("(quasiquote 1)"));
    assert_eq!(
        read_print("`(1 2 3)")?,
        String::from("(quasiquote (1 2 3))")
    );
    assert_eq!(read_print("~1")?, String::from("(unquote 1)"));
    assert_eq!(read_print("~(1 2 3)")?, String::from("(unquote (1 2 3))"));
    assert_eq!(
        read_print("`(1 ~a 3)")?,
        String::from("(quasiquote (1 (unquote a) 3))")
    );
    assert_eq!(
        read_print("~@(1 2 3)")?,
        String::from("(splice-unquote (1 2 3))")
    );
    Ok(())
}

#[test]
pub fn testing_keywords() -> Result<(), ParseError> {
    assert_eq!(read_print(":kw")?, String::from(":kw"));
    assert_eq!(
        read_print("(:kw1 :kw2 :kw3)")?,
        String::from("(:kw1 :kw2 :kw3)")
    );
    Ok(())
}

#[test]
pub fn testing_read_of_vectors() -> Result<(), ParseError> {
    assert_eq!(read_print("[+ 1 2]")?, String::from("[+ 1 2]"));
    assert_eq!(read_print("[]")?, String::from("[]"));
    assert_eq!(read_print("[ ]")?, String::from("[]"));
    assert_eq!(read_print("[[3 4]]")?, String::from("[[3 4]]"));
    assert_eq!(read_print("[+ 1 [+ 2 3]]")?, String::from("[+ 1 [+ 2 3]]"));
    assert_eq!(
        read_print("  [ +   1   [+   2 3   ]   ]  ")?,
        String::from("[+ 1 [+ 2 3]]")
    );
    assert_eq!(read_print("([])")?, String::from("([])"));
    Ok(())
}

#[test]
pub fn testing_read_of_hash_maps() -> Result<(), ParseError> {
    assert_eq!(read_print("{}")?, String::from("{}"));
    assert_eq!(read_print("{ }")?, String::from("{}"));
    assert_eq!(read_print(r#"{"abc" 1}"#)?, String::from(r#"{"abc" 1}"#));
    assert_eq!(
        read_print(r#"{"a" {"b" 2}}"#)?,
        String::from(r#"{"a" {"b" 2}}"#)
    );
    assert_eq!(
        read_print(r#"{"a" {"b" {"c" 3}}}"#)?,
        String::from(r#"{"a" {"b" {"c" 3}}}"#)
    );
    assert_eq!(
        read_print(r#"{  "a"  {"b"   {  "cde"     3   }  }}"#)?,
        String::from(r#"{"a" {"b" {"cde" 3}}}"#)
    );
    assert_eq!(
        read_print("{  :a  {:b   {  :cde     3   }  }}")?,
        String::from("{:a {:b {:cde 3}}}")
    );
    assert_eq!(read_print(r#"{"1" 1}"#)?, String::from(r#"{"1" 1}"#));
    assert_eq!(read_print("({})")?, String::from("({})"));
    Ok(())
}

#[test]
pub fn testing_read_of_comments() -> Result<(), ParseError> {
    assert_eq!(
        read_print(" ;; whole line comment (not an exception)")?,
        String::from("")
    );
    assert_eq!(
        read_print("1 ; comment after expression")?,
        String::from("1")
    );
    assert_eq!(
        read_print("1; comment after expression")?,
        String::from("1")
    );
    Ok(())
}

#[test]
pub fn testing_read_of_deref() -> Result<(), ParseError> {
    assert_eq!(read_print("@a")?, String::from("(deref a)"));
    Ok(())
}

#[test]
#[should_panic]
pub fn testing_read_of_metadata() {
    assert_eq!(
        read_print(r#"^{"a" 1} [1 2 3]"#).unwrap(),
        String::from(r#"(with-meta [1 2 3] {"a" 1})"#)
    );
}

#[test]
pub fn non_alphanumerice_characters_in_strings() -> Result<(), ParseError> {
    assert_eq!(read_print(r#""\n""#)?, String::from(r#""\n""#));
    assert_eq!(read_print("\"#\"")?, String::from("\"#\""));
    assert_eq!(read_print(r#""$""#)?, String::from(r#""$""#));
    assert_eq!(read_print(r#""%""#)?, String::from(r#""%""#));
    assert_eq!(read_print(r#"".""#)?, String::from(r#"".""#));
    assert_eq!(read_print(r#""\\""#)?, String::from(r#""\\""#));
    assert_eq!(read_print(r#""|""#)?, String::from(r#""|""#));
    Ok(())
}
