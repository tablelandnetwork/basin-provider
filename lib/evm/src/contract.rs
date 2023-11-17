pub use basin_storage::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
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
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUB_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PUB_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addCID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addCID"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pub"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cidsAtTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cidsAtTimestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pub"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cidsInRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cidsInRange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pub"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aftr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("before"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createPub"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pubsOfOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pubsOfOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CIDAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CIDAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("IncorrectRange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aftr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("before"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PubAlreadyExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PubDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PubDoesNotExist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASINSTORAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x9DW3`\0\x90\x81R\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5` R`@\x81 T`\xFF\x16\x15a\0OW[Pa\x12{\x90\x81a\0\xA3\x829\xF3[\x80\x80R\x80` R`@\x81 3\x82R` R`@\x81 `\x01`\xFF\x19\x82T\x16\x17\x90U3\x903\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x81\x80\xA48a\0BV[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\xE7W\x80c$\x8A\x9C\xA3\x14a\0\xE2W\x80c&)Jw\x14a\0\xDDW\x80c//\xF1]\x14a\0\xD8W\x80c6V\x8A\xBE\x14a\0\xD3W\x80cR\xB6+>\x14a\0\xCEW\x80c\x82+\xA4\x0B\x14a\0\xC9W\x80c\x91\xD1HT\x14a\0\xC4W\x80c\xA2\x17\xFD\xDF\x14a\0\xBFW\x80c\xD4\x1B\xC3\xAE\x14a\0\xBAW\x80c\xD5Gt\x1F\x14a\0\xB5W\x80c\xDEf]\xBC\x14a\0\xB0Wc\xFD\x93hX\x14a\0\xABW`\0\x80\xFD[a\t$V[a\x07\xF8V[a\x07\xB9V[a\x06\x95V[a\x06yV[a\x06'V[a\x05\xECV[a\x04\xDAV[a\x04\x18V[a\x03TV[a\x02GV[a\x01BV[4a\x01=W` 6`\x03\x19\x01\x12a\x01=W`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01=W` \x90cye\xDB\x0B`\xE0\x1B\x81\x14\x90\x81\x15a\x01,W[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x01!V[`\0\x80\xFD[4a\x01=W` 6`\x03\x19\x01\x12a\x01=W`\x045`\0R`\0` R` `\x01`@`\0 \x01T`@Q\x90\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01=WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01=WV[`\0[\x83\x81\x10a\x01\xB0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xA0V[\x90` \x91a\x01\xD9\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\x9DV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a\x02\x19WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a\x027`\x01\x93`?\x19\x86\x82\x03\x01\x87R\x8AQa\x01\xC0V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a\x02\tV[4a\x01=W` \x80`\x03\x196\x01\x12a\x01=W`\x01`\x01`\xA0\x1B\x03a\x02ia\x01qV[\x16`\0\x90\x81R`\x02\x82R`@\x80\x82 \x91\x82T\x90a\x02\x85\x82a\x10\xF2V[\x93a\x02\x92\x84Q\x95\x86a\x0C\xAEV[\x82\x85R\x81R\x84\x81 \x94\x81\x90\x80\x86\x01[\x84\x83\x10a\x02\xB9W\x85Q\x80a\x02\xB5\x89\x82a\x01\xE5V[\x03\x90\xF3[\x85Q\x82\x85\x92\x8AT\x92a\x02\xCA\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x038WP`\x01\x14a\x03\0W[Pa\x02\xF2\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x98\x01\x92\x01\x91\x96a\x02\xA1V[\x8C\x89R\x83\x89 \x95P\x88\x90[\x80\x82\x10a\x03!WP\x81\x01\x83\x01\x94Pa\x02\xF2a\x02\xE3V[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x87\x94\x90\x91\x01\x90a\x03\x0BV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x02\xF2a\x02\xE3V[4a\x01=W`@6`\x03\x19\x01\x12a\x01=W`\x045a\x03pa\x01\x87V[`\0\x91\x80\x83R\x82` Ra\x03\x8A`\x01`@\x85 \x01Ta\x0B\x9DV[\x80\x83R` \x83\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R\x92R\x90 T`\xFF\x16\x15a\x03\xB7W\x82\x80\xF3[\x80\x83R` \x83\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R\x92R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x84\x80\xA48\x80\x82\x80\xF3[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x041a\x01\x87V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x04OWa\x04M\x90`\x045a\x0C\xE4V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x01=W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01=W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01=WV[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x04\xF3a\x01qV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x05\x13\x906\x90`\x04\x01a\x04\xACV[a\x05\x1Ba\n%V[`@Q\x91\x81\x81\x847`\x01\x83\x83\x01\x90\x81R\x83\x90\x03` \x01\x90\x92 T`\x01`\x01`\xA0\x1B\x03\x92\x90\x83\x16a\x05\xCCW\x90\x81a\x05w\x85a\x05X\x84a\x05\xA2\x96a\x0E\xCEV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x05\x9D\x82\x82a\x05\x98\x88`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x0F\xCFV[a\x10\xCEV[\x91\x16\x90\x7F\xF8\xDE\xBC/\x17E\xEB\xA8i\t\x89\x0F-\xC0abG\x05\xC7C)4\x88)\xE0J\xBAC\xC0\x15\xB9\xA2`\0\x80\xA3\0[a\x05\xE8`@Q\x92\x83\x92c\\x\xF6\xED`\xE1\x1B\x84R`\x04\x84\x01a\x0F\x19V[\x03\x90\xFD[4a\x01=W`\x006`\x03\x19\x01\x12a\x01=W` `@Q\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x81R\xF3[4a\x01=W`@6`\x03\x19\x01\x12a\x01=W` `\xFFa\x06ma\x06Ga\x01\x87V[`\x045`\0R`\0\x84R`@`\0 \x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01=W`\x006`\x03\x19\x01\x12a\x01=W` `@Q`\0\x81R\xF3[4a\x01=W`@\x80`\x03\x196\x01\x12a\x01=W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x06\xCAa\x06\xD0\x916\x90`\x04\x01a\x04\xACV[\x90a\x0E\xE7V[\x90`\0\x90`$5\x82R` \x92\x83R\x80\x82 \x91\x82T\x90a\x06\xEE\x82a\x10\xF2V[\x93a\x06\xFB\x84Q\x95\x86a\x0C\xAEV[\x82\x85R\x81R\x84\x81 \x94\x81\x90\x80\x86\x01[\x84\x83\x10a\x07\x1EW\x85Q\x80a\x02\xB5\x89\x82a\x01\xE5V[\x85Q\x82\x85\x92\x8AT\x92a\x07/\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x07\x9DWP`\x01\x14a\x07eW[Pa\x07W\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x98\x01\x92\x01\x91\x96a\x07\nV[\x8C\x89R\x83\x89 \x95P\x88\x90[\x80\x82\x10a\x07\x86WP\x81\x01\x83\x01\x94Pa\x07Wa\x07HV[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x87\x94\x90\x91\x01\x90a\x07pV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x07Wa\x07HV[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x04M`\x045a\x07\xD8a\x01\x87V[\x90\x80`\0R`\0` Ra\x07\xF3`\x01`@`\0 \x01Ta\x0B\x9DV[a\x0C\xE4V[4a\x01=W``6`\x03\x19\x01\x12a\x01=W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x08)\x906\x90`\x04\x01a\x04\xACV[`$5\x91`D5\x90\x81\x84\x10\x15a\t\x02W\x92\x90a\x08Na\x08H\x84\x86a\x0F\0V[Ta\x11\xE7V[\x92`\0\x91a\x08\\\x83\x94a\r\x84V[\x92[\x81\x84\x10a\x08vW\x84\x86R`@Q\x80a\x02\xB5\x88\x82a\x01\xE5V[a\x08\xA0a\x08\x9B\x85a\x08\x8C\x86\x8B\x9A\x99\x97\x98\x9Aa\x0E\xE7V[\x90`\0R` R`@`\0 \x90V[a\x11\nV[\x93\x81\x93[\x85Q\x85\x10\x15a\x08\xE6Wa\x08\xDAa\x08\xE0\x91a\x08\xBE\x87\x89a\x121V[Qa\x08\xC9\x82\x8Ba\x121V[Ra\x08\xD4\x81\x8Aa\x121V[Pa\x10\xE3V[\x94a\x10\xE3V[\x93a\x08\xA4V[\x97\x92\x95\x96\x90\x93Pa\x08\xF8\x91\x94Pa\x10\xE3V[\x93\x90\x95\x93\x92a\x08^V[P`@Qc\xBC\x0C\x88\x85`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[4a\x01=W``6`\x03\x19\x01\x12a\x01=Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\x01=Wa\tV\x906\x90`\x04\x01a\x04\xACV[\x91`$5\x90\x81\x11a\x01=Wa\to\x906\x90`\x04\x01a\x04\xACV[a\tz\x93\x91\x93a\n%V[`@Q\x82\x84\x827`\x01\x81\x84\x01\x90\x81R\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x84\x15a\n\tW\x81\x81a\t\xC0a\t\xE1\x95\x94a\t\xDB\x94a\x05\x98`D5a\x08\x8C\x89\x8Ca\x0E\xE7V[a\t\xCA\x84\x87a\x0F\0V[a\t\xD4\x81Ta\x10\xE3V[\x90Ua\x10\xCEV[\x92a\x10\xCEV[\x90\x7F\xE3\xF9\xA4[\xA3\xCD\xF7E}\x98=Qg\x88\xBD\x8A]i\xA8\x02\xC3\xBC\xB40\xD0\x8E\x86[\x11I\x86\xF0`\0\x80\xA4\0[`@Qc\x15\xE6\xE0\xEB`\xE2\x1B\x81R\x80a\x05\xE8\x85\x87`\x04\x84\x01a\x0F\x19V[3`\0\x90\x81R\x7F\x1B\x02\\_t\x93\x12~\x9EBbQ\x9D\x0B\x05\x1A7g\xD7\xB2A\xDE>\x06\x84\xFDV\xF9\xE8#[`` R`@\x90 T\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x90`\xFF\x16\x15a\n\x81WPV[a\n\x8A3a\x0ENV[a\n\x92a\r\x97V[\x91`0a\n\x9E\x84a\r\xC8V[S`xa\n\xAA\x84a\r\xD5V[S`A[`\x01\x81\x11a\x0BVWa\x05\xE8`Ha\x0B>\x85a\x0B0\x88a\n\xCD\x88\x15a\x0E\x03V[`@Q\x94\x85\x93\x7FAccessControl: account \0\0\0\0\0\0\0\0\0` \x86\x01Ra\x0B\r\x81Q\x80\x92` `7\x89\x01\x91\x01a\x01\x9DV[\x84\x01p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`7\x82\x01R\x01\x90a\x0CDV[\x03`\x1F\x19\x81\x01\x83R\x82a\x0C\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x0C\xD0V[\x90`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0B\x93\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x87a\r\xE5V[S`\x04\x1C\x91a\r\xF6V[a\n\xAEV[a\r\xB2V[`\0\x81\x81R` \x81\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0B\xC0WPV[a\x0B\xC93a\x0ENV[a\x0B\xD1a\r\x97V[\x91`0a\x0B\xDD\x84a\r\xC8V[S`xa\x0B\xE9\x84a\r\xD5V[S`A[`\x01\x81\x11a\x0C\x0CWa\x05\xE8`Ha\x0B>\x85a\x0B0\x88a\n\xCD\x88\x15a\x0E\x03V[\x90`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0C?\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x87a\r\xE5V[a\x0B\xEDV[\x90a\x0CW` \x92\x82\x81Q\x94\x85\x92\x01a\x01\x9DV[\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[a\x0C[V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[\x90` a\x0C\xE1\x92\x81\x81R\x01\x90a\x01\xC0V[\x90V[`\0\x81\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x81 T\x90\x91\x90`\xFF\x16a\r\x14WPPPV[\x80\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R\x92R\x90 \x80T`\xFF\x19\x16\x90U3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x90\x80\xA4V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\r\x92WV[a\rnV[`@Q\x90a\r\xA4\x82a\x0CqV[`B\x82R``6` \x84\x017V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q\x15a\x0B\x98W` \x01\x90V[\x80Q`\x01\x10\x15a\x0B\x98W`!\x01\x90V[\x90\x81Q\x81\x10\x15a\x0B\x98W\x01` \x01\x90V[\x80\x15a\r\x92W`\0\x19\x01\x90V[\x15a\x0E\nWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R\xFD[`@Q\x90a\x0E[\x82a\x0C\x92V[`*\x82R`@6` \x84\x017`0a\x0Er\x83a\r\xC8V[S`xa\x0E~\x83a\r\xD5V[S`)\x90[`\x01\x82\x11a\x0E\x96Wa\x0C\xE1\x91P\x15a\x0E\x03V[`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0E\xC8\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x86a\r\xE5V[\x90a\x0E\x83V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x01\x81R\x03\x01\x90 \x90V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x04\x81R\x03\x01\x90 \x90V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x03\x81R\x03\x01\x90 \x90V[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0FqW[` \x83\x10\x14a\x0F[WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0FPV[\x90`\x1F\x81\x11a\x0F\x89WPPPV[`\0\x91\x82R` \x82 \x90` `\x1F\x85\x01`\x05\x1C\x83\x01\x94\x10a\x0F\xC5W[`\x1F\x01`\x05\x1C\x01\x91[\x82\x81\x10a\x0F\xBAWPPPV[\x81\x81U`\x01\x01a\x0F\xAEV[\x90\x92P\x82\x90a\x0F\xA5V[\x90\x91\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0C\x8DW`\x01\x92\x83\x82\x01\x80\x82U\x82\x10\x15a\x0B\x98W`\0R` \x90\x81`\0 \x01\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x0C\x8DWa\x10#\x83a\x10\x1D\x87Ta\x0FAV[\x87a\x0F{V[`\0\x91`\x1F\x84\x11`\x01\x14a\x10eWPa\x10V\x93P`\0\x91\x90\x83a\x10ZW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90UV[\x015\x90P8\x80a\x10AV[\x91\x83`\x1F\x19\x81\x16a\x10{\x88`\0R` `\0 \x90V[\x94\x83\x90[\x88\x83\x83\x10a\x10\xB4WPPP\x10a\x10\x9AW[PPP\x81\x1B\x01\x90UV[\x015`\0\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U8\x80\x80a\x10\x90V[\x86\x86\x015\x88U\x90\x96\x01\x95\x93\x84\x01\x93\x87\x93P\x90\x81\x01\x90a\x10\x7FV[\x81`@Q\x92\x83\x92\x837\x81\x01`\0\x81R\x03\x90 \x90V[`\0\x19\x81\x14a\r\x92W`\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C\x8DW`\x05\x1B` \x01\x90V[\x90\x81T\x91a\x11\x17\x83a\x10\xF2V[\x92`@\x91a\x11'\x83Q\x95\x86a\x0C\xAEV[\x81\x85R`\0\x90\x81R` \x80\x82 \x93\x82\x91\x90\x81\x88\x01[\x85\x84\x10a\x11LWPPPPPPPV[\x81Q\x83\x86\x92\x89T\x92a\x11]\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x11\xCBWP`\x01\x14a\x11\x93W[Pa\x11\x85\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x97\x01\x93\x01\x92\x95a\x11<V[\x8B\x8AR\x83\x8A \x95P\x89\x90[\x80\x82\x10a\x11\xB4WP\x81\x01\x83\x01\x94Pa\x11\x85a\x11vV[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x88\x94\x90\x91\x01\x90a\x11\x9EV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x11\x85a\x11vV[\x90a\x11\xF1\x82a\x10\xF2V[a\x11\xFE`@Q\x91\x82a\x0C\xAEV[\x82\x81R\x80\x92a\x12\x0F`\x1F\x19\x91a\x10\xF2V[\x01\x90`\0[\x82\x81\x10a\x12 WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x12\x14V[\x80Q\x82\x10\x15a\x0B\x98W` \x91`\x05\x1B\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xA6q\x08h\x1C\x07\x0B\x88\x92Q1\x94\xCD\xE0\xD9\xC1\x95\x01u\xE4\x85O/\xAFx8\xC4\xB6'\xBE)\xC3dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BASINSTORAGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\xE7W\x80c$\x8A\x9C\xA3\x14a\0\xE2W\x80c&)Jw\x14a\0\xDDW\x80c//\xF1]\x14a\0\xD8W\x80c6V\x8A\xBE\x14a\0\xD3W\x80cR\xB6+>\x14a\0\xCEW\x80c\x82+\xA4\x0B\x14a\0\xC9W\x80c\x91\xD1HT\x14a\0\xC4W\x80c\xA2\x17\xFD\xDF\x14a\0\xBFW\x80c\xD4\x1B\xC3\xAE\x14a\0\xBAW\x80c\xD5Gt\x1F\x14a\0\xB5W\x80c\xDEf]\xBC\x14a\0\xB0Wc\xFD\x93hX\x14a\0\xABW`\0\x80\xFD[a\t$V[a\x07\xF8V[a\x07\xB9V[a\x06\x95V[a\x06yV[a\x06'V[a\x05\xECV[a\x04\xDAV[a\x04\x18V[a\x03TV[a\x02GV[a\x01BV[4a\x01=W` 6`\x03\x19\x01\x12a\x01=W`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01=W` \x90cye\xDB\x0B`\xE0\x1B\x81\x14\x90\x81\x15a\x01,W[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x01!V[`\0\x80\xFD[4a\x01=W` 6`\x03\x19\x01\x12a\x01=W`\x045`\0R`\0` R` `\x01`@`\0 \x01T`@Q\x90\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01=WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01=WV[`\0[\x83\x81\x10a\x01\xB0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xA0V[\x90` \x91a\x01\xD9\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\x9DV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a\x02\x19WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a\x027`\x01\x93`?\x19\x86\x82\x03\x01\x87R\x8AQa\x01\xC0V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a\x02\tV[4a\x01=W` \x80`\x03\x196\x01\x12a\x01=W`\x01`\x01`\xA0\x1B\x03a\x02ia\x01qV[\x16`\0\x90\x81R`\x02\x82R`@\x80\x82 \x91\x82T\x90a\x02\x85\x82a\x10\xF2V[\x93a\x02\x92\x84Q\x95\x86a\x0C\xAEV[\x82\x85R\x81R\x84\x81 \x94\x81\x90\x80\x86\x01[\x84\x83\x10a\x02\xB9W\x85Q\x80a\x02\xB5\x89\x82a\x01\xE5V[\x03\x90\xF3[\x85Q\x82\x85\x92\x8AT\x92a\x02\xCA\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x038WP`\x01\x14a\x03\0W[Pa\x02\xF2\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x98\x01\x92\x01\x91\x96a\x02\xA1V[\x8C\x89R\x83\x89 \x95P\x88\x90[\x80\x82\x10a\x03!WP\x81\x01\x83\x01\x94Pa\x02\xF2a\x02\xE3V[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x87\x94\x90\x91\x01\x90a\x03\x0BV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x02\xF2a\x02\xE3V[4a\x01=W`@6`\x03\x19\x01\x12a\x01=W`\x045a\x03pa\x01\x87V[`\0\x91\x80\x83R\x82` Ra\x03\x8A`\x01`@\x85 \x01Ta\x0B\x9DV[\x80\x83R` \x83\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R\x92R\x90 T`\xFF\x16\x15a\x03\xB7W\x82\x80\xF3[\x80\x83R` \x83\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R\x92R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x84\x80\xA48\x80\x82\x80\xF3[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x041a\x01\x87V[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x04OWa\x04M\x90`\x045a\x0C\xE4V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x01=W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01=W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01=WV[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x04\xF3a\x01qV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x05\x13\x906\x90`\x04\x01a\x04\xACV[a\x05\x1Ba\n%V[`@Q\x91\x81\x81\x847`\x01\x83\x83\x01\x90\x81R\x83\x90\x03` \x01\x90\x92 T`\x01`\x01`\xA0\x1B\x03\x92\x90\x83\x16a\x05\xCCW\x90\x81a\x05w\x85a\x05X\x84a\x05\xA2\x96a\x0E\xCEV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x05\x9D\x82\x82a\x05\x98\x88`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x0F\xCFV[a\x10\xCEV[\x91\x16\x90\x7F\xF8\xDE\xBC/\x17E\xEB\xA8i\t\x89\x0F-\xC0abG\x05\xC7C)4\x88)\xE0J\xBAC\xC0\x15\xB9\xA2`\0\x80\xA3\0[a\x05\xE8`@Q\x92\x83\x92c\\x\xF6\xED`\xE1\x1B\x84R`\x04\x84\x01a\x0F\x19V[\x03\x90\xFD[4a\x01=W`\x006`\x03\x19\x01\x12a\x01=W` `@Q\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x81R\xF3[4a\x01=W`@6`\x03\x19\x01\x12a\x01=W` `\xFFa\x06ma\x06Ga\x01\x87V[`\x045`\0R`\0\x84R`@`\0 \x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01=W`\x006`\x03\x19\x01\x12a\x01=W` `@Q`\0\x81R\xF3[4a\x01=W`@\x80`\x03\x196\x01\x12a\x01=W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x06\xCAa\x06\xD0\x916\x90`\x04\x01a\x04\xACV[\x90a\x0E\xE7V[\x90`\0\x90`$5\x82R` \x92\x83R\x80\x82 \x91\x82T\x90a\x06\xEE\x82a\x10\xF2V[\x93a\x06\xFB\x84Q\x95\x86a\x0C\xAEV[\x82\x85R\x81R\x84\x81 \x94\x81\x90\x80\x86\x01[\x84\x83\x10a\x07\x1EW\x85Q\x80a\x02\xB5\x89\x82a\x01\xE5V[\x85Q\x82\x85\x92\x8AT\x92a\x07/\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x07\x9DWP`\x01\x14a\x07eW[Pa\x07W\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x98\x01\x92\x01\x91\x96a\x07\nV[\x8C\x89R\x83\x89 \x95P\x88\x90[\x80\x82\x10a\x07\x86WP\x81\x01\x83\x01\x94Pa\x07Wa\x07HV[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x87\x94\x90\x91\x01\x90a\x07pV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x07Wa\x07HV[4a\x01=W`@6`\x03\x19\x01\x12a\x01=Wa\x04M`\x045a\x07\xD8a\x01\x87V[\x90\x80`\0R`\0` Ra\x07\xF3`\x01`@`\0 \x01Ta\x0B\x9DV[a\x0C\xE4V[4a\x01=W``6`\x03\x19\x01\x12a\x01=W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01=Wa\x08)\x906\x90`\x04\x01a\x04\xACV[`$5\x91`D5\x90\x81\x84\x10\x15a\t\x02W\x92\x90a\x08Na\x08H\x84\x86a\x0F\0V[Ta\x11\xE7V[\x92`\0\x91a\x08\\\x83\x94a\r\x84V[\x92[\x81\x84\x10a\x08vW\x84\x86R`@Q\x80a\x02\xB5\x88\x82a\x01\xE5V[a\x08\xA0a\x08\x9B\x85a\x08\x8C\x86\x8B\x9A\x99\x97\x98\x9Aa\x0E\xE7V[\x90`\0R` R`@`\0 \x90V[a\x11\nV[\x93\x81\x93[\x85Q\x85\x10\x15a\x08\xE6Wa\x08\xDAa\x08\xE0\x91a\x08\xBE\x87\x89a\x121V[Qa\x08\xC9\x82\x8Ba\x121V[Ra\x08\xD4\x81\x8Aa\x121V[Pa\x10\xE3V[\x94a\x10\xE3V[\x93a\x08\xA4V[\x97\x92\x95\x96\x90\x93Pa\x08\xF8\x91\x94Pa\x10\xE3V[\x93\x90\x95\x93\x92a\x08^V[P`@Qc\xBC\x0C\x88\x85`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[4a\x01=W``6`\x03\x19\x01\x12a\x01=Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\x01=Wa\tV\x906\x90`\x04\x01a\x04\xACV[\x91`$5\x90\x81\x11a\x01=Wa\to\x906\x90`\x04\x01a\x04\xACV[a\tz\x93\x91\x93a\n%V[`@Q\x82\x84\x827`\x01\x81\x84\x01\x90\x81R\x81\x90\x03` \x01\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x84\x15a\n\tW\x81\x81a\t\xC0a\t\xE1\x95\x94a\t\xDB\x94a\x05\x98`D5a\x08\x8C\x89\x8Ca\x0E\xE7V[a\t\xCA\x84\x87a\x0F\0V[a\t\xD4\x81Ta\x10\xE3V[\x90Ua\x10\xCEV[\x92a\x10\xCEV[\x90\x7F\xE3\xF9\xA4[\xA3\xCD\xF7E}\x98=Qg\x88\xBD\x8A]i\xA8\x02\xC3\xBC\xB40\xD0\x8E\x86[\x11I\x86\xF0`\0\x80\xA4\0[`@Qc\x15\xE6\xE0\xEB`\xE2\x1B\x81R\x80a\x05\xE8\x85\x87`\x04\x84\x01a\x0F\x19V[3`\0\x90\x81R\x7F\x1B\x02\\_t\x93\x12~\x9EBbQ\x9D\x0B\x05\x1A7g\xD7\xB2A\xDE>\x06\x84\xFDV\xF9\xE8#[`` R`@\x90 T\x7F\xAF\xDAe\x8E\xE71\xB8\xF8b\x92\xE3\xB5*1\x154\xCD\x93d+\x12\xA6\x98\x01$91n\x0C:\t\x95\x90`\xFF\x16\x15a\n\x81WPV[a\n\x8A3a\x0ENV[a\n\x92a\r\x97V[\x91`0a\n\x9E\x84a\r\xC8V[S`xa\n\xAA\x84a\r\xD5V[S`A[`\x01\x81\x11a\x0BVWa\x05\xE8`Ha\x0B>\x85a\x0B0\x88a\n\xCD\x88\x15a\x0E\x03V[`@Q\x94\x85\x93\x7FAccessControl: account \0\0\0\0\0\0\0\0\0` \x86\x01Ra\x0B\r\x81Q\x80\x92` `7\x89\x01\x91\x01a\x01\x9DV[\x84\x01p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`7\x82\x01R\x01\x90a\x0CDV[\x03`\x1F\x19\x81\x01\x83R\x82a\x0C\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x0C\xD0V[\x90`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0B\x93\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x87a\r\xE5V[S`\x04\x1C\x91a\r\xF6V[a\n\xAEV[a\r\xB2V[`\0\x81\x81R` \x81\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0B\xC0WPV[a\x0B\xC93a\x0ENV[a\x0B\xD1a\r\x97V[\x91`0a\x0B\xDD\x84a\r\xC8V[S`xa\x0B\xE9\x84a\r\xD5V[S`A[`\x01\x81\x11a\x0C\x0CWa\x05\xE8`Ha\x0B>\x85a\x0B0\x88a\n\xCD\x88\x15a\x0E\x03V[\x90`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0C?\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x87a\r\xE5V[a\x0B\xEDV[\x90a\x0CW` \x92\x82\x81Q\x94\x85\x92\x01a\x01\x9DV[\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[a\x0C[V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\x8DW`@RV[\x90` a\x0C\xE1\x92\x81\x81R\x01\x90a\x01\xC0V[\x90V[`\0\x81\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x81 T\x90\x91\x90`\xFF\x16a\r\x14WPPPV[\x80\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R\x92R\x90 \x80T`\xFF\x19\x16\x90U3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x90\x80\xA4V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\r\x92WV[a\rnV[`@Q\x90a\r\xA4\x82a\x0CqV[`B\x82R``6` \x84\x017V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q\x15a\x0B\x98W` \x01\x90V[\x80Q`\x01\x10\x15a\x0B\x98W`!\x01\x90V[\x90\x81Q\x81\x10\x15a\x0B\x98W\x01` \x01\x90V[\x80\x15a\r\x92W`\0\x19\x01\x90V[\x15a\x0E\nWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R\xFD[`@Q\x90a\x0E[\x82a\x0C\x92V[`*\x82R`@6` \x84\x017`0a\x0Er\x83a\r\xC8V[S`xa\x0E~\x83a\r\xD5V[S`)\x90[`\x01\x82\x11a\x0E\x96Wa\x0C\xE1\x91P\x15a\x0E\x03V[`\x0F\x81\x16\x90`\x10\x82\x10\x15a\x0B\x98Wa\x0E\xC8\x91o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1Aa\x0B\x89\x84\x86a\r\xE5V[\x90a\x0E\x83V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x01\x81R\x03\x01\x90 \x90V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x04\x81R\x03\x01\x90 \x90V[` \x90\x82`@Q\x93\x84\x92\x837\x81\x01`\x03\x81R\x03\x01\x90 \x90V[\x90\x91\x80`@\x93` \x84R\x81` \x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0FqW[` \x83\x10\x14a\x0F[WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0FPV[\x90`\x1F\x81\x11a\x0F\x89WPPPV[`\0\x91\x82R` \x82 \x90` `\x1F\x85\x01`\x05\x1C\x83\x01\x94\x10a\x0F\xC5W[`\x1F\x01`\x05\x1C\x01\x91[\x82\x81\x10a\x0F\xBAWPPPV[\x81\x81U`\x01\x01a\x0F\xAEV[\x90\x92P\x82\x90a\x0F\xA5V[\x90\x91\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0C\x8DW`\x01\x92\x83\x82\x01\x80\x82U\x82\x10\x15a\x0B\x98W`\0R` \x90\x81`\0 \x01\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x0C\x8DWa\x10#\x83a\x10\x1D\x87Ta\x0FAV[\x87a\x0F{V[`\0\x91`\x1F\x84\x11`\x01\x14a\x10eWPa\x10V\x93P`\0\x91\x90\x83a\x10ZW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90UV[\x015\x90P8\x80a\x10AV[\x91\x83`\x1F\x19\x81\x16a\x10{\x88`\0R` `\0 \x90V[\x94\x83\x90[\x88\x83\x83\x10a\x10\xB4WPPP\x10a\x10\x9AW[PPP\x81\x1B\x01\x90UV[\x015`\0\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U8\x80\x80a\x10\x90V[\x86\x86\x015\x88U\x90\x96\x01\x95\x93\x84\x01\x93\x87\x93P\x90\x81\x01\x90a\x10\x7FV[\x81`@Q\x92\x83\x92\x837\x81\x01`\0\x81R\x03\x90 \x90V[`\0\x19\x81\x14a\r\x92W`\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C\x8DW`\x05\x1B` \x01\x90V[\x90\x81T\x91a\x11\x17\x83a\x10\xF2V[\x92`@\x91a\x11'\x83Q\x95\x86a\x0C\xAEV[\x81\x85R`\0\x90\x81R` \x80\x82 \x93\x82\x91\x90\x81\x88\x01[\x85\x84\x10a\x11LWPPPPPPPV[\x81Q\x83\x86\x92\x89T\x92a\x11]\x84a\x0FAV[\x80\x82R`\x01\x94\x80\x86\x16\x90\x81\x15a\x11\xCBWP`\x01\x14a\x11\x93W[Pa\x11\x85\x81`\x01\x96\x03\x82a\x0C\xAEV[\x81R\x01\x97\x01\x93\x01\x92\x95a\x11<V[\x8B\x8AR\x83\x8A \x95P\x89\x90[\x80\x82\x10a\x11\xB4WP\x81\x01\x83\x01\x94Pa\x11\x85a\x11vV[\x86T\x83\x83\x01\x86\x01R\x95\x85\x01\x95\x88\x94\x90\x91\x01\x90a\x11\x9EV[`\xFF\x19\x16\x85\x84\x01RP\x15\x15`\x05\x1B\x81\x01\x83\x01\x94Pa\x11\x85a\x11vV[\x90a\x11\xF1\x82a\x10\xF2V[a\x11\xFE`@Q\x91\x82a\x0C\xAEV[\x82\x81R\x80\x92a\x12\x0F`\x1F\x19\x91a\x10\xF2V[\x01\x90`\0[\x82\x81\x10a\x12 WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a\x12\x14V[\x80Q\x82\x10\x15a\x0B\x98W` \x91`\x05\x1B\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xA6q\x08h\x1C\x07\x0B\x88\x92Q1\x94\xCD\xE0\xD9\xC1\x95\x01u\xE4\x85O/\xAFx8\xC4\xB6'\xBE)\xC3dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BASINSTORAGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASINSTORAGE_ABI.clone(),
                    client,
                ),
            )
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
        pub fn pub_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([130, 43, 164, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addCID` (0xfd936858) function
        pub fn add_cid(
            &self,
            pub_: ::std::string::String,
            cid: ::std::string::String,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 147, 104, 88], (pub_, cid, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cidsAtTimestamp` (0xd41bc3ae) function
        pub fn cids_at_timestamp(
            &self,
            pub_: ::std::string::String,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([212, 27, 195, 174], (pub_, epoch))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cidsInRange` (0xde665dbc) function
        pub fn cids_in_range(
            &self,
            pub_: ::std::string::String,
            aftr: ::ethers::core::types::U256,
            before: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([222, 102, 93, 188], (pub_, aftr, before))
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
        ///Calls the contract's `pubsOfOwner` (0x26294a77) function
        pub fn pubs_of_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
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
        ///Gets the contract's `CIDAdded` event
        pub fn cid_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CidaddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PubCreated` event
        pub fn pub_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PubCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BasinStorageEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BasinStorage<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `IncorrectRange` with signature `IncorrectRange(uint256,uint256)` and selector `0xbc0c8885`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "IncorrectRange", abi = "IncorrectRange(uint256,uint256)")]
    pub struct IncorrectRange {
        pub aftr: ::ethers::core::types::U256,
        pub before: ::ethers::core::types::U256,
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
        Hash
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
        Hash
    )]
    #[etherror(name = "PubDoesNotExist", abi = "PubDoesNotExist(string)")]
    pub struct PubDoesNotExist {
        pub reason: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BasinStorageErrors {
        IncorrectRange(IncorrectRange),
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
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <IncorrectRange as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IncorrectRange(decoded));
            }
            if let Ok(decoded)
                = <PubAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubAlreadyExists(decoded));
            }
            if let Ok(decoded)
                = <PubDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubDoesNotExist(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasinStorageErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::IncorrectRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BasinStorageErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <IncorrectRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PubAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PubDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BasinStorageErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IncorrectRange(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<IncorrectRange> for BasinStorageErrors {
        fn from(value: IncorrectRange) -> Self {
            Self::IncorrectRange(value)
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
        Hash
    )]
    #[ethevent(name = "CIDAdded", abi = "CIDAdded(string,string,address)")]
    pub struct CidaddedFilter {
        #[ethevent(indexed)]
        pub cid: ::ethers::core::types::H256,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        CidaddedFilter(CidaddedFilter),
        PubCreatedFilter(PubCreatedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BasinStorageEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CidaddedFilter::decode_log(log) {
                return Ok(BasinStorageEvents::CidaddedFilter(decoded));
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
                Self::CidaddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CidaddedFilter> for BasinStorageEvents {
        fn from(value: CidaddedFilter) -> Self {
            Self::CidaddedFilter(value)
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "PUB_ADMIN_ROLE", abi = "PUB_ADMIN_ROLE()")]
    pub struct PubAdminRoleCall;
    ///Container type for all input parameters for the `addCID` function with signature `addCID(string,string,uint256)` and selector `0xfd936858`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addCID", abi = "addCID(string,string,uint256)")]
    pub struct AddCIDCall {
        pub pub_: ::std::string::String,
        pub cid: ::std::string::String,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cidsAtTimestamp` function with signature `cidsAtTimestamp(string,uint256)` and selector `0xd41bc3ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cidsAtTimestamp", abi = "cidsAtTimestamp(string,uint256)")]
    pub struct CidsAtTimestampCall {
        pub pub_: ::std::string::String,
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cidsInRange` function with signature `cidsInRange(string,uint256,uint256)` and selector `0xde665dbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cidsInRange", abi = "cidsInRange(string,uint256,uint256)")]
    pub struct CidsInRangeCall {
        pub pub_: ::std::string::String,
        pub aftr: ::ethers::core::types::U256,
        pub before: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethcall(name = "createPub", abi = "createPub(address,string)")]
    pub struct CreatePubCall {
        pub owner: ::ethers::core::types::Address,
        pub pub_: ::std::string::String,
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        AddCID(AddCIDCall),
        CidsAtTimestamp(CidsAtTimestampCall),
        CidsInRange(CidsInRangeCall),
        CreatePub(CreatePubCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
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
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <PubAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubAdminRole(decoded));
            }
            if let Ok(decoded)
                = <AddCIDCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddCID(decoded));
            }
            if let Ok(decoded)
                = <CidsAtTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CidsAtTimestamp(decoded));
            }
            if let Ok(decoded)
                = <CidsInRangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CidsInRange(decoded));
            }
            if let Ok(decoded)
                = <CreatePubCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePub(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <PubsOfOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PubsOfOwner(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasinStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddCID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CidsAtTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CidsInRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePub(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubsOfOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BasinStorageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddCID(element) => ::core::fmt::Display::fmt(element, f),
                Self::CidsAtTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::CidsInRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePub(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AddCIDCall> for BasinStorageCalls {
        fn from(value: AddCIDCall) -> Self {
            Self::AddCID(value)
        }
    }
    impl ::core::convert::From<CidsAtTimestampCall> for BasinStorageCalls {
        fn from(value: CidsAtTimestampCall) -> Self {
            Self::CidsAtTimestamp(value)
        }
    }
    impl ::core::convert::From<CidsInRangeCall> for BasinStorageCalls {
        fn from(value: CidsInRangeCall) -> Self {
            Self::CidsInRange(value)
        }
    }
    impl ::core::convert::From<CreatePubCall> for BasinStorageCalls {
        fn from(value: CreatePubCall) -> Self {
            Self::CreatePub(value)
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
        Hash
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
        Hash
    )]
    pub struct PubAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cidsAtTimestamp` function with signature `cidsAtTimestamp(string,uint256)` and selector `0xd41bc3ae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CidsAtTimestampReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `cidsInRange` function with signature `cidsInRange(string,uint256,uint256)` and selector `0xde665dbc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CidsInRangeReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `pubsOfOwner` function with signature `pubsOfOwner(address)` and selector `0x26294a77`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
