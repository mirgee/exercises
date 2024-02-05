pub use campaign_factory::*;
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
pub mod campaign_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("createCampaign"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createCampaign"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimum"),
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
                    ::std::borrow::ToOwned::to_owned("deployedCampaigns"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployedCampaigns"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployedCampaigns"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDeployedCampaigns",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CAMPAIGNFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct CampaignFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CampaignFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CampaignFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CampaignFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CampaignFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CampaignFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CampaignFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CAMPAIGNFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `createCampaign` (0xa3303a75) function
        pub fn create_campaign(
            &self,
            minimum: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 48, 58, 117], minimum)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployedCampaigns` (0x339d50a5) function
        pub fn deployed_campaigns(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([51, 157, 80, 165], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedCampaigns` (0x4acb9d4f) function
        pub fn get_deployed_campaigns(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([74, 203, 157, 79], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CampaignFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `createCampaign` function with signature `createCampaign(uint256)` and selector `0xa3303a75`
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
    #[ethcall(name = "createCampaign", abi = "createCampaign(uint256)")]
    pub struct CreateCampaignCall {
        pub minimum: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployedCampaigns` function with signature `deployedCampaigns(uint256)` and selector `0x339d50a5`
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
    #[ethcall(name = "deployedCampaigns", abi = "deployedCampaigns(uint256)")]
    pub struct DeployedCampaignsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getDeployedCampaigns` function with signature `getDeployedCampaigns()` and selector `0x4acb9d4f`
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
    #[ethcall(name = "getDeployedCampaigns", abi = "getDeployedCampaigns()")]
    pub struct GetDeployedCampaignsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CampaignFactoryCalls {
        CreateCampaign(CreateCampaignCall),
        DeployedCampaigns(DeployedCampaignsCall),
        GetDeployedCampaigns(GetDeployedCampaignsCall),
    }
    impl ::ethers::core::abi::AbiDecode for CampaignFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateCampaignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateCampaign(decoded));
            }
            if let Ok(decoded) = <DeployedCampaignsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployedCampaigns(decoded));
            }
            if let Ok(decoded) = <GetDeployedCampaignsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployedCampaigns(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CampaignFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateCampaign(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployedCampaigns(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployedCampaigns(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CampaignFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateCampaign(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployedCampaigns(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedCampaigns(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CreateCampaignCall> for CampaignFactoryCalls {
        fn from(value: CreateCampaignCall) -> Self {
            Self::CreateCampaign(value)
        }
    }
    impl ::core::convert::From<DeployedCampaignsCall> for CampaignFactoryCalls {
        fn from(value: DeployedCampaignsCall) -> Self {
            Self::DeployedCampaigns(value)
        }
    }
    impl ::core::convert::From<GetDeployedCampaignsCall> for CampaignFactoryCalls {
        fn from(value: GetDeployedCampaignsCall) -> Self {
            Self::GetDeployedCampaigns(value)
        }
    }
    ///Container type for all return fields from the `deployedCampaigns` function with signature `deployedCampaigns(uint256)` and selector `0x339d50a5`
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
    pub struct DeployedCampaignsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeployedCampaigns` function with signature `getDeployedCampaigns()` and selector `0x4acb9d4f`
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
    pub struct GetDeployedCampaignsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
}
