use std::{fmt::Debug, path::PathBuf};

/// Shorten two iterables to the first element they do not have in common (inclusive).
fn get_divergence<E>(e1: E, e2: E) -> (E, E)
where
    <E as IntoIterator>::Item: Eq + PartialEq + Debug + Default,
    E: IntoIterator + Default + Extend<<E as IntoIterator>::Item>,
{
    let mut diverged = false;

    e1.into_iter()
        .zip(e2)
        .take_while(|(e1_e, e2_e)| {
            if diverged {
                !diverged
            } else {
                diverged = e1_e != e2_e;
                true
            }
        })
        .unzip()
}

/// Split the path string accurately, that is, respecting escaping backslashes
fn split_path(s: &String) -> Vec<String> {
    // TODO: Find a better way to do this
    PathBuf::from(s)
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .rev()
        .collect()
}

/// Shorten all vault paths to the shortest unique value
/// Performance is probably not great, but we shouldn't have too many values to process
pub fn make_unique(vaults: Vec<String>) -> Vec<String> {
    let v: Vec<Vec<String>> = vaults.iter().map(split_path).collect();

    v.iter()
        .enumerate()
        .map(|(i, va)| {
            let mut v = v.clone();
            v.remove(i);

            let mut res: Vec<String> = v
                .iter()
                .map(|e| get_divergence(va.to_vec(), e.to_vec()).0)
                .max()
                .unwrap_or(va.to_vec());

            res.reverse();
            res.join("/")
        })
        .collect()
}

#[cfg(test)]
mod tests {

    mod differentiate {
        use crate::display_name::make_unique;

        #[test]
        fn different_name_same_path() {
            let vaults = vec![
                String::from("~/Documents/personal"),
                String::from("~/Documents/work"),
            ];

            let names = make_unique(vaults);

            assert_eq!(names[0], "personal");
            assert_eq!(names[1], "work");
        }

        #[test]
        fn same_name_different_parent() {
            let vaults = vec![
                String::from("~/Downloads/personal"),
                String::from("~/Documents/personal"),
            ];

            let names = make_unique(vaults);

            assert_eq!(names[0], "Downloads/personal");
            assert_eq!(names[1], "Documents/personal");
        }

        #[test]
        fn same_name_same_parent() {
            let vaults = vec![
                String::from("~/Downloads/vaults/personal"),
                String::from("~/Documents/vaults/personal"),
            ];

            let names = make_unique(vaults);

            assert_eq!(names[0], "Downloads/vaults/personal");
            assert_eq!(names[1], "Documents/vaults/personal");
        }

        #[test]
        fn many() {
            let vaults = vec![
                String::from("~/Downloads/vaults/personal"),
                String::from("~/Documents/vaults/personal"),
                String::from("~/Downloads/personal"),
                String::from("~/Documents/personal"),
                String::from("~/Documents/work"),
            ];

            let names = make_unique(vaults);

            assert_eq!(names[0], "Downloads/vaults/personal");
            assert_eq!(names[1], "Documents/vaults/personal");
            assert_eq!(names[2], "Downloads/personal");
            assert_eq!(names[3], "Documents/personal");
            assert_eq!(names[4], "work");
        }
    }

    mod get_divergence {
        use crate::display_name::get_divergence;

        #[test]
        fn test_get_divergence() {
            let v1 = vec![1, 2, 3, 4];
            let v2 = vec![1, 2, 4, 4];

            let (v1_d, v2_d) = get_divergence(v1, v2);

            assert_eq!(v1_d[..v1_d.len() - 1], v2_d[..v2_d.len() - 1]);
            assert_eq!(v1_d.len(), 3);
            assert_eq!(v2_d.len(), 3);
            assert_ne!(v1_d.last(), v2_d.last());
        }

        #[test]
        fn test_get_divergence_identical() {
            let v1 = vec![1, 2, 3, 4];
            let v2 = vec![1, 2, 3, 4];

            let (v1_d, v2_d) = get_divergence(v1, v2);

            assert_eq!(v1_d, v2_d);
        }

        #[test]
        fn test_first_elem_diverge() {
            let v1 = vec![2, 2, 3, 4];
            let v2 = vec![1, 2, 3, 4];

            let (v1_d, v2_d) = get_divergence(v1, v2);

            assert_eq!(v1_d.len(), 1);
            assert_eq!(v2_d.len(), 1);
            assert_ne!(v1_d[0], v2_d[0]);
            assert_ne!(v1_d.last(), v2_d.last());
        }

        #[test]
        fn test_unequal_length() {
            let v1 = vec![1, 3];
            let v2 = vec![1, 2, 3, 4];

            let (v1_d, v2_d) = get_divergence(v1, v2);

            assert_eq!(v1_d.len(), 2);
            assert_eq!(v2_d.len(), 2);
            assert_eq!(v1_d[0], v2_d[0]);
            assert_ne!(v1_d.last(), v2_d.last());
        }
    }
}
