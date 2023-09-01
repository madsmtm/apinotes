//! Test a few headers from LLVM's test suite:
//! https://github.com/llvm/llvm-project/tree/llvmorg-15.0.4/clang/test/APINotes
//!
//! Put here instead of in the usual `tests/` directory since we want to be
//! able to easily construct the values, but most of them are marked with
//! `#[non_exhaustive]`.
use std::path::Path;

use crate::{
    ApiNotes, Class, Data, Function, General, Global, Kind, Map, Method, MethodKey, Nullability,
    Parameter, Property, PropertyKey, Protocol, Tag, Version,
};

impl MethodKey {
    fn new(selector: &str, kind: Kind) -> Self {
        Self {
            selector: selector.into(),
            kind,
        }
    }
}

impl PropertyKey {
    fn new(name: &str, kind: Kind) -> Self {
        Self {
            name: name.into(),
            kind: Some(kind),
        }
    }
}

fn assert_eq_to_file(expected: ApiNotes, name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("clang_tests")
        .join(name);

    let actual = ApiNotes::from_path(&path).expect("ApiNotes::from_path");
    if expected != actual {
        if expected.swift_versions != actual.swift_versions {
            panic!(
                "expected swift version: {:#?}\n\ngot {:#?}",
                expected.swift_versions, actual.swift_versions
            );
        }
        if expected.data != actual.data {
            if expected.data.classes != actual.data.classes {
                panic!(
                    "expected classes: {:#?}\n\ngot {:#?}",
                    expected.data.classes, actual.data.classes
                );
            }
            if expected.data.protocols != actual.data.protocols {
                panic!(
                    "expected protocols: {:#?}\n\ngot {:#?}",
                    expected.data.protocols, actual.data.protocols
                );
            }
            if expected.data.tags != actual.data.tags {
                panic!(
                    "expected tags: {:#?}\n\ngot {:#?}",
                    expected.data.tags, actual.data.tags
                );
            }
            if expected.data.typedefs != actual.data.typedefs {
                panic!(
                    "expected typedefs: {:#?}\n\ngot {:#?}",
                    expected.data.typedefs, actual.data.typedefs
                );
            }
            if expected.data.globals != actual.data.globals {
                panic!(
                    "expected globals: {:#?}\n\ngot {:#?}",
                    expected.data.globals, actual.data.globals
                );
            }
            if expected.data.enumerators != actual.data.enumerators {
                panic!(
                    "expected enumerators: {:#?}\n\ngot {:#?}",
                    expected.data.enumerators, actual.data.enumerators
                );
            }
            if expected.data.functions != actual.data.functions {
                panic!(
                    "expected functions: {:#?}\n\ngot {:#?}",
                    expected.data.functions, actual.data.functions
                );
            }
        }
        panic!(
            "expected: {:#?}\n\ngot {:#?}",
            expected.data.classes, actual.data.classes
        );
    }
}

fn default<T: Default>() -> T {
    Default::default()
}

#[test]
fn simple() {
    let properties: Map<_, _> = [
        (
            PropertyKey::new("nonnullProperty", Kind::Class),
            Property {
                nullability: Some(Nullability::Nonnull),
                ..default()
            },
        ),
        (
            PropertyKey::new("nonnullNewProperty", Kind::Class),
            Property {
                nullability: Some(Nullability::Nonnull),
                ..default()
            },
        ),
        (
            PropertyKey::new("optionalProperty", Kind::Class),
            Property {
                nullability: Some(Nullability::Optional),
                ..default()
            },
        ),
        (
            PropertyKey::new("optionalNewProperty", Kind::Class),
            Property {
                nullability: Some(Nullability::Optional),
                ..default()
            },
        ),
        (
            PropertyKey::new("unspecifiedProperty", Kind::Instance),
            Property {
                nullability: Some(Nullability::Unspecified),
                ..default()
            },
        ),
        (
            PropertyKey::new("unspecifiedNewProperty", Kind::Instance),
            Property {
                nullability: Some(Nullability::Unspecified),
                ..default()
            },
        ),
        (
            PropertyKey::new("scalarProperty", Kind::Instance),
            Property {
                nullability: Some(Nullability::Scalar),
                ..default()
            },
        ),
        (
            PropertyKey::new("scalarNewProperty", Kind::Instance),
            Property {
                nullability: Some(Nullability::Scalar),
                ..default()
            },
        ),
    ]
    .into();
    let expected = ApiNotes {
        name: "SimpleKit".into(),
        swift_versions: default(),
        data: Data {
            classes: [(
                "I".into(),
                Class {
                    properties,
                    ..default()
                },
            )]
            .into(),
            ..default()
        },
    };

    assert_eq_to_file(expected, "Simple.apinotes");
}

