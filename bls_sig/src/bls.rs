use ark_ec::{pairing::Pairing, AffineRepr} ;
use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, G1Affine, G2Affine, Fr as ScalarField, Fq12};
use ark_ff::Field;
use ark_std::{Zero, UniformRand};
use num_bigint::{self, BigUint};


impl BLS {

    
    fn keygen(ikm: Vec<u8>) {
        let mut rng = ark_std::test_rng();
        let s = ScalarField::rand(&mut rng);
        let a : G1Affine = G1::rand(&mut rng).into();
        let b: G2Affine = G2::rand(&mut rng).into();


    } 

    fn sign(&self) -> BLSignature {
        todo!()    
        let _message_field = ScalarField::from(self.message);
        let _privkey_field = ScalarField::from(self.priv_key);
        let _message_g1 = G1Affine::from(_message_field);
        let _privkey_g2 = G2Affine::from(_privkey_field)
        let _r = Bls12_381::pairing(_message_g1, _privkey_g2;


    }

    fn verify(&self) -> bool {
        unimplemented!()
    }

    fn aggregate(&self) -> BLS {
        unimplemented!()
    }

    fn aggregatedVerify(&self)-> bool {
        unimplemented!()
    }

    fn keyValidate() -> bool {

    }
}
