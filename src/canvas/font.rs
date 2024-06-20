use std::collections::HashMap;

use crate::PixelPoint;

pub struct Font {
    glyphs: HashMap<char, Glyph>,
}

impl Font {
    pub fn new(glyphs: HashMap<char, Glyph>) -> Font {
        Font { glyphs }
    }

    pub fn glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&c)
    }
}

pub struct Glyph {
    width: usize,
    height: usize,
    points: Vec<PixelPoint>,
}

impl Glyph {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn points(&self) -> &[PixelPoint] {
        &self.points
    }
}

const DOT: char = '$';

const A_CAP: &str = " $
$ $
$$$
$ $
$ $
";
const B_CAP: &str = "$$
$ $
$$
$ $
$$
";
const C_CAP: &str = " $$
$
$
$
 $$
";
const D_CAP: &str = "$$
$ $
$ $
$ $
$$
";
const E_CAP: &str = "$$$
$
$$
$
$$$
";
const F_CAP: &str = "$$$
$
$$
$
$
";
const G_CAP: &str = " $$$
$
$ $$
$  $
 $$
";
const H_CAP: &str = "$ $
$ $
$$$
$ $
$ $
";
const I_CAP: &str = "$$$
 $
 $
 $
$$$
";
const J_CAP: &str = "  $
  $
  $
$ $
 $
";
const K_CAP: &str = "$ $
$ $
$$
$ $
$ $
";
const L_CAP: &str = "$
$
$
$
$$$
";
const M_CAP: &str = "$   $
$$ $$
$ $ $
$   $
$   $
";
const N_CAP: &str = "$  $
$$ $
$ $$
$  $
$  $
";
const O_CAP: &str = " $
$ $
$ $
$ $
 $
";
const P_CAP: &str = "$$
$ $
$$
$
$
";
const Q_CAP: &str = " $
$ $
$ $
$ $
 $$
";
const R_CAP: &str = "$$
$ $
$$
$ $
$ $
";
const S_CAP: &str = " $$
$
 $
  $
$$
";
const T_CAP: &str = "$$$
 $
 $
 $
 $
";
const U_CAP: &str = "$  $
$  $
$  $
$  $
 $$
";
const V_CAP: &str = "$ $
$ $
$ $
$ $
 $
";
const W_CAP: &str = "$   $
$   $
$   $
$ $ $
 $ $
";
const X_CAP: &str = "$ $
 $
$ $
$ $
$ $
";
const Y_CAP: &str = "$ $
 $
 $
 $
 $
";
const Z_CAP: &str = "$$$
 $
$
$
$$$
";

const A_LOW: &str = "

 $$
$ $
 $$
";
const B_LOW: &str = "$
$
$$
$ $
$$
";
const C_LOW: &str = "

 $
$
 $
";
const D_LOW: &str = "  $
  $
 $$
$ $
 $$
";
const E_LOW: &str = "
 $
$$$
$
 $$
";
const F_LOW: &str = "  $
 $
$$$
 $
 $
";
const G_LOW: &str = "
 $$
$ $
 $$
  $
$$
";
const H_LOW: &str = "$
$
$$
$ $
$ $
";
const I_LOW: &str = "$

$
$
$
";
const J_LOW: &str = "  $

  $
  $
$ $
 $
";
const K_LOW: &str = "$
$
$ $
$$
$ $
";
const L_LOW: &str = "$
$
$
$
 $
";
const M_LOW: &str = "

$$ $
$ $ $
$   $
";
const N_LOW: &str = "

$$
$ $
$ $
";
const O_LOW: &str = "

 $
$ $
 $
";
const P_LOW: &str = "

$$
$ $
$$
$
";
const Q_LOW: &str = "

 $
$ $
 $$
  $
";
const R_LOW: &str = "

$$
$
$
";
const S_LOW: &str = "
 $
$
 $
$
";
const T_LOW: &str = "
 $
$$$
 $
  $
";
const U_LOW: &str = "

$ $
$ $
 $$
";
const V_LOW: &str = "

$ $
$ $
 $
";
const W_LOW: &str = "

$   $
$ $ $
 $ $
";
const X_LOW: &str = "

$ $
 $
$ $
";
const Y_LOW: &str = "

$ $
 $$
  $
$$
";
const Z_LOW: &str = "
$$$$
  $
 $
$$$$
";

const ZERO: &str = " $$
$ $$
$$ $
$  $
 $$
";
const ONE: &str = " $
$$
 $
 $
$$$
";
const TWO: &str = " $$
$  $
  $
 $
$$$$
";
const THREE: &str = "$$
  $
 $
  $
$$
";
const FOUR: &str = "  $
 $$
$ $
$$$$
  $
";
const FIVE: &str = "$$$
$
$$
  $