#[test]
fn simple_kit() {
    let expected = ApiNotes {
        name: "SimpleKit".into(),
        data: Data {
            classes: [(
                "MethodTest".into(),
                Class {
                    methods: [
                        (
                            MethodKey::new("getOwnedToUnowned", Kind::Instance),
                            Method { ..default() },
                        ),
                        (
                            MethodKey::new("getUnownedToOwned", Kind::Instance),
                            Method { ..default() },
                        ),
                    ]
                    .into(),
                    ..default()
                },
            )]
            .into(),
            tags: [
                (
                    "RenamedAgainInAPINotesA".into(),
                    Tag {
                        general: General {
                            swift_name: Some("SuccessfullyRenamedA".into()),
                            ..default()
                        },
                        ..default()
                    },
                ),
                (
                    "RenamedAgainInAPINotesB".into(),
                    Tag {
                        general: General {
                            swift_name: Some("SuccessfullyRenamedB".into()),
                            ..default()
                        },
                        ..default()
                    },
                ),
            ]
            .into(),
            functions: [
                ("getCFOwnedToUnowned".into(), Function { ..default() }),
                ("getCFUnownedToOwned".into(), Function { ..default() }),
                ("getCFOwnedToNone".into(), Function { ..default() }),
                ("getObjCOwnedToUnowned".into(), Function { ..default() }),
                ("getObjCUnownedToOwned".into(), Function { ..default() }),
                (
                    "indirectGetCFOwnedToUnowned".into(),
                    Function {
                        parameters: [(0, Parameter { ..default() })].into(),
                        ..default()
                    },
                ),
                (
                    "indirectGetCFUnownedToOwned".into(),
                    Function {
                        parameters: [(0, Parameter { ..default() })].into(),
                        ..default()
                    },
                ),
                (
                    "indirectGetCFOwnedToNone".into(),
                    Function {
                        parameters: [(0, Parameter { ..default() })].into(),
                        ..default()
                    },
                ),
                (
                    "indirectGetCFNoneToOwned".into(),
                    Function {
                        parameters: [(0, Parameter { ..default() })].into(),
                        ..default()
                    },
                ),
                (
                    "getCFAuditedToUnowned_DUMP".into(),
                    Function { ..default() },
                ),
                ("getCFAuditedToOwned_DUMP".into(), Function { ..default() }),
                ("getCFAuditedToNone_DUMP".into(), Function { ..default() }),
            ]
            .into(),
            ..default()
        },
        ..default()
    };

    assert_eq_to_file(expected, "SimpleKit.apinotes");
}

