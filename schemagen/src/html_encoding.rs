use crate::libc::MemchrExt as _;

/// # Panics
///
/// - If unclosed html code encountered.
/// - On unknown code.
#[must_use]
pub fn decode(mut value: &[u8]) -> Vec<u8> {
    let mut r = Vec::<u8>::with_capacity(value.len());

    while let Some(start_pos) = value.find_needle(b'&') {
        let len = value[start_pos..]
            .find_needle(b';')
            .expect("unclosed html code");
        r.extend(&value[..start_pos]);

        r.push(match &value[start_pos + 1..start_pos + len] {
            code if code[0] == b'#' => {
                let mut n = 0u8;
                for b in &code[1..] {
                    n = (n * 10) + b - b'0';
                }
                n
            }
            b"lt" => b'<',
            b"gt" => b'>',
            b"quot" => b'"',
            code => unimplemented!("unknown code {:?}", unsafe {
                str::from_utf8_unchecked(code)
            }),
        });

        value = &value[start_pos + len + 1..];
    }

    r.extend(value);
    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn html_encoding_decode() {
        assert_eq!(
            unsafe {
                String::from_utf8_unchecked(super::decode(b"The update&#39;s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you&#39;re using <a href=\"#setwebhook\">webhooks</a>, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially."))
            },
            unsafe {
                str::from_utf8_unchecked(b"The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using <a href=\"#setwebhook\">webhooks</a>, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.")
            },
        );
    }
}