$$
";
const SIX: &str = " $$
$
$$
$ $
 $
";
const SEVEN: &str = "$$$
  $
 $
 $
 $
";
const EIGHT: &str = " $
$ $
 $
$ $
 $
";
const NINE: &str = " $
$ $
 $$
  $
$$
";

const SPACE: &str = " 
";
const EXCLAMATION: &str = "$
$
$

$
";
const QUESTION: &str = "$$
  $
 $

 $
";
const PERIOD: &str = "



$
";
const COMMA: &str = "



$
$
";
const SINGLE_QUOTE: &str = "$
$
";
const DOUBLE_QUOTE: &str = "$ $
$ $
";
const LEFT_PAREN: &str = " $
$
$
$
 $
";
const RIGHT_PAREN: &str = "$
 $
 $
 $
$
";
const LEFT_BRACKET: &str = "$$
$
$
$
$$
";
const RIGHT_BRACKET: &str = "$$
 $
 $
 $
$$
";
const LEFT_BRACE: &str = "  $
 $
$$
 $
  $
";
const RIGHT_BRACE: &str = "$
 $
 $$
 $
$
";
const LEFT_ANGLE: &str = "  $
 $
$
 $
  $
";
const RIGHT_ANGLE: &str = "$
 $
  $
 $
$
";
const COLON: &str = "

$

$
";
const SEMICOLON: &str = "

$

$
$
";
const SLASH: &str = " $
 $
$
$
$
";
const BACKSLASH: &str = "$
$
 $
 $
 $
";
const DASH: &str = "

$$$
";
const UNDERSCORE: &str = "



$$$
";
const EQUALS: &str = "
$$$

$$$
";
const PLUS: &str = "
 $
$$$
 $
";
const ASTERISK: &str = "
 $
$$$
 $
$ $
";
const AMPERSAND: &str = " $
$ $
 $
$ $$
 $$$
";
const CARET: &str = " $
$ $
";
const PERCENT: &str = "$ $
  $
 $
$
$ $
";
const DOLLAR: &str = " $$$$
$ $
 $$$
  $ $
$$$$
";
const HASH: &str = " $ $
$$$$$
 $ $
$$$$$
 $ $
";
const AT: &str = " $$$
$   $
$  $$
$ $ $
 $$$$
";
const BACKTICK: &str = "$
";
const TILDE: &str = "
 $ $
$ $
";
const PIPE: &str = "$
$
$
$
$
";

const UNKNOWN: &str = "$$$
$$$
$$$
$$$
$$$
";
pub fn unknown_glyph() -> Glyph {
    glyph_from_str(UNKNOWN)
}

