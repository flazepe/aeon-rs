macro_rules! yes_no {
    ($condition:expr $(, $yes:expr, $no:expr)?$(,)?) => {
        {
            let _yes = "Yes";
            $(let _yes = $yes;)?

            let _no = "No";
            $(let _no = $no;)?

            match $condition {
                true => _yes,
                false => _no,
            }
        }
    };
}

pub(crate) use yes_no;
