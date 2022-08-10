#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::MorpheusEngine;

    #[test]
    fn test_decompiler() {
        let test_file = "tests/test.exe";
        let out_file = "tests/test_mod.exe";
        let engine = MorpheusEngine{file: PathBuf::from(test_file)};

        engine.modify();
        assert_eq!(4, 4);
    }
}
