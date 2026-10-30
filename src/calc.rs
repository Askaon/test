//! Silnik obliczeń: parsowanie i liczenie wyrażeń.
//! Obsługuje: + - * / % ^ (oraz × ÷), nawiasy, liczby z kropką lub przecinkiem.

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Liczba(f64),
    Plus,
    Minus,
    Razy,
    Dziel,
    Modulo,
    Potega,
    LNawias,
    PNawias,
}

fn tokenizuj(wejscie: &str) -> Result<Vec<Token>, String> {
    let mut tokeny = Vec::new();
    let znaki: Vec<char> = wejscie.chars().collect();
    let mut i = 0;

    while i < znaki.len() {
        let c = znaki[i];
        match c {
            ' ' | '\t' => {}
            '+' => tokeny.push(Token::Plus),
            '-' | '−' => tokeny.push(Token::Minus),
            '*' | '×' => tokeny.push(Token::Razy),
            '/' | '÷' => tokeny.push(Token::Dziel),
            '%' => tokeny.push(Token::Modulo),
            '^' => tokeny.push(Token::Potega),
            '(' => tokeny.push(Token::LNawias),
            ')' => tokeny.push(Token::PNawias),
            '0'..='9' | '.' | ',' => {
                let start = i;
                while i < znaki.len()
                    && (znaki[i].is_ascii_digit() || znaki[i] == '.' || znaki[i] == ',')
                {
                    i += 1;
                }
                let tekst: String = znaki[start..i].iter().collect::<String>().replace(',', ".");
                let liczba = tekst
                    .parse::<f64>()
                    .map_err(|_| format!("Niepoprawna liczba: {}", tekst))?;
                tokeny.push(Token::Liczba(liczba));
                continue;
            }
            _ => return Err(format!("Nieznany znak: '{}'", c)),
        }
        i += 1;
    }
    Ok(tokeny)
}

struct Parser {
    tokeny: Vec<Token>,
    poz: usize,
}

impl Parser {
    fn podglad(&self) -> Option<&Token> {
        self.tokeny.get(self.poz)
    }

    fn nastepny(&mut self) -> Option<Token> {
        let t = self.tokeny.get(self.poz).cloned();
        self.poz += 1;
        t
    }

    // wyrazenie := skladnik (('+' | '-') skladnik)*
    fn wyrazenie(&mut self) -> Result<f64, String> {
        let mut wynik = self.skladnik()?;
        loop {
            match self.podglad() {
                Some(Token::Plus) => {
                    self.nastepny();
                    wynik += self.skladnik()?;
                }
                Some(Token::Minus) => {
                    self.nastepny();
                    wynik -= self.skladnik()?;
                }
                _ => break,
            }
        }
        Ok(wynik)
    }

    // skladnik := unarny (('*' | '/' | '%') unarny)*
    fn skladnik(&mut self) -> Result<f64, String> {
        let mut wynik = self.unarny()?;
        loop {
            match self.podglad() {
                Some(Token::Razy) => {
                    self.nastepny();
                    wynik *= self.unarny()?;
                }
                Some(Token::Dziel) => {
                    self.nastepny();
                    let d = self.unarny()?;
                    if d == 0.0 {
                        return Err("Dzielenie przez zero".into());
                    }
                    wynik /= d;
                }
                Some(Token::Modulo) => {
                    self.nastepny();
                    let d = self.unarny()?;
                    if d == 0.0 {
                        return Err("Modulo przez zero".into());
                    }
                    wynik %= d;
                }
                _ => break,
            }
        }
        Ok(wynik)
    }

    // unarny := ('-' | '+') unarny | potega
    fn unarny(&mut self) -> Result<f64, String> {
        match self.podglad() {
            Some(Token::Minus) => {
                self.nastepny();
                Ok(-self.unarny()?)
            }
            Some(Token::Plus) => {
                self.nastepny();
                self.unarny()
            }
            _ => self.potega(),
        }
    }

    // potega := atom ('^' unarny)?   (prawostronnie łączna)
    fn potega(&mut self) -> Result<f64, String> {
        let podstawa = self.atom()?;
        if let Some(Token::Potega) = self.podglad() {
            self.nastepny();
            let wykladnik = self.unarny()?;
            return Ok(podstawa.powf(wykladnik));
        }
        Ok(podstawa)
    }

    // atom := liczba | '(' wyrazenie ')'
    fn atom(&mut self) -> Result<f64, String> {
        match self.nastepny() {
            Some(Token::Liczba(n)) => Ok(n),
            Some(Token::LNawias) => {
                let w = self.wyrazenie()?;
                match self.nastepny() {
                    Some(Token::PNawias) => Ok(w),
                    _ => Err("Brakuje nawiasu ')'".into()),
                }
            }
            Some(_) => Err("Nieoczekiwany symbol".into()),
            None => Err("Niekompletne wyrażenie".into()),
        }
    }
}

/// Liczy wartość wyrażenia tekstowego.
pub fn oblicz(wejscie: &str) -> Result<f64, String> {
    let tokeny = tokenizuj(wejscie)?;
    if tokeny.is_empty() {
        return Err("Puste wyrażenie".into());
    }
    let mut p = Parser { tokeny, poz: 0 };
    let wynik = p.wyrazenie()?;
    if p.poz < p.tokeny.len() {
        return Err("Nadmiarowe symbole".into());
    }
    if !wynik.is_finite() {
        return Err("Wynik poza zakresem".into());
    }
    Ok(wynik)
}

/// Formatuje wynik po polsku (przecinek dziesiętny, bez zbędnych zer).
pub fn formatuj(x: f64) -> String {
    let s = if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        let s = format!("{:.10}", x);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    };
    s.replace('.', ",")
}

#[cfg(test)]
mod testy {
    use super::*;

    #[test]
    fn dzialania() {
        assert_eq!(oblicz("2+3*4").unwrap(), 14.0);
        assert_eq!(oblicz("(2+3)×4").unwrap(), 20.0);
        assert_eq!(oblicz("9÷2").unwrap(), 4.5);
        assert_eq!(oblicz("-3 + 5").unwrap(), 2.0);
        assert_eq!(oblicz("2^3^2").unwrap(), 512.0);
        assert_eq!(oblicz("-2^2").unwrap(), -4.0);
        assert_eq!(oblicz("7 % 3").unwrap(), 1.0);
        assert_eq!(oblicz("1,5 × 2").unwrap(), 3.0);
    }

    #[test]
    fn bledy() {
        assert!(oblicz("1÷0").is_err());
        assert!(oblicz("(1+2").is_err());
        assert!(oblicz("2+").is_err());
        assert!(oblicz("").is_err());
    }

    #[test]
    fn format() {
        assert_eq!(formatuj(7.25), "7,25");
        assert_eq!(formatuj(42.0), "42");
        assert_eq!(formatuj(1.0 / 3.0), "0,3333333333");
        // wynik po "=" musi dać się ponownie przeliczyć
        assert_eq!(oblicz(&formatuj(7.25)).unwrap(), 7.25);
    }
}
