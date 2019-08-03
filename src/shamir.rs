use crate::field::Field;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Secret<F: Field>(F);

impl<F: Field> Secret<F> {

    pub fn generate_shares<R: Rng>(&self, k: u32, n: u32, rng: &mut R) -> Vec<SecretShare<F>> {
        let coefficients: Vec<F> = get_coefficients(k-1, rng);

        (0..n).map(|i| {
            let mut result = self.0.clone();
            let x = F::one().mul_by_scalar(i+1);
            let mut i = 1;
            for a in coefficients.clone() {
                result = result.add(&a.multiply(&x.to_pow(i)));
                i += 1;
            }
            SecretShare{index: x, share: result}
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct SecretShare<F: Field>{
    index: F,
    share: F,
}

pub fn reveal_secret<F: Field>(shares: Vec<SecretShare<F>>) -> Secret<F> {
    let mut sum = F::zero();
    for i in 0..shares.len() {
        let x = shares[i].index;
        let fx = shares[i].share;
        let mut product = F::one();
        for j in 0..shares.len() {
            if j == i {
                // do nothing
            } else {
                let idx = shares[j].index;
                let denom = idx.subtract(&x).inverse().unwrap();
                product = product.multiply(&idx.multiply(&denom));
            }
        }
        sum = sum.add(&fx.multiply(&product));
    }
    Secret(sum)
}

fn get_coefficients<F: Field, R: Rng>(amount: u32, rng: &mut R) -> Vec<F> {
    let mut coefficients: Vec<F> = Vec::new();
    for _ in 0..amount {
        let mut opt = None;
        while opt.is_none() {
            let coeff = F::rand(rng);
            if !coefficients.contains(&coeff) {
                opt = Some(coeff);
            }
        }
        coefficients.push(opt.unwrap());
    }
    coefficients
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use crate::field_instance::*;

    #[test]
    fn test_share_and_reveal() {
        for _ in 0..100 {
            let mut rng = thread_rng();
            let secret = G1613::new(1182);
            let secret = Secret(secret);

            let shares = secret.generate_shares(5, 10, &mut rng);
            let revealed = reveal_secret(shares[0..5].to_vec());
            assert_eq!(revealed.0, secret.0);
            let revealed = reveal_secret(shares[5..10].to_vec());
            assert_eq!(revealed.0, secret.0);
            let fail_reveal = reveal_secret(shares[2..6].to_vec());
            assert_ne!(fail_reveal.0, secret.0)
        }
    }
}
