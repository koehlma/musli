#![cfg(feature = "std")]

use musli_json::Encoding;

// M marker indicating that some attributes should only apply when we're
// decoding in a JSON mode.
mod my_modes {
    use musli::{en::PairsEncoder, mode::Mode, Encode};

    pub(crate) enum Json {}

    impl Mode for Json {}

    impl<T, E> Encode<Json> for Result<T, E>
    where
        T: Encode<Json>,
        E: Encode<Json>,
    {
        fn encode<C, Enc>(&self, cx: &mut C, encoder: Enc) -> Result<Enc::Ok, C::Error>
        where
            C: musli::Context<Input = Enc::Error>,
            Enc: musli::Encoder,
        {
            let mut encoder = encoder.encode_map(cx, 2)?;
            match self {
                Ok(value) => {
                    encoder.insert::<Json, C, _, _>(cx, "result", "Ok")?;
                    encoder.insert::<Json, C, _, _>(cx, "value", value)?;
                }
                Err(error) => {
                    encoder.insert::<Json, C, _, _>(cx, "result", "Err")?;
                    encoder.insert::<Json, C, _, _>(cx, "error", error)?;
                }
            }
            encoder.end(cx)
        }
    }
}

const CONFIG: Encoding<my_modes::Json> = Encoding::new().with_mode();

#[test]
fn test_simple_json_encoding() {
    let expected = vec![Ok("abc"), Err(true)];

    let out = CONFIG.to_string(&expected).unwrap();
    println!("{}", out);
    assert_eq!(
        out,
        r#"[{"result":"Ok","value":"abc"},{"result":"Err","error":true}]"#
    )
}