fn glyph_from_str(s: &str) -> Glyph {
    let mut width = 0;
    let mut height = 0;
    let mut points = Vec::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == DOT {
                points.push(PixelPoint {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
        width = width.max(line.len());
        height = y + 1;
    }
    Glyph {
        width,
        height,
        points,
    }
}

pub fn default_font() -> Font {
    let mut glyphs = HashMap::new();
    glyphs.insert('A', glyph_from_str(A_CAP));
    glyphs.insert('B', glyph_from_str(B_CAP));
    glyphs.insert('C', glyph_from_str(C_CAP));
    glyphs.insert('D', glyph_from_str(D_CAP));
    glyphs.insert('E', glyph_from_str(E_CAP));
    glyphs.insert('F', glyph_from_str(F_CAP));
    glyphs.insert('G', glyph_from_str(G_CAP));
    glyphs.insert('H', glyph_from_str(H_CAP));
    glyphs.insert('I', glyph_from_str(I_CAP));
    glyphs.insert('J', glyph_from_str(J_CAP));
    glyphs.insert('K', glyph_from_str(K_CAP));
    glyphs.insert('L', glyph_from_str(L_CAP));
    glyphs.insert('M', glyph_from_str(M_CAP));
    glyphs.insert('N', glyph_from_str(N_CAP));
    glyphs.insert('O', glyph_from_str(O_CAP));
    glyphs.insert('P', glyph_from_str(P_CAP));
    glyphs.insert('Q', glyph_from_str(Q_CAP));
    glyphs.insert('R', glyph_from_str(R_CAP));
    glyphs.insert('S', glyph_from_str(S_CAP));
    glyphs.insert('T', glyph_from_str(T_CAP));
    glyphs.insert('U', glyph_from_str(U_CAP));
    glyphs.insert('V', glyph_from_str(V_CAP));
    glyphs.insert('W', glyph_from_str(W_CAP));
    glyphs.insert('X', glyph_from_str(X_CAP));
    glyphs.insert('Y', glyph_from_str(Y_CAP));
    glyphs.insert('Z', glyph_from_str(Z_CAP));
    glyphs.insert('a', glyph_from_str(A_LOW));
    glyphs.insert('b', glyph_from_str(B_LOW));
    glyphs.insert('c', glyph_from_str(C_LOW));
    glyphs.insert('d', glyph_from_str(D_LOW));
    glyphs.insert('e', glyph_from_str(E_LOW));
    glyphs.insert('f', glyph_from_str(F_LOW));
    glyphs.insert('g', glyph_from_str(G_LOW));
    glyphs.insert('h', glyph_from_str(H_LOW));
    glyphs.insert('i', glyph_from_str(I_LOW));
    glyphs.insert('j', glyph_from_str(J_LOW));
    glyphs.insert('k', glyph_from_str(K_LOW));
    glyphs.insert('l', glyph_from_str(L_LOW));
    glyphs.insert('m', glyph_from_str(M_LOW));
    glyphs.insert('n', glyph_from_str(N_LOW));
    glyphs.insert('o', glyph_from_str(O_LOW));
    glyphs.insert('p', glyph_from_str(P_LOW));
    glyphs.insert('q', glyph_from_str(Q_LOW));
    glyphs.insert('r', glyph_from_str(R_LOW));
    glyphs.insert('s', glyph_from_str(S_LOW));
    glyphs.insert('t', glyph_from_str(T_LOW));
    glyphs.insert('u', glyph_from_str(U_LOW));
    glyphs.insert('v', glyph_from_str(V_LOW));
    glyphs.insert('w', glyph_from_str(W_LOW));
    glyphs.insert('x', glyph_from_str(X_LOW));
    glyphs.insert('y', glyph_from_str(Y_LOW));
    glyphs.insert('z', glyph_from_str(Z_LOW));
    glyphs.insert('0', glyph_from_str(ZERO));
    glyphs.insert('1', glyph_from_str(ONE));
    glyphs.insert('2', glyph_from_str(TWO));
    glyphs.insert('3', glyph_from_str(THREE));
    glyphs.insert('4', glyph_from_str(FOUR));
    glyphs.insert('5', glyph_from_str(FIVE));
    glyphs.insert('6', glyph_from_str(SIX));
    glyphs.insert('7', glyph_from_str(SEVEN));
    glyphs.insert('8', glyph_from_str(EIGHT));
    glyphs.insert('9', glyph_from_str(NINE));
    glyphs.insert(' ', glyph_from_str(SPACE));
    glyphs.insert('!', glyph_from_str(EXCLAMATION));
    glyphs.insert('?', glyph_from_str(QUESTION));
    glyphs.insert('.', glyph_from_str(PERIOD));
    glyphs.insert(',', glyph_from_str(COMMA));
    glyphs.insert('\'', glyph_from_str(SINGLE_QUOTE));
    glyphs.insert('"', glyph_from_str(DOUBLE_QUOTE));
    glyphs.insert('(', glyph_from_str(LEFT_PAREN));
    glyphs.insert(')', glyph_from_str(RIGHT_PAREN));
    glyphs.insert('[', glyph_from_str(LEFT_BRACKET));
    glyphs.insert(']', glyph_from_str(RIGHT_BRACKET));
    glyphs.insert('{', glyph_from_str(LEFT_BRACE));
    glyphs.insert('}', glyph_from_str(RIGHT_BRACE));
    glyphs.insert('<', glyph_from_str(LEFT_ANGLE));
    glyphs.insert('>', glyph_from_str(RIGHT_ANGLE));
    glyphs.insert(':', glyph_from_str(COLON));
    glyphs.insert(';', glyph_from_str(SEMICOLON));
    glyphs.insert('/', glyph_from_str(SLASH));
    glyphs.insert('\\', glyph_from_str(BACKSLASH));
    glyphs.insert('-', glyph_from_str(DASH));
    glyphs.insert('_', glyph_from_str(UNDERSCORE));
    glyphs.insert('=', glyph_from_str(EQUALS));
    glyphs.insert('+', glyph_from_str(PLUS));
    glyphs.insert('*', glyph_from_str(ASTERISK));
    glyphs.insert('&', glyph_from_str(AMPERSAND));
    glyphs.insert('^', glyph_from_str(CARET));
    glyphs.insert('%', glyph_from_str(PERCENT));
    glyphs.insert('$', glyph_from_str(DOLLAR));
    glyphs.insert('#', glyph_from_str(HASH));
    glyphs.insert('@', glyph_from_str(AT));
    glyphs.insert('`', glyph_from_str(BACKTICK));
    glyphs.insert('~', glyph_from_str(TILDE));
    glyphs.insert('|', glyph_from_str(PIPE));

    Font { glyphs }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_from_str() {
        let glyph = glyph_from_str(A_CAP);
        assert_eq!(glyph.width, 3);
        assert_eq!(glyph.height, 5);
        assert_eq!(glyph.points.len(), 10);
    }
}