#[test]
fn some_kit() {
    let expected = ApiNotes {
        name: "SomeKit".into(),
        data: Data {
            classes: [
                (
                    "A".into(),
                    Class {
                        methods: [
                            (
                                MethodKey::new("transform:", Kind::Instance),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("anything but this".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("transform:integer:", Kind::Instance),
                                Method {
                                    nullability_of_ret: Some(Nullability::Nonnull),
                                    nullability: Some(vec![
                                        Nullability::Nonnull,
                                        Nullability::Scalar,
                                    ]),
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("implicitGetOnlyInstance", Kind::Instance),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("getter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("implicitGetOnlyClass", Kind::Class),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("getter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("implicitGetSetInstance", Kind::Instance),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("getter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("implicitGetSetClass", Kind::Class),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("getter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("setImplicitGetSetInstance:", Kind::Instance),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("setter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                MethodKey::new("setImplicitGetSetClass:", Kind::Class),
                                Method {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("setter gone".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                        ]
                        .into(),
                        properties: [
                            (
                                PropertyKey::new("intValue", Kind::Instance),
                                Property {
                                    general: General {
                                        availability: Some("none".into()),
                                        availability_msg: Some("wouldn't work anyway".into()),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ),
                            (
                                PropertyKey::new("nonnullAInstance", Kind::Instance),
                                Property {
                                    nullability: Some(Nullability::Nonnull),
                                    ..default()
                                },
                            ),
                            (
                                PropertyKey::new("nonnullAClass", Kind::Class),
                                Property {
                                    nullability: Some(Nullability::Nonnull),
                                    ..default()
                                },
                            ),
                            (
                                PropertyKey {
                                    name: "nonnullABoth".into(),
                                    kind: None,
                                },
                                Property {
                                    nullability: Some(Nullability::Nonnull),
                                    ..default()
                                },
                            ),
                        ]
                        .into(),
                        ..default()
                    },
                ),
                (
                    "B".into(),
                    Class {
                        general: General {
                            availability: Some("none".into()),
                            availability_msg: Some("just don't".into()),
                            ..default()
                        },
                        ..default()
                    },
                ),
                (
                    "C".into(),
                    Class {
                        methods: [(
                            MethodKey::new("initWithA:", Kind::Instance),
                            Method {
                                designated_init: true,
                                ..default()
                            },
                        )]
                        .into(),
                        ..default()
                    },
                ),
                (
                    "OverriddenTypes".into(),
                    Class {
                        methods: [(
                            MethodKey::new("methodToMangle:second:", Kind::Instance),
                            Method {
                                result_type: Some("char *".into()),
                                parameters: [
                                    (
                                        0,
                                        Parameter {
                                            type_: Some("SOMEKIT_DOUBLE *".into()),
                                            ..default()
                                        },
                                    ),
                                    (
                                        1,
                                        Parameter {
                                            type_: Some("float *".into()),
                                            ..default()
                                        },
                                    ),
                                ]
                                .into(),
                                ..default()
                            },
                        )]
                        .into(),
                        properties: [(
                            PropertyKey::new("intPropertyToMangle", Kind::Instance),
                            Property {
                                type_: Some("double *".into()),
                                ..default()
                            },
                        )]
                        .into(),
                        ..default()
                    },
                ),
            ]
            .into(),
            functions: [(
                "global_int_fun".into(),
                Function {
                    result_type: Some("char *".into()),
                    parameters: [
                        (
                            0,
                            Parameter {
                                type_: Some("double *".into()),
                                ..default()
                            },
                        ),
                        (
                            1,
                            Parameter {
                                type_: Some("float *".into()),
                                ..default()
                            },
                        ),
                    ]
                    .into(),
                    ..default()
                },
            )]
            .into(),
            globals: [(
                "global_int_ptr".into(),
                Global {
                    type_: Some("double (*)(int, int)".into()),
                    ..default()
                },
            )]
            .into(),
            ..default()
        },
        swift_versions: [(
            Version::V3,
            Data {
                classes: [(
                    "A".into(),
                    Class {
                        methods: [(
                            MethodKey::new("transform:integer:", Kind::Instance),
                            Method {
                                nullability_of_ret: Some(Nullability::Optional),
                                nullability: Some(vec![Nullability::Optional, Nullability::Scalar]),
                                ..default()
                            },
                        )]
                        .into(),
                        properties: [
                            (
                                PropertyKey::new("explicitNonnullInstance", Kind::Instance),
                                Property {
                                    nullability: Some(Nullability::Optional),
                                    ..default()
                                },
                            ),
                            (
                                PropertyKey::new("explicitNullableInstance", Kind::Instance),
                                Property {
                                    nullability: Some(Nullability::Nonnull),
                                    ..default()
                                },
                            ),
                        ]
                        .into(),
                        ..default()
                    },
                )]
                .into(),
                ..default()
            },
        )]
        .into(),
        ..default()
    };

    assert_eq_to_file(expected, "SomeKit.apinotes");
}

#[test]
fn some_kit_private() {
    let expected = ApiNotes {
        name: "SomeKit".into(),
        data: Data {
            classes: [(
                "A".into(),
                Class {
                    methods: [(
                        MethodKey::new("privateTransform:input:", Kind::Instance),
                        Method {
                            nullability_of_ret: Some(Nullability::Nonnull),
                            nullability: Some(vec![Nullability::Nonnull, Nullability::Scalar]),
                            ..default()
                        },
                    )]
                    .into(),
                    properties: [(
                        PropertyKey {
                            name: "internalProperty".into(),
                            kind: None,
                        },
                        Property {
                            nullability: Some(Nullability::Nonnull),
                            ..default()
                        },
                    )]
                    .into(),
                    ..default()
                },
            )]
            .into(),
            protocols: [(
                "InternalProtocol".into(),
                Protocol {
                    general: General {
                        availability: Some("none".into()),
                        availability_msg: Some("not for you".into()),
                        ..default()
                    },
                    ..default()
                },
            )]
            .into(),
            ..default()
        },
        ..default()
    };

    assert_eq_to_file(expected, "SomeKit_private.apinotes");
}
