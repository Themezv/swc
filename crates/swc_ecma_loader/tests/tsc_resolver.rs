#![cfg(feature = "tsc")]

use std::collections::HashMap;

use anyhow::{anyhow, Error};
use swc_common::{collections::AHashMap, FileName};
use swc_ecma_loader::{
    resolve::{Resolution, Resolve},
    resolvers::tsc::TsConfigResolver,
};

#[test]
fn base_dir_exact() {}

#[test]
fn base_dir_wildcard() {}

#[test]
fn exact() {
    let mut map = HashMap::default();
    map.insert("jquery".to_string(), "fail".to_string());
    map.insert(
        "./node_modules/jquery/dist/jquery".to_string(),
        "success".to_string(),
    );
    let r = TsConfigResolver::new(
        TestResolver(map),
        ".".into(),
        vec![(
            "jquery".into(),
            vec!["node_modules/jquery/dist/jquery".into()],
        )],
    );

    {
        let resolved = r
            .resolve(&FileName::Anon, "jquery")
            .expect("should resolve");

        assert_eq!(
            resolved,
            Resolution {
                filename: FileName::Custom("success".into()),
                slug: None
            }
        );
    }

    {
        r.resolve(&FileName::Anon, "unrelated")
            .expect_err("should not touch error");
    }
}

#[test]
fn pattern_1() {
    let mut map = HashMap::default();
    map.insert("./folder1/file1".to_string(), "success-1".to_string());
    map.insert("./folder1/file2".to_string(), "success-2".to_string());
    map.insert(
        "./generated/folder2/file3".to_string(),
        "success-3".to_string(),
    );

    let r = TsConfigResolver::new(
        TestResolver(map),
        ".".into(),
        vec![("*".into(), vec!["*".into(), "generated/*".into()])],
    );

    {
        let resolved = r
            .resolve(&FileName::Anon, "folder1/file2")
            .expect("should resolve");

        assert_eq!(
            resolved,
            Resolution {
                filename: FileName::Custom("success-2".into()),
                slug: None
            }
        );
    }

    {
        let resolved = r
            .resolve(&FileName::Anon, "folder2/file3")
            .expect("should resolve");

        assert_eq!(
            resolved,
            Resolution {
                filename: FileName::Custom("success-3".into()),
                slug: None
            }
        );
    }
}

struct TestResolver(AHashMap<String, String>);

impl Resolve for TestResolver {
    fn resolve(&self, _: &FileName, src: &str) -> Result<Resolution, Error> {
        self.0
            .get(src)
            .cloned()
            .map(FileName::Custom)
            .map(|v| Resolution {
                filename: v,
                slug: None,
            })
            .ok_or_else(|| anyhow!("failed to resolve `{}`", src))
    }
}
