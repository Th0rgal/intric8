use x25519_dalek::EphemeralSecret;
use x25519_dalek::PublicKey;

use rand_os::OsRng;

fn main() {
    let mut alice_csprng = OsRng::new().unwrap();
    let alice_secret = EphemeralSecret::new(&mut alice_csprng);
    let alice_public = PublicKey::from(&alice_secret);

    let mut bob_csprng = OsRng::new().unwrap();
    let bob_secret = EphemeralSecret::new(&mut bob_csprng);
    let bob_public = PublicKey::from(&bob_secret);

    let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
    let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);

    println!("secret: {:?}", base64::encode(alice_shared_secret.as_bytes()));
}
