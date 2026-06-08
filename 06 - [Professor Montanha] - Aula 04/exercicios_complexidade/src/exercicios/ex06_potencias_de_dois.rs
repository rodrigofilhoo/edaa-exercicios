pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1;
    while i < n {
        println!("  {}", i);
        i *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_potencias_ate_16() {
        potencias_de_dois(16);
    }

    #[test]
    fn teste_potencias_ate_1() {
        potencias_de_dois(1);
    }

    #[test]
    fn teste_potencias_ate_1024() {
        potencias_de_dois(1024);
    }
}
