use super::Loc;
use super::token::{TokenKind, Token};
use super::ast::{Op, Ast};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
  UnexpectedToken(Token),
  NotExpression(Token),
  NotOperator(Token),
  UnclosedOpenParen(Token),
  RedundantExpression(Token),
  Eof,
}

// atom
fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  tokens
    .next()
    .ok_or(ParseError::Eof)
    .and_then(|tok| match tok.value {
      // NUMBER
	  TokenKind::Number(n) => Ok(Ast::num(n, tok.loc)),
	  TokenKind::Plus     => Ok(Ast::op(Op::add(tok.loc), tok.loc)),
	  TokenKind::Minus    => Ok(Ast::op(Op::sub(tok.loc), tok.loc)),
	  TokenKind::Asterisk => Ok(Ast::op(Op::mul(tok.loc), tok.loc)),
	  TokenKind::Slash    => Ok(Ast::op(Op::div(tok.loc), tok.loc)),
	  TokenKind::Less     => Ok(Ast::op(Op::lt(tok.loc), tok.loc)),
	  TokenKind::Equal    => Ok(Ast::op(Op::equal(tok.loc), tok.loc)),
	  TokenKind::Greater  => Ok(Ast::op(Op::gt(tok.loc), tok.loc)),
	  TokenKind::And      => Ok(Ast::op(Op::and(tok.loc), tok.loc)),
	  TokenKind::Or       => Ok(Ast::op(Op::or(tok.loc), tok.loc)),
      _ => Err(ParseError::NotExpression(tok)),
    })
}

/// cdr  : ')'
///      | '.' list ')'
///      | list cdr
///      ;
fn parse_cdr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  let tok = tokens.peek().ok_or(ParseError::Eof)?.clone();
  match tok.value {
      TokenKind::RParen => {
        tokens.next();
        Ok(Ast::nil(tok.loc))
      },
      TokenKind::Dot => {
        tokens.next();
        let t = parse_list(tokens)?;
        let tok = tokens.peek().ok_or(ParseError::Eof)?.clone();
        match tok.value {
            TokenKind::RParen => {
              tokens.next();
              Ok(t)
            },
            _ => Err(ParseError::UnclosedOpenParen(tok)),
        }
      },
      _ => {
        let l = parse_list(tokens)?;
        let r = parse_cdr(tokens)?;
        let loc = l.loc;
        Ok(Ast::pair(l, r, loc))
      },
    }
}

/// pair : ')'
///      | list cdr
///      ;
fn parse_pair<Tokens>(tokens: &mut Peekable<Tokens>, loc: Loc) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  match tokens.peek()
    .ok_or(ParseError::Eof)?
    .value {
      TokenKind::RParen => {
        tokens.next();
        Ok(Ast::nil(loc))
      },
      _ => {
        let l = parse_list(tokens)?;
        let r = parse_cdr(tokens)?;
        Ok(Ast::pair(l, r, loc))
      },
    }
}

/// list : "(" pair
///      | "'"  list
///      | ATOM
///      ;
fn parse_list<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  let tok = tokens.peek().ok_or(ParseError::Eof)?.clone();
  match tok.value {
      TokenKind::LParen => {
        tokens.next();
        parse_pair(tokens, tok.loc)
      },
      TokenKind::Quote => {
        tokens.next();
        let q = parse_list(tokens)?;
        Ok(Ast::quote(q, tok.loc))
      },
      _ => parse_atom(tokens),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  parse_list(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
  let mut tokens = tokens.into_iter().peekable();
  let ret = parse_expr(&mut tokens)?;
  match tokens.next() {
    Some(tok) => Err(ParseError::RedundantExpression(tok)),
    None => Ok(ret),
  }
}