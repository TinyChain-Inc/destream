use futures::executor::block_on;

#[test]
fn decoded_owned_sequences_and_strings_have_no_spare_allocation() {
    block_on(async {
        for count in [0, 1, 2, 3, 5, 9, 17] {
            let expected = vec!["quoted \" text".to_owned(); count];
            let input = destream_json::encode(&expected).unwrap();
            let actual: Vec<String> = destream_json::try_decode((), input).await.unwrap();
            assert_eq!(actual, expected);
            assert_eq!(actual.capacity(), actual.len());
            for string in actual {
                assert_eq!(string.capacity(), string.len());
            }
        }
    });
}
