pub use basin_storage::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod basin_storage {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUB_ADMIN_ROLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PUB_ADMIN_ROLE"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addDeals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addDeals"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct BasinStorage.DealInfo[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createPub"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createPub"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealActivation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealActivation"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealClient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealClient"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealClientCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealClientCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct CommonTypes.BigInt",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealLabel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealLabel"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealProvider"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealProvider"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealProviderCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealProviderCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct CommonTypes.BigInt",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealTerm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealTerm"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealTotalPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealTotalPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct CommonTypes.BigInt",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dealVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dealVerified"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dealID"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("role"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("grantRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestNDeals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestNDeals"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("n"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BasinStorage.DealInfo[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paginatedDeals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("paginatedDeals"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("offset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("limit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BasinStorage.DealInfo[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pubsOfOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pubsOfOwner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DealAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DealAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dealId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("cid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PubCreated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pub"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActorError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ActorError"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("errorCode"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ActorNotFound"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ActorNotFound"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DealEpochAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DealEpochAlreadyExists",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("epoch"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailToCallActor"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailToCallActor"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCodec"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidCodec"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidResponseLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidResponseLength",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotEnoughBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PubAlreadyExists"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("reason"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubDoesNotExist"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PubDoesNotExist"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("reason"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASINSTORAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1F`\x003b\0\0%V[b\0\0\xC6V[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\xC2W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\x813\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a/s\x80b\0\0\xD6`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cW\x08\x92\xC8\x11a\0\xB8W\x80c\x89\xEC\x0B\x93\x11a\0|W\x80c\x89\xEC\x0B\x93\x14a\x03'W\x80c\x91\xD1HT\x14a\x03:W\x80c\x9F)7\x0B\x14a\x03MW\x80c\xA2\x17\xFD\xDF\x14a\x03`W\x80c\xD0oh\x02\x14a\x03hW\x80c\xD5Gt\x1F\x14a\x03{W`\0\x80\xFD[\x80cW\x08\x92\xC8\x14a\x02\x8DW\x80cY\xB6L]\x14a\x02\xA0W\x80co\nC\xC7\x14a\x02\xC0W\x80c\x82+\xA4\x0B\x14a\x02\xD3W\x80c\x87\xA4\x1B\x81\x14a\x02\xFAW`\0\x80\xFD[\x80c//\xF1]\x11a\x01\nW\x80c//\xF1]\x14a\x02\x0CW\x80c6V\x8A\xBE\x14a\x02!W\x80c<~Y\x99\x14a\x024W\x80c?\xF4!\xE9\x14a\x02TW\x80cHMZ:\x14a\x02gW\x80cR\xB6+>\x14a\x02zW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01GW\x80c\x06\xA0\x9D\xEA\x14a\x01oW\x80c\x12\x1Eb\x0E\x14a\x01\x9AW\x80c$\x8A\x9C\xA3\x14a\x01\xBBW\x80c&)Jw\x14a\x01\xECW[`\0\x80\xFD[a\x01Za\x01U6`\x04a$IV[a\x03\x8EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x82a\x01}6`\x04a$\x88V[a\x03\xC5V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01fV[a\x01\xADa\x01\xA86`\x04a$\x88V[a\x03\xD0V[`@Qa\x01f\x92\x91\x90a$\xF5V[a\x01\xDEa\x01\xC96`\x04a%\x19V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01fV[a\x01\xFFa\x01\xFA6`\x04a%NV[a\x03\xF1V[`@Qa\x01f\x91\x90a%iV[a\x02\x1Fa\x02\x1A6`\x04a%\xCBV[a\x04\xE0V[\0[a\x02\x1Fa\x02/6`\x04a%\xCBV[a\x05\nV[a\x02Ga\x02B6`\x04a$\x88V[a\x05\x8DV[`@Qa\x01f\x91\x90a%\xF7V[a\x01Za\x02b6`\x04a$\x88V[a\x05\xABV[a\x02Ga\x02u6`\x04a$\x88V[a\x05\xB6V[a\x02\x1Fa\x02\x886`\x04a&lV[a\x05\xD4V[a\x02\x1Fa\x02\x9B6`\x04a&\xBEV[a\x07\rV[a\x02\xB3a\x02\xAE6`\x04a'WV[a\t'V[`@Qa\x01f\x91\x90a'\xA7V[a\x02\xB3a\x02\xCE6`\x04a(AV[a\tuV[a\x01\xDE\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x81V[a\x03\ra\x03\x086`\x04a$\x88V[a\t\xC2V[`@\x80Q`\x07\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x01fV[a\x02Ga\x0356`\x04a$\x88V[a\t\xD0V[a\x01Za\x03H6`\x04a%\xCBV[a\t\xEEV[a\x03\ra\x03[6`\x04a$\x88V[a\n\x17V[a\x01\xDE`\0\x81V[a\x01\x82a\x03v6`\x04a$\x88V[a\n%V[a\x02\x1Fa\x03\x896`\x04a%\xCBV[a\n0V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\xBFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0a\x03\xBF\x82a\nUV[```\0\x80a\x03\xDE\x84a\n\x93V[\x80Q` \x90\x91\x01Q\x90\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x04\xD5W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04H\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04t\x90a(\x8CV[\x80\x15a\x04\xC1W\x80`\x1F\x10a\x04\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04)V[PPPP\x90P\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x04\xFB\x81a\n\xDDV[a\x05\x05\x83\x83a\n\xEAV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x05\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\x89\x82\x82a\x0BnV[PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0B\xD3V[`\0a\x03\xBF\x82a\x0C\x1DV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0CSV[\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95a\x05\xFE\x81a\n\xDDV[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x01\x84\x84`@Qa\x06\x1B\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06VW\x82\x82`@Qc\\x\xF6\xED`\xE1\x1B\x81R`\x04\x01a\x05v\x92\x91\x90a(\xD6V[\x83`\x01\x84\x84`@Qa\x06i\x92\x91\x90a(\xC6V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x86\x16`\0\x90\x81R`\x02\x82R\x91\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01a\x06\xBC\x83\x85\x83a)iV[P\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x06\xD7\x92\x91\x90a(\xC6V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF8\xDE\xBC/\x17E\xEB\xA8i\t\x89\x0F-\xC0abG\x05\xC7C)4\x88)\xE0J\xBAC\xC0\x15\xB9\xA2\x90`\0\x90\xA3PPPPV[\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95a\x077\x81a\n\xDDV[`\0C\x90P`\0`\x01\x87\x87`@Qa\x07P\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80a\x07\x8DW\x86\x86`@Qc\x15\xE6\xE0\xEB`\xE2\x1B\x81R`\x04\x01a\x05v\x92\x91\x90a(\xD6V[`\0[\x84\x81\x10\x15a\t\x1DW`\x04\x88\x88`@Qa\x07\xAA\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x86\x86\x83\x81\x81\x10a\x07\xDBWa\x07\xDBa*)V[\x90P` \x02\x81\x01\x90a\x07\xED\x91\x90a*?V[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x90\x92 \x90\x91`\x03\x02\x01a\x08\x0F\x82\x82a*\xA5V[PP`\x03\x88\x88`@Qa\x08#\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x90`\0a\x08@\x83a+\xC6V[\x91\x90PUP\x81`\x01`\x01`\xA0\x1B\x03\x16\x88\x88`@Qa\x08_\x92\x91\x90a(\xC6V[`@Q\x80\x91\x03\x90 \x87\x87\x84\x81\x81\x10a\x08yWa\x08ya*)V[\x90P` \x02\x81\x01\x90a\x08\x8B\x91\x90a*?V[a\x08\x99\x90` \x81\x01\x90a$\x88V[`\x01`\x01`@\x1B\x03\x16\x7Fy\xB3\x877\xA5?\x0B\xC2.4\x83sU\xA5\xC9\xD1\xBF\xCD\xDF\x81|\xB0\x02/\x82\xFE\x93\x8B\x05\x88\xBF?\x89\x89\x86\x81\x81\x10a\x08\xD5Wa\x08\xD5a*)V[\x90P` \x02\x81\x01\x90a\x08\xE7\x91\x90a*?V[a\x08\xF5\x90`@\x81\x01\x90a*_V[`@Qa\t\x03\x92\x91\x90a(\xD6V[`@Q\x80\x91\x03\x90\xA4\x80a\t\x15\x81a+\xC6V[\x91PPa\x07\x90V[PPPPPPPPV[```\0`\x03\x86\x86`@Qa\t=\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x80\x83\x11a\t[W\x82a\t]V[\x80[\x92Pa\tk\x86\x86\x86\x86a\x0C\x92V[\x96\x95PPPPPPV[```\0`\x03\x85\x85`@Qa\t\x8B\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x80\x83\x11a\t\xA9W\x82a\t\xABV[\x80[\x92Pa\t\xB9\x85\x85C\x86a\x0C\x92V[\x95\x94PPPPPV[`\0\x80`\0a\x03\xDE\x84a\x0FlV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0F\xB5V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80`\0a\x03\xDE\x84a\x0F\xF4V[`\0a\x03\xBF\x82a\x102V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\nK\x81a\n\xDDV[a\x05\x05\x83\x83a\x0BnV[`\0\x80a\nj\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\x80`\x05c\x07\xA1\xF0Q`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x10\xBAV[\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\n\xBC\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\xD2`\x05c\x02\xC3s\x86`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x10\xD1V[a\n\xE7\x813a\x11\x08V[PV[a\n\xF4\x82\x82a\t\xEEV[a\x05\x89W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0B*3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x0Bx\x82\x82a\t\xEEV[\x15a\x05\x89W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0B\xFC\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\xB2\x05\x9CI`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11aV[`\0\x80a\x0C2\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0CH`\x05c\x9C\x9A\xC8\x19`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11\xC5V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0C|\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\xFF\x88\xE8<`Q\x85a\x10\x95V[```\0\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xAFWa\x0C\xAFa)\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\rW\x81` \x01[a\x0C\xFA`@Q\x80``\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xCDW\x90P[P\x90P\x84[\x84\x83\x10\x15a\x0F_W`\0`\x04\x89\x89`@Qa\r.\x92\x91\x90a(\xC6V[\x90\x81R`@\x80Q\x91\x82\x90\x03` \x90\x81\x01\x83 `\0\x86\x81R\x90\x82R\x82\x81 \x80T\x80\x84\x02\x86\x01\x84\x01\x90\x94R\x83\x85R\x92\x91\x84\x01[\x82\x82\x10\x15a\x0E\xCBW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x85\x02\x90\x91\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\r\xA8\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xD4\x90a(\x8CV[\x80\x15a\x0E!W\x80`\x1F\x10a\r\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0E:\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ef\x90a(\x8CV[\x80\x15a\x0E\xB3W\x80`\x1F\x10a\x0E\x88Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xB3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x96W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r_V[PPPP\x90P`\0[\x81Q\x81\x10\x15a\x0F=W\x86\x85\x10\x15a\x0F=W\x81\x81\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a*)V[` \x02` \x01\x01Q\x84\x86\x81Q\x81\x10a\x0F\x11Wa\x0F\x11a*)V[` \x02` \x01\x01\x81\x90RP\x84\x80a\x0F'\x90a+\xC6V[\x95PP\x80\x80a\x0F5\x90a+\xC6V[\x91PPa\x0E\xD4V[P\x81`\0\x03a\x0FLWPa\x0F_V[\x81a\x0FV\x81a+\xDFV[\x92PPPa\r\x12V[P\x90\x81R\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x0F\x94\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0F\xAA`\x05c\t\xC3\x0B `Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11\xD3V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0F\xDE\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\x0B\xF4lW`Q\x85a\x10\x95V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x10\x1C\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0F\xAA`\x05c\x99\x04\xF2\xFF`Q\x85a\x10\x95V[`\0\x80a\x10G\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\x80`\x05c7\xBC6\xDA`Q\x85a\x10\x95V[```\0a\x10s\x83`\x01`\x01`@\x1B\x03\x16a\x127V[\x90P`\0a\x10\x80\x82a\x12\x85V[\x90Pa\x10\x8C\x81\x85a\x12\xA6V[a\n\x8B\x81a\x12\xB2V[``a\x12\xFD\x80a\x10\xAF\x87\x87\x87\x87`\0`\x01c\xFF\xFF\xFF\xFF\x88\x16V[\x97\x96PPPPPPPV[`\0\x80\x80a\x10\xC8\x84\x82a\x13\xD7V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01\x81\x90R\x90a\x10\xC8\x84\x83a\x14dV[a\x11\x12\x82\x82a\t\xEEV[a\x05\x89Wa\x11\x1F\x81a\x16\x12V[a\x11*\x83` a\x16$V[`@Q` \x01a\x11;\x92\x91\x90a+\xF6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05v\x91`\x04\x01a,kV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R`\0` \x83\x01\x81\x90R\x83Q\x15a\x11\x9FWa\x11\x8A\x84\x82a\x17\xC6V[\x81Q\x91\x93P\x91P\x15a\x11\x9FWa\n\x8B\x82a\x19kV[PP`@\x80Q`\0\x81\x83\x01\x81\x81R``\x83\x01\x90\x93R\x91\x81R` \x81\x01\x91\x90\x91R\x92\x91PPV[`\0\x80\x80a\x10\xC8\x84\x82a\x1A\xC7V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80a\x11\xF4\x84\x82a\x1BbV[\x92P\x90P`\x02\x81\x14a\x12\x08Wa\x12\x08a,~V[a\x12\x12\x84\x83a\x1B\xD5V[`\x07\x91\x90\x91\x0B\x84R\x91Pa\x12&\x84\x83a\x1B\xD5V[P`\x07\x0B` \x84\x01RP\x90\x92\x91PPV[`\0`\x17\x82\x11a\x12IWP`\x01\x91\x90PV[`\xFF\x82\x11a\x12YWP`\x02\x91\x90PV[a\xFF\xFF\x82\x11a\x12jWP`\x03\x91\x90PV[c\xFF\xFF\xFF\xFF\x82\x11a\x12}WP`\x05\x91\x90PV[P`\t\x91\x90PV[a\x12\x8Da$\x14V[\x80Qa\x12\x99\x90\x83a\x1B\xEFV[P`\0` \x82\x01R\x91\x90PV[a\x05\x89\x82`\0\x83a\x1CfV[``\x81` \x01Q`\0\x14a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk$\xB7;0\xB64\xB2\x10!\xA1'\xA9`\xA1\x1B`D\x82\x01R`d\x01a\x05vV[PQQ\x90V[``a\x13\x10`\x05`\x7F`\x99\x1B\x01\x84a\x1D\x85V[`\0\x80`\x05`\x7F`\x99\x1B\x01\x88\x86\x86a\x13)W`\0a\x13,V[`\x01[\x8A\x8A\x8E`@Q` \x01a\x13D\x96\x95\x94\x93\x92\x91\x90a,\x94V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13^\x91a,\xE3V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x13\x99W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13\x9EV[``\x91P[P\x91P\x91P\x81a\x13\xC1W`@Qc\x8A}\xB5\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xCA\x81a\x1D\xD3V[\x99\x98PPPPPPPPPV[`\0\x80`\0\x80a\x13\xE7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16\x15a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Finvalid maj (expected MajUnsigne`D\x82\x01RddInt)`\xD8\x1B`d\x82\x01R`\x84\x01a\x05vV[\x92P\x83\x91PP[\x92P\x92\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0\x80`\0a\x14\x88\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x02\x14\x80a\x14\xADWP`\xFF\x82\x16`\x03\x14[a\x15\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7Finvalid maj (expected MajByteStr`D\x82\x01Rting or MajTextString)`X\x1B`d\x82\x01R`\x84\x01a\x05vV[`\0a\x15#\x82\x87a,\xF5V[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15?Wa\x15?a)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15iW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x87[\x83\x81\x10\x15a\x15\xDEW\x89\x81\x81Q\x81\x10a\x15\x8AWa\x15\x8Aa*)V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x15\xA7Wa\x15\xA7a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81a\x15\xC8\x81a+\xC6V[\x92PP\x80\x80a\x15\xD6\x90a+\xC6V[\x91PPa\x15pV[P`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R`\xFF\x86\x16`\x03\x14` \x82\x01Ra\x16\x02\x85\x8Aa,\xF5V[\x96P\x96PPPPPP\x92P\x92\x90PV[``a\x03\xBF`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x163\x83`\x02a-\x08V[a\x16>\x90`\x02a,\xF5V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16UWa\x16Ua)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16\x7FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x16\x9AWa\x16\x9Aa*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x16\xC9Wa\x16\xC9a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x16\xED\x84`\x02a-\x08V[a\x16\xF8\x90`\x01a,\xF5V[\x90P[`\x01\x81\x11\x15a\x17pWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x17,Wa\x17,a*)V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x17BWa\x17Ba*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x17i\x81a+\xDFV[\x90Pa\x16\xFBV[P\x83\x15a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05vV[\x93\x92PPPV[```\0\x80`\0a\x17\xD7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x06\x14\x80a\x17\xFCWP`\xFF\x82\x16`\x02\x14[a\x18_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7Finvalid maj (expected MajTag or `D\x82\x01RmMajByteString)`\x90\x1B`d\x82\x01R`\x84\x01a\x05vV[`\x05\x19`\xFF\x83\x16\x01a\x18\x98Wa\x18u\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x02\x14a\x18\x98Wa\x18\x98a,~V[`\0a\x18\xA4\x82\x87a,\xF5V[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xC0Wa\x18\xC0a)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x87[\x83\x81\x10\x15a\x19_W\x89\x81\x81Q\x81\x10a\x19\x0BWa\x19\x0Ba*)V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x19(Wa\x19(a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81a\x19I\x81a+\xC6V[\x92PP\x80\x80a\x19W\x90a+\xC6V[\x91PPa\x18\xF1V[P\x81a\x16\x02\x85\x8Aa,\xF5V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81Q`\0\x03a\x19\xB4WPP`@\x80Q`\x80\x81\x01\x82R`\x01\x91\x81\x01\x91\x82R`\0``\x82\x01\x81\x90R\x91\x81R` \x81\x01\x91\x90\x91R\x90V[`\0`\x01\x83Qa\x19\xC4\x91\x90a-\x1FV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDBWa\x19\xDBa)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A\x05W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x83`\0\x81Q\x81\x10a\x1A\x1DWa\x1A\x1Da*)V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\x01`\xF8\x1B\x03a\x1A:WP`\x01[`\x01[\x84Q\x81\x10\x15a\x1A\xACW\x84\x81\x81Q\x81\x10a\x1AXWa\x1AXa*)V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x1As`\x01\x84a-\x1FV[\x81Q\x81\x10a\x1A\x83Wa\x1A\x83a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\xA4\x81a+\xC6V[\x91PPa\x1A=V[P`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R\x15\x15` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x1A\xD7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x07\x14a\x1B:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid maj (expected MajOther)\0`D\x82\x01R`d\x01a\x05vV[`\x15\x81\x14\x80a\x1BIWP`\x14\x81\x14[a\x1BUWa\x1BUa,~V[`\x14\x14\x15\x95\x93\x94PPPPV[`\0\x80`\0\x80a\x1Br\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x04\x14a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid maj (expected MajArray)\0`D\x82\x01R`d\x01a\x05vV[`\0\x80\x80a\x1B\xE3\x85\x85a \x90V[\x90\x96\x90\x95P\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x1C\x0F` \x83a-2V[\x15a\x1C7Wa\x1C\x1F` \x83a-2V[a\x1C*\x90` a-\x1FV[a\x1C4\x90\x83a,\xF5V[\x91P[` \x80\x84\x01\x83\x90R`@Q\x80\x85R`\0\x81R\x90\x81\x84\x01\x01\x81\x81\x10\x15a\x1C[W`\0\x80\xFD[`@RP\x91\x92\x91PPV[`\x17\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1C\x92W\x82Qa\x1C\x8C\x90`\xE0`\x05\x85\x90\x1B\x16\x83\x17a!-V[PPPPV[`\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1C\xD2W\x82Qa\x1C\xBA\x90`\x18a\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x01a!\x96V[a\xFF\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1D\x13W\x82Qa\x1C\xFB\x90`\x19a\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x02a!\x96V[c\xFF\xFF\xFF\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1DVW\x82Qa\x1D>\x90`\x1Aa\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x04a!\x96V[\x82Qa\x1Dm\x90`\x1Ba\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x08a!\x96V[G\x81\x81\x10\x15a\x1D\xB1W`@QcG\x87\xA1\x03`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x05vV[\x82?\x15\x15\x80a\x1C\x8CW`@Qc\x06M\x95K`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0\x80`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x1D\xEE\x91\x90a-TV[\x91\x94P\x92P\x90P`\x01`\x01`@\x1B\x03\x82\x16a\x1E(W\x80Q\x15a\x1E#W`@Qc\x0Et\x99\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x94V[`\x01`\x01`@\x1B\x03\x82\x16`Q\x14\x80a\x1EIWP`\x01`\x01`@\x1B\x03\x82\x16`q\x14[\x15a\x1EpW\x80Q`\0\x03a\x1E#W`@Qc\x0Et\x99\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xF1\xF6\xBC\xED`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x05vV[\x82\x15a\n\x8BW`@Qc\xD4\xBBfq`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05vV[`\0\x80`\0\x80a\x1E\xC6\x86\x86a\"\x1BV[\x90Pa\x1E\xD3`\x01\x86a,\xF5V[\x94P`\x07`\x05\x82\x90\x1C\x16`\x1F\x82\x16`\x1C\x81\x10a\x1F?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fcannot handle headers with extra`D\x82\x01Rd > 27`\xD8\x1B`d\x82\x01R`\x84\x01a\x05vV[`\x18\x81`\xFF\x16\x10\x15a\x1F]W\x90\x94P`\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x18\x03a\x1F\xD9W`\0a\x1Ft\x89\x89a\"\x1BV[\x90Pa\x1F\x81`\x01\x89a,\xF5V[\x97P`\x18\x81`\xFF\x16\x10\x15a\x1F\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk4\xB7;0\xB64\xB2\x101\xB17\xB9`\xA1\x1B`D\x82\x01R`d\x01a\x05vV[\x91\x95PP`\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x19\x03a \x13W`\0a\x1F\xF0\x89\x89a\"jV[\x90Pa\x1F\xFD`\x02\x89a,\xF5V[\x97P\x91\x95PPa\xFF\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x1A\x03a OW`\0a *\x89\x89a\"\xA3V[\x90Pa 7`\x04\x89a,\xF5V[\x97P\x91\x95PPc\xFF\xFF\xFF\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x1B\x14a bWa ba,~V[`\0a n\x89\x89a\"\xDCV[\x90Pa {`\x08\x89a,\xF5V[\x97P\x91\x95P\x90\x93P\x85\x92PPP[\x92P\x92P\x92V[`\0\x80`\0\x80a \xA0\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x01\x14\x80a \xC3WP`\xFF\x82\x16\x15[a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7Finvalid maj (expected MajSignedI`D\x82\x01Rtnt or MajUnsignedInt)`X\x1B`d\x82\x01R`\x84\x01a\x05vV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x82QQ`\0a!R\x82`\x01a,\xF5V[\x90P\x84` \x01Q\x82\x10a!sWa!s\x85a!n\x83`\x02a-\x08V[a#\x15V[\x84Q` \x83\x82\x01\x01\x85\x81SP\x80Q\x82\x11\x15a!\x8CW\x81\x81R[P\x93\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x83QQ`\0a!\xBA\x82\x85a,\xF5V[\x90P\x85` \x01Q\x81\x11\x15a!\xD7Wa!\xD7\x86a!n\x83`\x02a-\x08V[`\0`\x01a!\xE7\x86a\x01\0a/\x03V[a!\xF1\x91\x90a-\x1FV[\x90P\x86Q\x82\x81\x01\x87\x83\x19\x82Q\x16\x17\x81RP\x80Q\x83\x11\x15a\"\x0FW\x82\x81R[P\x95\x96\x95PPPPPPV[`\0a\"(\x82`\x01a,\xF5V[\x83Q\x10\x15a\"HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[\x82\x82\x81Q\x81\x10a\"ZWa\"Za*)V[\x01` \x01Q`\xF8\x1C\x90P\x92\x91PPV[`\0a\"w\x82`\x02a,\xF5V[\x83Q\x10\x15a\"\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xF0\x1C\x90V[`\0a\"\xB0\x82`\x04a,\xF5V[\x83Q\x10\x15a\"\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xE0\x1C\x90V[`\0a\"\xE9\x82`\x08a,\xF5V[\x83Q\x10\x15a#\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xC0\x1C\x90V[\x81Qa#!\x83\x83a\x1B\xEFV[Pa\x1C\x8C\x83\x82`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x17\xBF\x83\x83\x84Q`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x82Q\x82\x11\x15a#fW`\0\x80\xFD[\x83QQ`\0a#u\x84\x83a,\xF5V[\x90P\x85` \x01Q\x81\x11\x15a#\x92Wa#\x92\x86a!n\x83`\x02a-\x08V[\x85Q\x80Q\x83\x82\x01` \x01\x91`\0\x91\x80\x85\x11\x15a#\xACW\x84\x82R[PPP` \x86\x01[` \x86\x10a#\xECW\x80Q\x82Ra#\xCB` \x83a,\xF5V[\x91Pa#\xD8` \x82a,\xF5V[\x90Pa#\xE5` \x87a-\x1FV[\x95Pa#\xB4V[Q\x81Q`\0\x19` \x88\x90\x03a\x01\0\n\x01\x90\x81\x16\x90\x19\x91\x90\x91\x16\x17\x90RP\x84\x91PP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80a$<`@Q\x80`@\x01`@R\x80``\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a$[W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17\xBFW`\0\x80\xFD[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\n\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\x9AW`\0\x80\xFD[\x815a\x17\xBF\x81a$sV[`\0[\x83\x81\x10\x15a$\xC0W\x81\x81\x01Q\x83\x82\x01R` \x01a$\xA8V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\xE1\x81` \x86\x01` \x86\x01a$\xA5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a%\x08`@\x83\x01\x85a$\xC9V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%+W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%IW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a%`W`\0\x80\xFD[a\x17\xBF\x82a%2V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a%\xBEW`?\x19\x88\x86\x03\x01\x84Ra%\xAC\x85\x83Qa$\xC9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a%\x90V[P\x92\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a%\xDEW`\0\x80\xFD[\x825\x91Pa%\xEE` \x84\x01a%2V[\x90P\x92P\x92\x90PV[` \x81R`\0\x82Q`@` \x84\x01Ra&\x13``\x84\x01\x82a$\xC9V[\x90P` \x84\x01Q\x15\x15`@\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a&=W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a&TW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14]W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a&\x81W`\0\x80\xFD[a&\x8A\x84a%2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA5W`\0\x80\xFD[a&\xB1\x86\x82\x87\x01a&+V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a&\xD4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&\xEBW`\0\x80\xFD[a&\xF7\x88\x83\x89\x01a&+V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a'\x10W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a'$W`\0\x80\xFD[\x815\x81\x81\x11\x15a'3W`\0\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a'HW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a'mW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x83W`\0\x80\xFD[a'\x8F\x87\x82\x88\x01a&+V[\x90\x98\x90\x97P` \x87\x015\x96`@\x015\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a(3W`?\x19\x89\x84\x03\x01\x85R\x81Q```\x01`\x01`@\x1B\x03\x82Q\x16\x85R\x88\x82\x01Q\x81\x8A\x87\x01Ra(\x05\x82\x87\x01\x82a$\xC9V[\x91PP\x87\x82\x01Q\x91P\x84\x81\x03\x88\x86\x01Ra(\x1F\x81\x83a$\xC9V[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a'\xCEV[P\x90\x98\x97PPPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a(VW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a(lW`\0\x80\xFD[a(x\x86\x82\x87\x01a&+V[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a(\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a(\xC0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x05\x05W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a)BWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a)aW\x82\x81U`\x01\x01a)NV[PPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a)\x80Wa)\x80a)\x05V[a)\x94\x83a)\x8E\x83Ta(\x8CV[\x83a)\x1BV[`\0`\x1F\x84\x11`\x01\x81\x14a)\xC8W`\0\x85\x15a)\xB0WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua*\"V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a)\xF9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a)\xD9V[P\x86\x82\x10\x15a*\x16W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12a*UW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a*vW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a*\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14]W`\0\x80\xFD[\x815a*\xB0\x81a$sV[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x91\x82\x16\x17\x82U`\x01\x90\x81\x83\x01` a*\xDF\x86\x82\x01\x87a*_V[\x84\x81\x11\x15a*\xEFWa*\xEFa)\x05V[a+\x03\x81a*\xFD\x86Ta(\x8CV[\x86a)\x1BV[`\0\x94P`\x1F\x81\x11`\x01\x81\x14a+9W`\0\x82\x15a+!WP\x82\x86\x015[`\0\x19`\x03\x84\x90\x1B\x1C\x19\x16`\x01\x83\x90\x1B\x17\x85Ua+\x8EV[`\0\x85\x81R` \x90 `\x1F\x19\x83\x16\x90\x87[\x82\x81\x10\x15a+gW\x85\x89\x015\x82U\x97\x86\x01\x97\x90\x89\x01\x90\x86\x01a+JV[P\x83\x82\x10\x15a+\x84W`\0\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x88\x86\x015\x16\x81U[PP\x86\x82\x88\x1B\x01\x85U[PPPPPPPa+\xA2`@\x83\x01\x83a*_V[a\x1C\x8C\x81\x83`\x02\x86\x01a)iV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a+\xD8Wa+\xD8a+\xB0V[P`\x01\x01\x90V[`\0\x81a+\xEEWa+\xEEa+\xB0V[P`\0\x19\x01\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,.\x81`\x17\x85\x01` \x88\x01a$\xA5V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa,_\x81`(\x84\x01` \x88\x01a$\xA5V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x17\xBF` \x83\x01\x84a$\xC9V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x89\x16\x83R\x87` \x84\x01R\x80\x87\x16`@\x84\x01R\x80\x86\x16``\x84\x01R`\xC0`\x80\x84\x01Ra,\xCD`\xC0\x84\x01\x86a$\xC9V[\x91P\x80\x84\x16`\xA0\x84\x01RP\x97\x96PPPPPPPV[`\0\x82Qa*U\x81\x84` \x87\x01a$\xA5V[\x80\x82\x01\x80\x82\x11\x15a\x03\xBFWa\x03\xBFa+\xB0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xBFWa\x03\xBFa+\xB0V[\x81\x81\x03\x81\x81\x11\x15a\x03\xBFWa\x03\xBFa+\xB0V[`\0\x82a-OWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a-iW`\0\x80\xFD[\x83Q\x92P` \x84\x01Qa-{\x81a$sV[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\x98W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a-\xACW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a-\xBEWa-\xBEa)\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a-\xE6Wa-\xE6a)\x05V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a-\xFFW`\0\x80\xFD[a.\x10\x83` \x83\x01` \x88\x01a$\xA5V[\x80\x95PPPPPP\x92P\x92P\x92V[`\x01\x81\x81[\x80\x85\x11\x15a.ZW\x81`\0\x19\x04\x82\x11\x15a.@Wa.@a+\xB0V[\x80\x85\x16\x15a.MW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a.$V[P\x92P\x92\x90PV[`\0\x82a.qWP`\x01a\x03\xBFV[\x81a.~WP`\0a\x03\xBFV[\x81`\x01\x81\x14a.\x94W`\x02\x81\x14a.\x9EWa.\xBAV[`\x01\x91PPa\x03\xBFV[`\xFF\x84\x11\x15a.\xAFWa.\xAFa+\xB0V[PP`\x01\x82\x1Ba\x03\xBFV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.\xDDWP\x81\x81\na\x03\xBFV[a.\xE7\x83\x83a.\x1FV[\x80`\0\x19\x04\x82\x11\x15a.\xFBWa.\xFBa+\xB0V[\x02\x93\x92PPPV[`\0a\x17\xBF\x83\x83a.bV[` \x80\x82R`\x14\x90\x82\x01Rsslicing out of range``\x1B`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 `\x0Ek\x17>\x10\xEA\xCE\r\x8F\xF9#\xEBa\xF7_\x1B\x0B/#\0\x0E\r\x124\xA0\x12\x90\x85\xED\xEF\x18dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BASINSTORAGE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cW\x08\x92\xC8\x11a\0\xB8W\x80c\x89\xEC\x0B\x93\x11a\0|W\x80c\x89\xEC\x0B\x93\x14a\x03'W\x80c\x91\xD1HT\x14a\x03:W\x80c\x9F)7\x0B\x14a\x03MW\x80c\xA2\x17\xFD\xDF\x14a\x03`W\x80c\xD0oh\x02\x14a\x03hW\x80c\xD5Gt\x1F\x14a\x03{W`\0\x80\xFD[\x80cW\x08\x92\xC8\x14a\x02\x8DW\x80cY\xB6L]\x14a\x02\xA0W\x80co\nC\xC7\x14a\x02\xC0W\x80c\x82+\xA4\x0B\x14a\x02\xD3W\x80c\x87\xA4\x1B\x81\x14a\x02\xFAW`\0\x80\xFD[\x80c//\xF1]\x11a\x01\nW\x80c//\xF1]\x14a\x02\x0CW\x80c6V\x8A\xBE\x14a\x02!W\x80c<~Y\x99\x14a\x024W\x80c?\xF4!\xE9\x14a\x02TW\x80cHMZ:\x14a\x02gW\x80cR\xB6+>\x14a\x02zW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01GW\x80c\x06\xA0\x9D\xEA\x14a\x01oW\x80c\x12\x1Eb\x0E\x14a\x01\x9AW\x80c$\x8A\x9C\xA3\x14a\x01\xBBW\x80c&)Jw\x14a\x01\xECW[`\0\x80\xFD[a\x01Za\x01U6`\x04a$IV[a\x03\x8EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x82a\x01}6`\x04a$\x88V[a\x03\xC5V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01fV[a\x01\xADa\x01\xA86`\x04a$\x88V[a\x03\xD0V[`@Qa\x01f\x92\x91\x90a$\xF5V[a\x01\xDEa\x01\xC96`\x04a%\x19V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01fV[a\x01\xFFa\x01\xFA6`\x04a%NV[a\x03\xF1V[`@Qa\x01f\x91\x90a%iV[a\x02\x1Fa\x02\x1A6`\x04a%\xCBV[a\x04\xE0V[\0[a\x02\x1Fa\x02/6`\x04a%\xCBV[a\x05\nV[a\x02Ga\x02B6`\x04a$\x88V[a\x05\x8DV[`@Qa\x01f\x91\x90a%\xF7V[a\x01Za\x02b6`\x04a$\x88V[a\x05\xABV[a\x02Ga\x02u6`\x04a$\x88V[a\x05\xB6V[a\x02\x1Fa\x02\x886`\x04a&lV[a\x05\xD4V[a\x02\x1Fa\x02\x9B6`\x04a&\xBEV[a\x07\rV[a\x02\xB3a\x02\xAE6`\x04a'WV[a\t'V[`@Qa\x01f\x91\x90a'\xA7V[a\x02\xB3a\x02\xCE6`\x04a(AV[a\tuV[a\x01\xDE\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x81V[a\x03\ra\x03\x086`\x04a$\x88V[a\t\xC2V[`@\x80Q`\x07\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x01fV[a\x02Ga\x0356`\x04a$\x88V[a\t\xD0V[a\x01Za\x03H6`\x04a%\xCBV[a\t\xEEV[a\x03\ra\x03[6`\x04a$\x88V[a\n\x17V[a\x01\xDE`\0\x81V[a\x01\x82a\x03v6`\x04a$\x88V[a\n%V[a\x02\x1Fa\x03\x896`\x04a%\xCBV[a\n0V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\xBFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0a\x03\xBF\x82a\nUV[```\0\x80a\x03\xDE\x84a\n\x93V[\x80Q` \x90\x91\x01Q\x90\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x04\xD5W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04H\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04t\x90a(\x8CV[\x80\x15a\x04\xC1W\x80`\x1F\x10a\x04\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04)V[PPPP\x90P\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x04\xFB\x81a\n\xDDV[a\x05\x05\x83\x83a\n\xEAV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x05\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\x89\x82\x82a\x0BnV[PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0B\xD3V[`\0a\x03\xBF\x82a\x0C\x1DV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0CSV[\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95a\x05\xFE\x81a\n\xDDV[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x01\x84\x84`@Qa\x06\x1B\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06VW\x82\x82`@Qc\\x\xF6\xED`\xE1\x1B\x81R`\x04\x01a\x05v\x92\x91\x90a(\xD6V[\x83`\x01\x84\x84`@Qa\x06i\x92\x91\x90a(\xC6V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x86\x16`\0\x90\x81R`\x02\x82R\x91\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01a\x06\xBC\x83\x85\x83a)iV[P\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x06\xD7\x92\x91\x90a(\xC6V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF8\xDE\xBC/\x17E\xEB\xA8i\t\x89\x0F-\xC0abG\x05\xC7C)4\x88)\xE0J\xBAC\xC0\x15\xB9\xA2\x90`\0\x90\xA3PPPPV[\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95a\x077\x81a\n\xDDV[`\0C\x90P`\0`\x01\x87\x87`@Qa\x07P\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80a\x07\x8DW\x86\x86`@Qc\x15\xE6\xE0\xEB`\xE2\x1B\x81R`\x04\x01a\x05v\x92\x91\x90a(\xD6V[`\0[\x84\x81\x10\x15a\t\x1DW`\x04\x88\x88`@Qa\x07\xAA\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x86\x86\x83\x81\x81\x10a\x07\xDBWa\x07\xDBa*)V[\x90P` \x02\x81\x01\x90a\x07\xED\x91\x90a*?V[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x90\x92 \x90\x91`\x03\x02\x01a\x08\x0F\x82\x82a*\xA5V[PP`\x03\x88\x88`@Qa\x08#\x92\x91\x90a(\xC6V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x90`\0a\x08@\x83a+\xC6V[\x91\x90PUP\x81`\x01`\x01`\xA0\x1B\x03\x16\x88\x88`@Qa\x08_\x92\x91\x90a(\xC6V[`@Q\x80\x91\x03\x90 \x87\x87\x84\x81\x81\x10a\x08yWa\x08ya*)V[\x90P` \x02\x81\x01\x90a\x08\x8B\x91\x90a*?V[a\x08\x99\x90` \x81\x01\x90a$\x88V[`\x01`\x01`@\x1B\x03\x16\x7Fy\xB3\x877\xA5?\x0B\xC2.4\x83sU\xA5\xC9\xD1\xBF\xCD\xDF\x81|\xB0\x02/\x82\xFE\x93\x8B\x05\x88\xBF?\x89\x89\x86\x81\x81\x10a\x08\xD5Wa\x08\xD5a*)V[\x90P` \x02\x81\x01\x90a\x08\xE7\x91\x90a*?V[a\x08\xF5\x90`@\x81\x01\x90a*_V[`@Qa\t\x03\x92\x91\x90a(\xD6V[`@Q\x80\x91\x03\x90\xA4\x80a\t\x15\x81a+\xC6V[\x91PPa\x07\x90V[PPPPPPPPV[```\0`\x03\x86\x86`@Qa\t=\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x80\x83\x11a\t[W\x82a\t]V[\x80[\x92Pa\tk\x86\x86\x86\x86a\x0C\x92V[\x96\x95PPPPPPV[```\0`\x03\x85\x85`@Qa\t\x8B\x92\x91\x90a(\xC6V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x80\x83\x11a\t\xA9W\x82a\t\xABV[\x80[\x92Pa\t\xB9\x85\x85C\x86a\x0C\x92V[\x95\x94PPPPPV[`\0\x80`\0a\x03\xDE\x84a\x0FlV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x03\xBF\x82a\x0F\xB5V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80`\0a\x03\xDE\x84a\x0F\xF4V[`\0a\x03\xBF\x82a\x102V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\nK\x81a\n\xDDV[a\x05\x05\x83\x83a\x0BnV[`\0\x80a\nj\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\x80`\x05c\x07\xA1\xF0Q`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x10\xBAV[\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\n\xBC\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\xD2`\x05c\x02\xC3s\x86`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x10\xD1V[a\n\xE7\x813a\x11\x08V[PV[a\n\xF4\x82\x82a\t\xEEV[a\x05\x89W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0B*3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x0Bx\x82\x82a\t\xEEV[\x15a\x05\x89W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0B\xFC\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\xB2\x05\x9CI`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11aV[`\0\x80a\x0C2\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0CH`\x05c\x9C\x9A\xC8\x19`Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11\xC5V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0C|\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\xFF\x88\xE8<`Q\x85a\x10\x95V[```\0\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xAFWa\x0C\xAFa)\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\rW\x81` \x01[a\x0C\xFA`@Q\x80``\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xCDW\x90P[P\x90P\x84[\x84\x83\x10\x15a\x0F_W`\0`\x04\x89\x89`@Qa\r.\x92\x91\x90a(\xC6V[\x90\x81R`@\x80Q\x91\x82\x90\x03` \x90\x81\x01\x83 `\0\x86\x81R\x90\x82R\x82\x81 \x80T\x80\x84\x02\x86\x01\x84\x01\x90\x94R\x83\x85R\x92\x91\x84\x01[\x82\x82\x10\x15a\x0E\xCBW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x85\x02\x90\x91\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\r\xA8\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xD4\x90a(\x8CV[\x80\x15a\x0E!W\x80`\x1F\x10a\r\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0E:\x90a(\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Ef\x90a(\x8CV[\x80\x15a\x0E\xB3W\x80`\x1F\x10a\x0E\x88Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xB3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x96W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r_V[PPPP\x90P`\0[\x81Q\x81\x10\x15a\x0F=W\x86\x85\x10\x15a\x0F=W\x81\x81\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a*)V[` \x02` \x01\x01Q\x84\x86\x81Q\x81\x10a\x0F\x11Wa\x0F\x11a*)V[` \x02` \x01\x01\x81\x90RP\x84\x80a\x0F'\x90a+\xC6V[\x95PP\x80\x80a\x0F5\x90a+\xC6V[\x91PPa\x0E\xD4V[P\x81`\0\x03a\x0FLWPa\x0F_V[\x81a\x0FV\x81a+\xDFV[\x92PPPa\r\x12V[P\x90\x81R\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x0F\x94\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0F\xAA`\x05c\t\xC3\x0B `Q\x85a\x10\x95V[\x90Pa\n\x8B\x81a\x11\xD3V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0a\x0F\xDE\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0C\x12`\x05c\x0B\xF4lW`Q\x85a\x10\x95V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x10\x1C\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\x0F\xAA`\x05c\x99\x04\xF2\xFF`Q\x85a\x10\x95V[`\0\x80a\x10G\x83`\x01`\x01`@\x1B\x03\x16a\x10]V[\x90P`\0a\n\x80`\x05c7\xBC6\xDA`Q\x85a\x10\x95V[```\0a\x10s\x83`\x01`\x01`@\x1B\x03\x16a\x127V[\x90P`\0a\x10\x80\x82a\x12\x85V[\x90Pa\x10\x8C\x81\x85a\x12\xA6V[a\n\x8B\x81a\x12\xB2V[``a\x12\xFD\x80a\x10\xAF\x87\x87\x87\x87`\0`\x01c\xFF\xFF\xFF\xFF\x88\x16V[\x97\x96PPPPPPPV[`\0\x80\x80a\x10\xC8\x84\x82a\x13\xD7V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01\x81\x90R\x90a\x10\xC8\x84\x83a\x14dV[a\x11\x12\x82\x82a\t\xEEV[a\x05\x89Wa\x11\x1F\x81a\x16\x12V[a\x11*\x83` a\x16$V[`@Q` \x01a\x11;\x92\x91\x90a+\xF6V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05v\x91`\x04\x01a,kV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R`\0` \x83\x01\x81\x90R\x83Q\x15a\x11\x9FWa\x11\x8A\x84\x82a\x17\xC6V[\x81Q\x91\x93P\x91P\x15a\x11\x9FWa\n\x8B\x82a\x19kV[PP`@\x80Q`\0\x81\x83\x01\x81\x81R``\x83\x01\x90\x93R\x91\x81R` \x81\x01\x91\x90\x91R\x92\x91PPV[`\0\x80\x80a\x10\xC8\x84\x82a\x1A\xC7V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80a\x11\xF4\x84\x82a\x1BbV[\x92P\x90P`\x02\x81\x14a\x12\x08Wa\x12\x08a,~V[a\x12\x12\x84\x83a\x1B\xD5V[`\x07\x91\x90\x91\x0B\x84R\x91Pa\x12&\x84\x83a\x1B\xD5V[P`\x07\x0B` \x84\x01RP\x90\x92\x91PPV[`\0`\x17\x82\x11a\x12IWP`\x01\x91\x90PV[`\xFF\x82\x11a\x12YWP`\x02\x91\x90PV[a\xFF\xFF\x82\x11a\x12jWP`\x03\x91\x90PV[c\xFF\xFF\xFF\xFF\x82\x11a\x12}WP`\x05\x91\x90PV[P`\t\x91\x90PV[a\x12\x8Da$\x14V[\x80Qa\x12\x99\x90\x83a\x1B\xEFV[P`\0` \x82\x01R\x91\x90PV[a\x05\x89\x82`\0\x83a\x1CfV[``\x81` \x01Q`\0\x14a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk$\xB7;0\xB64\xB2\x10!\xA1'\xA9`\xA1\x1B`D\x82\x01R`d\x01a\x05vV[PQQ\x90V[``a\x13\x10`\x05`\x7F`\x99\x1B\x01\x84a\x1D\x85V[`\0\x80`\x05`\x7F`\x99\x1B\x01\x88\x86\x86a\x13)W`\0a\x13,V[`\x01[\x8A\x8A\x8E`@Q` \x01a\x13D\x96\x95\x94\x93\x92\x91\x90a,\x94V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13^\x91a,\xE3V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x13\x99W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13\x9EV[``\x91P[P\x91P\x91P\x81a\x13\xC1W`@Qc\x8A}\xB5\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xCA\x81a\x1D\xD3V[\x99\x98PPPPPPPPPV[`\0\x80`\0\x80a\x13\xE7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16\x15a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Finvalid maj (expected MajUnsigne`D\x82\x01RddInt)`\xD8\x1B`d\x82\x01R`\x84\x01a\x05vV[\x92P\x83\x91PP[\x92P\x92\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\0\x80`\0a\x14\x88\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x02\x14\x80a\x14\xADWP`\xFF\x82\x16`\x03\x14[a\x15\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7Finvalid maj (expected MajByteStr`D\x82\x01Rting or MajTextString)`X\x1B`d\x82\x01R`\x84\x01a\x05vV[`\0a\x15#\x82\x87a,\xF5V[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15?Wa\x15?a)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15iW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x87[\x83\x81\x10\x15a\x15\xDEW\x89\x81\x81Q\x81\x10a\x15\x8AWa\x15\x8Aa*)V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x15\xA7Wa\x15\xA7a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81a\x15\xC8\x81a+\xC6V[\x92PP\x80\x80a\x15\xD6\x90a+\xC6V[\x91PPa\x15pV[P`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R`\xFF\x86\x16`\x03\x14` \x82\x01Ra\x16\x02\x85\x8Aa,\xF5V[\x96P\x96PPPPPP\x92P\x92\x90PV[``a\x03\xBF`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x163\x83`\x02a-\x08V[a\x16>\x90`\x02a,\xF5V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16UWa\x16Ua)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16\x7FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x16\x9AWa\x16\x9Aa*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x16\xC9Wa\x16\xC9a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x16\xED\x84`\x02a-\x08V[a\x16\xF8\x90`\x01a,\xF5V[\x90P[`\x01\x81\x11\x15a\x17pWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x17,Wa\x17,a*)V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x17BWa\x17Ba*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x17i\x81a+\xDFV[\x90Pa\x16\xFBV[P\x83\x15a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05vV[\x93\x92PPPV[```\0\x80`\0a\x17\xD7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x06\x14\x80a\x17\xFCWP`\xFF\x82\x16`\x02\x14[a\x18_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7Finvalid maj (expected MajTag or `D\x82\x01RmMajByteString)`\x90\x1B`d\x82\x01R`\x84\x01a\x05vV[`\x05\x19`\xFF\x83\x16\x01a\x18\x98Wa\x18u\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x02\x14a\x18\x98Wa\x18\x98a,~V[`\0a\x18\xA4\x82\x87a,\xF5V[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xC0Wa\x18\xC0a)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x87[\x83\x81\x10\x15a\x19_W\x89\x81\x81Q\x81\x10a\x19\x0BWa\x19\x0Ba*)V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x19(Wa\x19(a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81a\x19I\x81a+\xC6V[\x92PP\x80\x80a\x19W\x90a+\xC6V[\x91PPa\x18\xF1V[P\x81a\x16\x02\x85\x8Aa,\xF5V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81Q`\0\x03a\x19\xB4WPP`@\x80Q`\x80\x81\x01\x82R`\x01\x91\x81\x01\x91\x82R`\0``\x82\x01\x81\x90R\x91\x81R` \x81\x01\x91\x90\x91R\x90V[`\0`\x01\x83Qa\x19\xC4\x91\x90a-\x1FV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDBWa\x19\xDBa)\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1A\x05W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x83`\0\x81Q\x81\x10a\x1A\x1DWa\x1A\x1Da*)V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\x01`\xF8\x1B\x03a\x1A:WP`\x01[`\x01[\x84Q\x81\x10\x15a\x1A\xACW\x84\x81\x81Q\x81\x10a\x1AXWa\x1AXa*)V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x1As`\x01\x84a-\x1FV[\x81Q\x81\x10a\x1A\x83Wa\x1A\x83a*)V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\xA4\x81a+\xC6V[\x91PPa\x1A=V[P`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R\x15\x15` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x1A\xD7\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x07\x14a\x1B:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid maj (expected MajOther)\0`D\x82\x01R`d\x01a\x05vV[`\x15\x81\x14\x80a\x1BIWP`\x14\x81\x14[a\x1BUWa\x1BUa,~V[`\x14\x14\x15\x95\x93\x94PPPPV[`\0\x80`\0\x80a\x1Br\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x04\x14a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid maj (expected MajArray)\0`D\x82\x01R`d\x01a\x05vV[`\0\x80\x80a\x1B\xE3\x85\x85a \x90V[\x90\x96\x90\x95P\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x1C\x0F` \x83a-2V[\x15a\x1C7Wa\x1C\x1F` \x83a-2V[a\x1C*\x90` a-\x1FV[a\x1C4\x90\x83a,\xF5V[\x91P[` \x80\x84\x01\x83\x90R`@Q\x80\x85R`\0\x81R\x90\x81\x84\x01\x01\x81\x81\x10\x15a\x1C[W`\0\x80\xFD[`@RP\x91\x92\x91PPV[`\x17\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1C\x92W\x82Qa\x1C\x8C\x90`\xE0`\x05\x85\x90\x1B\x16\x83\x17a!-V[PPPPV[`\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1C\xD2W\x82Qa\x1C\xBA\x90`\x18a\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x01a!\x96V[a\xFF\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1D\x13W\x82Qa\x1C\xFB\x90`\x19a\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x02a!\x96V[c\xFF\xFF\xFF\xFF\x81`\x01`\x01`@\x1B\x03\x16\x11a\x1DVW\x82Qa\x1D>\x90`\x1Aa\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x04a!\x96V[\x82Qa\x1Dm\x90`\x1Ba\x1F\xE0`\x05\x86\x90\x1B\x16\x17a!-V[P\x82Qa\x1C\x8C\x90`\x01`\x01`@\x1B\x03\x83\x16`\x08a!\x96V[G\x81\x81\x10\x15a\x1D\xB1W`@QcG\x87\xA1\x03`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x05vV[\x82?\x15\x15\x80a\x1C\x8CW`@Qc\x06M\x95K`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0\x80`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x1D\xEE\x91\x90a-TV[\x91\x94P\x92P\x90P`\x01`\x01`@\x1B\x03\x82\x16a\x1E(W\x80Q\x15a\x1E#W`@Qc\x0Et\x99\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x94V[`\x01`\x01`@\x1B\x03\x82\x16`Q\x14\x80a\x1EIWP`\x01`\x01`@\x1B\x03\x82\x16`q\x14[\x15a\x1EpW\x80Q`\0\x03a\x1E#W`@Qc\x0Et\x99\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xF1\xF6\xBC\xED`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x05vV[\x82\x15a\n\x8BW`@Qc\xD4\xBBfq`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x05vV[`\0\x80`\0\x80a\x1E\xC6\x86\x86a\"\x1BV[\x90Pa\x1E\xD3`\x01\x86a,\xF5V[\x94P`\x07`\x05\x82\x90\x1C\x16`\x1F\x82\x16`\x1C\x81\x10a\x1F?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fcannot handle headers with extra`D\x82\x01Rd > 27`\xD8\x1B`d\x82\x01R`\x84\x01a\x05vV[`\x18\x81`\xFF\x16\x10\x15a\x1F]W\x90\x94P`\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x18\x03a\x1F\xD9W`\0a\x1Ft\x89\x89a\"\x1BV[\x90Pa\x1F\x81`\x01\x89a,\xF5V[\x97P`\x18\x81`\xFF\x16\x10\x15a\x1F\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk4\xB7;0\xB64\xB2\x101\xB17\xB9`\xA1\x1B`D\x82\x01R`d\x01a\x05vV[\x91\x95PP`\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x19\x03a \x13W`\0a\x1F\xF0\x89\x89a\"jV[\x90Pa\x1F\xFD`\x02\x89a,\xF5V[\x97P\x91\x95PPa\xFF\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x1A\x03a OW`\0a *\x89\x89a\"\xA3V[\x90Pa 7`\x04\x89a,\xF5V[\x97P\x91\x95PPc\xFF\xFF\xFF\xFF\x16\x92P\x84\x91Pa \x89\x90PV[\x80`\xFF\x16`\x1B\x14a bWa ba,~V[`\0a n\x89\x89a\"\xDCV[\x90Pa {`\x08\x89a,\xF5V[\x97P\x91\x95P\x90\x93P\x85\x92PPP[\x92P\x92P\x92V[`\0\x80`\0\x80a \xA0\x86\x86a\x1E\xB6V[\x96P\x90\x92P`\x01`\x01`@\x1B\x03\x16\x90P`\xFF\x82\x16`\x01\x14\x80a \xC3WP`\xFF\x82\x16\x15[a\x14VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7Finvalid maj (expected MajSignedI`D\x82\x01Rtnt or MajUnsignedInt)`X\x1B`d\x82\x01R`\x84\x01a\x05vV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x82QQ`\0a!R\x82`\x01a,\xF5V[\x90P\x84` \x01Q\x82\x10a!sWa!s\x85a!n\x83`\x02a-\x08V[a#\x15V[\x84Q` \x83\x82\x01\x01\x85\x81SP\x80Q\x82\x11\x15a!\x8CW\x81\x81R[P\x93\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x83QQ`\0a!\xBA\x82\x85a,\xF5V[\x90P\x85` \x01Q\x81\x11\x15a!\xD7Wa!\xD7\x86a!n\x83`\x02a-\x08V[`\0`\x01a!\xE7\x86a\x01\0a/\x03V[a!\xF1\x91\x90a-\x1FV[\x90P\x86Q\x82\x81\x01\x87\x83\x19\x82Q\x16\x17\x81RP\x80Q\x83\x11\x15a\"\x0FW\x82\x81R[P\x95\x96\x95PPPPPPV[`\0a\"(\x82`\x01a,\xF5V[\x83Q\x10\x15a\"HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[\x82\x82\x81Q\x81\x10a\"ZWa\"Za*)V[\x01` \x01Q`\xF8\x1C\x90P\x92\x91PPV[`\0a\"w\x82`\x02a,\xF5V[\x83Q\x10\x15a\"\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xF0\x1C\x90V[`\0a\"\xB0\x82`\x04a,\xF5V[\x83Q\x10\x15a\"\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xE0\x1C\x90V[`\0a\"\xE9\x82`\x08a,\xF5V[\x83Q\x10\x15a#\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05v\x90a/\x0FV[P\x01` \x01Q`\xC0\x1C\x90V[\x81Qa#!\x83\x83a\x1B\xEFV[Pa\x1C\x8C\x83\x82`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x17\xBF\x83\x83\x84Q`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x82Q\x82\x11\x15a#fW`\0\x80\xFD[\x83QQ`\0a#u\x84\x83a,\xF5V[\x90P\x85` \x01Q\x81\x11\x15a#\x92Wa#\x92\x86a!n\x83`\x02a-\x08V[\x85Q\x80Q\x83\x82\x01` \x01\x91`\0\x91\x80\x85\x11\x15a#\xACW\x84\x82R[PPP` \x86\x01[` \x86\x10a#\xECW\x80Q\x82Ra#\xCB` \x83a,\xF5V[\x91Pa#\xD8` \x82a,\xF5V[\x90Pa#\xE5` \x87a-\x1FV[\x95Pa#\xB4V[Q\x81Q`\0\x19` \x88\x90\x03a\x01\0\n\x01\x90\x81\x16\x90\x19\x91\x90\x91\x16\x17\x90RP\x84\x91PP\x93\x92PPPV[`@Q\x80`@\x01`@R\x80a$<`@Q\x80`@\x01`@R\x80``\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a$[W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x17\xBFW`\0\x80\xFD[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\n\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\x9AW`\0\x80\xFD[\x815a\x17\xBF\x81a$sV[`\0[\x83\x81\x10\x15a$\xC0W\x81\x81\x01Q\x83\x82\x01R` \x01a$\xA8V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\xE1\x81` \x86\x01` \x86\x01a$\xA5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a%\x08`@\x83\x01\x85a$\xC9V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%+W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%IW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a%`W`\0\x80\xFD[a\x17\xBF\x82a%2V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a%\xBEW`?\x19\x88\x86\x03\x01\x84Ra%\xAC\x85\x83Qa$\xC9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a%\x90V[P\x92\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a%\xDEW`\0\x80\xFD[\x825\x91Pa%\xEE` \x84\x01a%2V[\x90P\x92P\x92\x90PV[` \x81R`\0\x82Q`@` \x84\x01Ra&\x13``\x84\x01\x82a$\xC9V[\x90P` \x84\x01Q\x15\x15`@\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a&=W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a&TW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14]W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a&\x81W`\0\x80\xFD[a&\x8A\x84a%2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA5W`\0\x80\xFD[a&\xB1\x86\x82\x87\x01a&+V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a&\xD4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&\xEBW`\0\x80\xFD[a&\xF7\x88\x83\x89\x01a&+V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a'\x10W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a'$W`\0\x80\xFD[\x815\x81\x81\x11\x15a'3W`\0\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a'HW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a'mW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x83W`\0\x80\xFD[a'\x8F\x87\x82\x88\x01a&+V[\x90\x98\x90\x97P` \x87\x015\x96`@\x015\x95P\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a(3W`?\x19\x89\x84\x03\x01\x85R\x81Q```\x01`\x01`@\x1B\x03\x82Q\x16\x85R\x88\x82\x01Q\x81\x8A\x87\x01Ra(\x05\x82\x87\x01\x82a$\xC9V[\x91PP\x87\x82\x01Q\x91P\x84\x81\x03\x88\x86\x01Ra(\x1F\x81\x83a$\xC9V[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a'\xCEV[P\x90\x98\x97PPPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a(VW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a(lW`\0\x80\xFD[a(x\x86\x82\x87\x01a&+V[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a(\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a(\xC0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x05\x05W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a)BWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a)aW\x82\x81U`\x01\x01a)NV[PPPPPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15a)\x80Wa)\x80a)\x05V[a)\x94\x83a)\x8E\x83Ta(\x8CV[\x83a)\x1BV[`\0`\x1F\x84\x11`\x01\x81\x14a)\xC8W`\0\x85\x15a)\xB0WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua*\"V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a)\xF9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a)\xD9V[P\x86\x82\x10\x15a*\x16W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12a*UW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a*vW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a*\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14]W`\0\x80\xFD[\x815a*\xB0\x81a$sV[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x91\x82\x16\x17\x82U`\x01\x90\x81\x83\x01` a*\xDF\x86\x82\x01\x87a*_V[\x84\x81\x11\x15a*\xEFWa*\xEFa)\x05V[a+\x03\x81a*\xFD\x86Ta(\x8CV[\x86a)\x1BV[`\0\x94P`\x1F\x81\x11`\x01\x81\x14a+9W`\0\x82\x15a+!WP\x82\x86\x015[`\0\x19`\x03\x84\x90\x1B\x1C\x19\x16`\x01\x83\x90\x1B\x17\x85Ua+\x8EV[`\0\x85\x81R` \x90 `\x1F\x19\x83\x16\x90\x87[\x82\x81\x10\x15a+gW\x85\x89\x015\x82U\x97\x86\x01\x97\x90\x89\x01\x90\x86\x01a+JV[P\x83\x82\x10\x15a+\x84W`\0\x19`\xF8\x85`\x03\x1B\x16\x1C\x19\x88\x86\x015\x16\x81U[PP\x86\x82\x88\x1B\x01\x85U[PPPPPPPa+\xA2`@\x83\x01\x83a*_V[a\x1C\x8C\x81\x83`\x02\x86\x01a)iV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a+\xD8Wa+\xD8a+\xB0V[P`\x01\x01\x90V[`\0\x81a+\xEEWa+\xEEa+\xB0V[P`\0\x19\x01\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,.\x81`\x17\x85\x01` \x88\x01a$\xA5V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa,_\x81`(\x84\x01` \x88\x01a$\xA5V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x17\xBF` \x83\x01\x84a$\xC9V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x89\x16\x83R\x87` \x84\x01R\x80\x87\x16`@\x84\x01R\x80\x86\x16``\x84\x01R`\xC0`\x80\x84\x01Ra,\xCD`\xC0\x84\x01\x86a$\xC9V[\x91P\x80\x84\x16`\xA0\x84\x01RP\x97\x96PPPPPPPV[`\0\x82Qa*U\x81\x84` \x87\x01a$\xA5V[\x80\x82\x01\x80\x82\x11\x15a\x03\xBFWa\x03\xBFa+\xB0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xBFWa\x03\xBFa+\xB0V[\x81\x81\x03\x81\x81\x11\x15a\x03\xBFWa\x03\xBFa+\xB0V[`\0\x82a-OWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a-iW`\0\x80\xFD[\x83Q\x92P` \x84\x01Qa-{\x81a$sV[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\x98W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a-\xACW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a-\xBEWa-\xBEa)\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a-\xE6Wa-\xE6a)\x05V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a-\xFFW`\0\x80\xFD[a.\x10\x83` \x83\x01` \x88\x01a$\xA5V[\x80\x95PPPPPP\x92P\x92P\x92V[`\x01\x81\x81[\x80\x85\x11\x15a.ZW\x81`\0\x19\x04\x82\x11\x15a.@Wa.@a+\xB0V[\x80\x85\x16\x15a.MW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a.$V[P\x92P\x92\x90PV[`\0\x82a.qWP`\x01a\x03\xBFV[\x81a.~WP`\0a\x03\xBFV[\x81`\x01\x81\x14a.\x94W`\x02\x81\x14a.\x9EWa.\xBAV[`\x01\x91PPa\x03\xBFV[`\xFF\x84\x11\x15a.\xAFWa.\xAFa+\xB0V[PP`\x01\x82\x1Ba\x03\xBFV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.\xDDWP\x81\x81\na\x03\xBFV[a.\xE7\x83\x83a.\x1FV[\x80`\0\x19\x04\x82\x11\x15a.\xFBWa.\xFBa+\xB0V[\x02\x93\x92PPPV[`\0a\x17\xBF\x83\x83a.bV[` \x80\x82R`\x14\x90\x82\x01Rsslicing out of range``\x1B`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 `\x0Ek\x17>\x10\xEA\xCE\r\x8F\xF9#\xEBa\xF7_\x1B\x0B/#\0\x0E\r\x124\xA0\x12\x90\x85\xED\xEF\x18dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BASINSTORAGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BasinStorage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BasinStorage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BasinStorage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BasinStorage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BasinStorage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BasinStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BasinStorage<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BASINSTORAGE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BASINSTORAGE_ABI.clone(),
                BASINSTORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PUB_ADMIN_ROLE` (0x822ba40b) function
        pub fn pub_admin_role(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([130, 43, 164, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addDeals` (0x570892c8) function
        pub fn add_deals(
            &self,
            pub_: ::std::string::String,
            deals: ::std::vec::Vec<DealInfo>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 8, 146, 200], (pub_, deals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPub` (0x52b62b3e) function
        pub fn create_pub(
            &self,
            owner: ::ethers::core::types::Address,
            pub_: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 182, 43, 62], (owner, pub_))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealActivation` (0x9f29370b) function
        pub fn deal_activation(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (i64, i64)> {
            self.0
                .method_hash([159, 41, 55, 11], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealClient` (0x06a09dea) function
        pub fn deal_client(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([6, 160, 157, 234], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealClientCollateral` (0x89ec0b93) function
        pub fn deal_client_collateral(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, BigInt> {
            self.0
                .method_hash([137, 236, 11, 147], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealLabel` (0x121e620e) function
        pub fn deal_label(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::Bytes, bool)>
        {
            self.0
                .method_hash([18, 30, 98, 14], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealProvider` (0xd06f6802) function
        pub fn deal_provider(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([208, 111, 104, 2], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealProviderCollateral` (0x3c7e5999) function
        pub fn deal_provider_collateral(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, BigInt> {
            self.0
                .method_hash([60, 126, 89, 153], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealTerm` (0x87a41b81) function
        pub fn deal_term(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (i64, i64)> {
            self.0
                .method_hash([135, 164, 27, 129], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealTotalPrice` (0x484d5a3a) function
        pub fn deal_total_price(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, BigInt> {
            self.0
                .method_hash([72, 77, 90, 58], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealVerified` (0x3ff421e9) function
        pub fn deal_verified(
            &self,
            deal_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([63, 244, 33, 233], deal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestNDeals` (0x6f0a43c7) function
        pub fn latest_n_deals(
            &self,
            pub_: ::std::string::String,
            n: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DealInfo>> {
            self.0
                .method_hash([111, 10, 67, 199], (pub_, n))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paginatedDeals` (0x59b64c5d) function
        pub fn paginated_deals(
            &self,
            pub_: ::std::string::String,
            offset: ::ethers::core::types::U256,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DealInfo>> {
            self.0
                .method_hash([89, 182, 76, 93], (pub_, offset, limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pubsOfOwner` (0x26294a77) function
        pub fn pubs_of_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::string::String>>
        {
            self.0
                .method_hash([38, 41, 74, 119], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DealAdded` event
        pub fn deal_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DealAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PubCreated` event
        pub fn pub_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PubCreatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleAdminChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleGrantedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoleRevokedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BasinStorageEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for BasinStorage<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActorError` with signature `ActorError(int256)` and selector `0xd4bb6671`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ActorError", abi = "ActorError(int256)")]
    pub struct ActorError {
        pub error_code: ::ethers::core::types::I256,
    }
    ///Custom Error type `ActorNotFound` with signature `ActorNotFound()` and selector `0x64d954b0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ActorNotFound", abi = "ActorNotFound()")]
    pub struct ActorNotFound;
    ///Custom Error type `DealEpochAlreadyExists` with signature `DealEpochAlreadyExists(uint256)` and selector `0x8af75d66`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "DealEpochAlreadyExists",
        abi = "DealEpochAlreadyExists(uint256)"
    )]
    pub struct DealEpochAlreadyExists {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Custom Error type `FailToCallActor` with signature `FailToCallActor()` and selector `0x8a7db5bf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FailToCallActor", abi = "FailToCallActor()")]
    pub struct FailToCallActor;
    ///Custom Error type `InvalidCodec` with signature `InvalidCodec(uint64)` and selector `0xf1f6bced`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidCodec", abi = "InvalidCodec(uint64)")]
    pub struct InvalidCodec(pub u64);
    ///Custom Error type `InvalidResponseLength` with signature `InvalidResponseLength()` and selector `0x0e749907`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidResponseLength", abi = "InvalidResponseLength()")]
    pub struct InvalidResponseLength;
    ///Custom Error type `NotEnoughBalance` with signature `NotEnoughBalance(uint256,uint256)` and selector `0x8f0f4206`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotEnoughBalance", abi = "NotEnoughBalance(uint256,uint256)")]
    pub struct NotEnoughBalance {
        pub balance: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    ///Custom Error type `PubAlreadyExists` with signature `PubAlreadyExists(string)` and selector `0xb8f1edda`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PubAlreadyExists", abi = "PubAlreadyExists(string)")]
    pub struct PubAlreadyExists {
        pub reason: ::std::string::String,
    }
    ///Custom Error type `PubDoesNotExist` with signature `PubDoesNotExist(string)` and selector `0x579b83ac`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PubDoesNotExist", abi = "PubDoesNotExist(string)")]
    pub struct PubDoesNotExist {
        pub reason: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BasinStorageErrors {
        ActorError(ActorError),
        ActorNotFound(ActorNotFound),
        DealEpochAlreadyExists(DealEpochAlreadyExists),
        FailToCallActor(FailToCallActor),
        InvalidCodec(InvalidCodec),
        InvalidResponseLength(InvalidResponseLength),
        NotEnoughBalance(NotEnoughBalance),
        PubAlreadyExists(PubAlreadyExists),
        PubDoesNotExist(PubDoesNotExist),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BasinStorageErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ActorError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActorError(decoded));
            }
            if let Ok(decoded) = <ActorNotFound as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActorNotFound(decoded));
            }
            if let Ok(decoded) =
                <DealEpochAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealEpochAlreadyExists(decoded));
            }
            if let Ok(decoded) = <FailToCallActor as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailToCallActor(decoded));
            }
            if let Ok(decoded) = <InvalidCodec as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidCodec(decoded));
            }
            if let Ok(decoded) =
                <InvalidResponseLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidResponseLength(decoded));
            }
            if let Ok(decoded) = <NotEnoughBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughBalance(decoded));
            }
            if let Ok(decoded) = <PubAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubAlreadyExists(decoded));
            }
            if let Ok(decoded) = <PubDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubDoesNotExist(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasinStorageErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActorError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ActorNotFound(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealEpochAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailToCallActor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidCodec(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidResponseLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubAlreadyExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubDoesNotExist(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BasinStorageErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <ActorError as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ActorNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DealEpochAlreadyExists as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FailToCallActor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidCodec as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidResponseLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotEnoughBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <PubAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <PubDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BasinStorageErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActorError(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActorNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealEpochAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailToCallActor(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCodec(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidResponseLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BasinStorageErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActorError> for BasinStorageErrors {
        fn from(value: ActorError) -> Self {
            Self::ActorError(value)
        }
    }
    impl ::core::convert::From<ActorNotFound> for BasinStorageErrors {
        fn from(value: ActorNotFound) -> Self {
            Self::ActorNotFound(value)
        }
    }
    impl ::core::convert::From<DealEpochAlreadyExists> for BasinStorageErrors {
        fn from(value: DealEpochAlreadyExists) -> Self {
            Self::DealEpochAlreadyExists(value)
        }
    }
    impl ::core::convert::From<FailToCallActor> for BasinStorageErrors {
        fn from(value: FailToCallActor) -> Self {
            Self::FailToCallActor(value)
        }
    }
    impl ::core::convert::From<InvalidCodec> for BasinStorageErrors {
        fn from(value: InvalidCodec) -> Self {
            Self::InvalidCodec(value)
        }
    }
    impl ::core::convert::From<InvalidResponseLength> for BasinStorageErrors {
        fn from(value: InvalidResponseLength) -> Self {
            Self::InvalidResponseLength(value)
        }
    }
    impl ::core::convert::From<NotEnoughBalance> for BasinStorageErrors {
        fn from(value: NotEnoughBalance) -> Self {
            Self::NotEnoughBalance(value)
        }
    }
    impl ::core::convert::From<PubAlreadyExists> for BasinStorageErrors {
        fn from(value: PubAlreadyExists) -> Self {
            Self::PubAlreadyExists(value)
        }
    }
    impl ::core::convert::From<PubDoesNotExist> for BasinStorageErrors {
        fn from(value: PubDoesNotExist) -> Self {
            Self::PubDoesNotExist(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "DealAdded", abi = "DealAdded(uint256,string,address,string)")]
    pub struct DealAddedFilter {
        #[ethevent(indexed)]
        pub deal_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub pub_: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub cid: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PubCreated", abi = "PubCreated(string,address)")]
    pub struct PubCreatedFilter {
        #[ethevent(indexed)]
        pub pub_: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BasinStorageEvents {
        DealAddedFilter(DealAddedFilter),
        PubCreatedFilter(PubCreatedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BasinStorageEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DealAddedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::DealAddedFilter(decoded));
            }
            if let Ok(decoded) = PubCreatedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::PubCreatedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BasinStorageEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DealAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DealAddedFilter> for BasinStorageEvents {
        fn from(value: DealAddedFilter) -> Self {
            Self::DealAddedFilter(value)
        }
    }
    impl ::core::convert::From<PubCreatedFilter> for BasinStorageEvents {
        fn from(value: PubCreatedFilter) -> Self {
            Self::PubCreatedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for BasinStorageEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for BasinStorageEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for BasinStorageEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `PUB_ADMIN_ROLE` function with signature `PUB_ADMIN_ROLE()` and selector `0x822ba40b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PUB_ADMIN_ROLE", abi = "PUB_ADMIN_ROLE()")]
    pub struct PubAdminRoleCall;
    ///Container type for all input parameters for the `addDeals` function with signature `addDeals(string,(uint64,string,string)[])` and selector `0x570892c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addDeals", abi = "addDeals(string,(uint64,string,string)[])")]
    pub struct AddDealsCall {
        pub pub_: ::std::string::String,
        pub deals: ::std::vec::Vec<DealInfo>,
    }
    ///Container type for all input parameters for the `createPub` function with signature `createPub(address,string)` and selector `0x52b62b3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createPub", abi = "createPub(address,string)")]
    pub struct CreatePubCall {
        pub owner: ::ethers::core::types::Address,
        pub pub_: ::std::string::String,
    }
    ///Container type for all input parameters for the `dealActivation` function with signature `dealActivation(uint64)` and selector `0x9f29370b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealActivation", abi = "dealActivation(uint64)")]
    pub struct DealActivationCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealClient` function with signature `dealClient(uint64)` and selector `0x06a09dea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealClient", abi = "dealClient(uint64)")]
    pub struct DealClientCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealClientCollateral` function with signature `dealClientCollateral(uint64)` and selector `0x89ec0b93`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealClientCollateral", abi = "dealClientCollateral(uint64)")]
    pub struct DealClientCollateralCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealLabel` function with signature `dealLabel(uint64)` and selector `0x121e620e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealLabel", abi = "dealLabel(uint64)")]
    pub struct DealLabelCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealProvider` function with signature `dealProvider(uint64)` and selector `0xd06f6802`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealProvider", abi = "dealProvider(uint64)")]
    pub struct DealProviderCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealProviderCollateral` function with signature `dealProviderCollateral(uint64)` and selector `0x3c7e5999`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "dealProviderCollateral",
        abi = "dealProviderCollateral(uint64)"
    )]
    pub struct DealProviderCollateralCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealTerm` function with signature `dealTerm(uint64)` and selector `0x87a41b81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealTerm", abi = "dealTerm(uint64)")]
    pub struct DealTermCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealTotalPrice` function with signature `dealTotalPrice(uint64)` and selector `0x484d5a3a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealTotalPrice", abi = "dealTotalPrice(uint64)")]
    pub struct DealTotalPriceCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `dealVerified` function with signature `dealVerified(uint64)` and selector `0x3ff421e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dealVerified", abi = "dealVerified(uint64)")]
    pub struct DealVerifiedCall {
        pub deal_id: u64,
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestNDeals` function with signature `latestNDeals(string,uint256)` and selector `0x6f0a43c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "latestNDeals", abi = "latestNDeals(string,uint256)")]
    pub struct LatestNDealsCall {
        pub pub_: ::std::string::String,
        pub n: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `paginatedDeals` function with signature `paginatedDeals(string,uint256,uint256)` and selector `0x59b64c5d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "paginatedDeals",
        abi = "paginatedDeals(string,uint256,uint256)"
    )]
    pub struct PaginatedDealsCall {
        pub pub_: ::std::string::String,
        pub offset: ::ethers::core::types::U256,
        pub limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pubsOfOwner` function with signature `pubsOfOwner(address)` and selector `0x26294a77`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pubsOfOwner", abi = "pubsOfOwner(address)")]
    pub struct PubsOfOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BasinStorageCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        PubAdminRole(PubAdminRoleCall),
        AddDeals(AddDealsCall),
        CreatePub(CreatePubCall),
        DealActivation(DealActivationCall),
        DealClient(DealClientCall),
        DealClientCollateral(DealClientCollateralCall),
        DealLabel(DealLabelCall),
        DealProvider(DealProviderCall),
        DealProviderCollateral(DealProviderCollateralCall),
        DealTerm(DealTermCall),
        DealTotalPrice(DealTotalPriceCall),
        DealVerified(DealVerifiedCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        LatestNDeals(LatestNDealsCall),
        PaginatedDeals(PaginatedDealsCall),
        PubsOfOwner(PubsOfOwnerCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for BasinStorageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <PubAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubAdminRole(decoded));
            }
            if let Ok(decoded) = <AddDealsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddDeals(decoded));
            }
            if let Ok(decoded) = <CreatePubCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePub(decoded));
            }
            if let Ok(decoded) =
                <DealActivationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealActivation(decoded));
            }
            if let Ok(decoded) = <DealClientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DealClient(decoded));
            }
            if let Ok(decoded) =
                <DealClientCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealClientCollateral(decoded));
            }
            if let Ok(decoded) = <DealLabelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DealLabel(decoded));
            }
            if let Ok(decoded) = <DealProviderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealProvider(decoded));
            }
            if let Ok(decoded) =
                <DealProviderCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealProviderCollateral(decoded));
            }
            if let Ok(decoded) = <DealTermCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DealTerm(decoded));
            }
            if let Ok(decoded) =
                <DealTotalPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealTotalPrice(decoded));
            }
            if let Ok(decoded) = <DealVerifiedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DealVerified(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <LatestNDealsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestNDeals(decoded));
            }
            if let Ok(decoded) =
                <PaginatedDealsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PaginatedDeals(decoded));
            }
            if let Ok(decoded) = <PubsOfOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubsOfOwner(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasinStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubAdminRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddDeals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreatePub(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealActivation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealClientCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DealLabel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealProvider(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealProviderCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DealTerm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealTotalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealVerified(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GrantRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestNDeals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PaginatedDeals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubsOfOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BasinStorageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddDeals(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePub(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealActivation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealClientCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealLabel(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealProviderCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealTerm(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealTotalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestNDeals(element) => ::core::fmt::Display::fmt(element, f),
                Self::PaginatedDeals(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubsOfOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for BasinStorageCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<PubAdminRoleCall> for BasinStorageCalls {
        fn from(value: PubAdminRoleCall) -> Self {
            Self::PubAdminRole(value)
        }
    }
    impl ::core::convert::From<AddDealsCall> for BasinStorageCalls {
        fn from(value: AddDealsCall) -> Self {
            Self::AddDeals(value)
        }
    }
    impl ::core::convert::From<CreatePubCall> for BasinStorageCalls {
        fn from(value: CreatePubCall) -> Self {
            Self::CreatePub(value)
        }
    }
    impl ::core::convert::From<DealActivationCall> for BasinStorageCalls {
        fn from(value: DealActivationCall) -> Self {
            Self::DealActivation(value)
        }
    }
    impl ::core::convert::From<DealClientCall> for BasinStorageCalls {
        fn from(value: DealClientCall) -> Self {
            Self::DealClient(value)
        }
    }
    impl ::core::convert::From<DealClientCollateralCall> for BasinStorageCalls {
        fn from(value: DealClientCollateralCall) -> Self {
            Self::DealClientCollateral(value)
        }
    }
    impl ::core::convert::From<DealLabelCall> for BasinStorageCalls {
        fn from(value: DealLabelCall) -> Self {
            Self::DealLabel(value)
        }
    }
    impl ::core::convert::From<DealProviderCall> for BasinStorageCalls {
        fn from(value: DealProviderCall) -> Self {
            Self::DealProvider(value)
        }
    }
    impl ::core::convert::From<DealProviderCollateralCall> for BasinStorageCalls {
        fn from(value: DealProviderCollateralCall) -> Self {
            Self::DealProviderCollateral(value)
        }
    }
    impl ::core::convert::From<DealTermCall> for BasinStorageCalls {
        fn from(value: DealTermCall) -> Self {
            Self::DealTerm(value)
        }
    }
    impl ::core::convert::From<DealTotalPriceCall> for BasinStorageCalls {
        fn from(value: DealTotalPriceCall) -> Self {
            Self::DealTotalPrice(value)
        }
    }
    impl ::core::convert::From<DealVerifiedCall> for BasinStorageCalls {
        fn from(value: DealVerifiedCall) -> Self {
            Self::DealVerified(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for BasinStorageCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for BasinStorageCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for BasinStorageCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<LatestNDealsCall> for BasinStorageCalls {
        fn from(value: LatestNDealsCall) -> Self {
            Self::LatestNDeals(value)
        }
    }
    impl ::core::convert::From<PaginatedDealsCall> for BasinStorageCalls {
        fn from(value: PaginatedDealsCall) -> Self {
            Self::PaginatedDeals(value)
        }
    }
    impl ::core::convert::From<PubsOfOwnerCall> for BasinStorageCalls {
        fn from(value: PubsOfOwnerCall) -> Self {
            Self::PubsOfOwner(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for BasinStorageCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for BasinStorageCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for BasinStorageCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PUB_ADMIN_ROLE` function with signature `PUB_ADMIN_ROLE()` and selector `0x822ba40b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PubAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `dealActivation` function with signature `dealActivation(uint64)` and selector `0x9f29370b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealActivationReturn(pub i64, pub i64);
    ///Container type for all return fields from the `dealClient` function with signature `dealClient(uint64)` and selector `0x06a09dea`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealClientReturn(pub u64);
    ///Container type for all return fields from the `dealClientCollateral` function with signature `dealClientCollateral(uint64)` and selector `0x89ec0b93`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealClientCollateralReturn(pub BigInt);
    ///Container type for all return fields from the `dealLabel` function with signature `dealLabel(uint64)` and selector `0x121e620e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealLabelReturn(pub ::ethers::core::types::Bytes, pub bool);
    ///Container type for all return fields from the `dealProvider` function with signature `dealProvider(uint64)` and selector `0xd06f6802`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealProviderReturn(pub u64);
    ///Container type for all return fields from the `dealProviderCollateral` function with signature `dealProviderCollateral(uint64)` and selector `0x3c7e5999`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealProviderCollateralReturn(pub BigInt);
    ///Container type for all return fields from the `dealTerm` function with signature `dealTerm(uint64)` and selector `0x87a41b81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealTermReturn(pub i64, pub i64);
    ///Container type for all return fields from the `dealTotalPrice` function with signature `dealTotalPrice(uint64)` and selector `0x484d5a3a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealTotalPriceReturn(pub BigInt);
    ///Container type for all return fields from the `dealVerified` function with signature `dealVerified(uint64)` and selector `0x3ff421e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealVerifiedReturn(pub bool);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `latestNDeals` function with signature `latestNDeals(string,uint256)` and selector `0x6f0a43c7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LatestNDealsReturn(pub ::std::vec::Vec<DealInfo>);
    ///Container type for all return fields from the `paginatedDeals` function with signature `paginatedDeals(string,uint256,uint256)` and selector `0x59b64c5d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PaginatedDealsReturn(pub ::std::vec::Vec<DealInfo>);
    ///Container type for all return fields from the `pubsOfOwner` function with signature `pubsOfOwner(address)` and selector `0x26294a77`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PubsOfOwnerReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///`DealInfo(uint64,string,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DealInfo {
        pub id: u64,
        pub selector_path: ::std::string::String,
        pub cid: ::std::string::String,
    }
    ///`BigInt(bytes,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BigInt {
        pub val: ::ethers::core::types::Bytes,
        pub neg: bool,
    }
}
