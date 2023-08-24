// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Ownable} from "openzeppelin/access/Ownable.sol";
import {MarketAPI} from "filecoin-solidity/contracts/v0.8/MarketAPI.sol";
import {MarketTypes} from "filecoin-solidity/contracts/v0.8/types/MarketTypes.sol";
import {CommonTypes} from "filecoin-solidity/contracts/v0.8/types/CommonTypes.sol";

contract DataIndexOne is Ownable {
    struct DealInfo {
        uint64 id;
        string publicationId;
        string selectorPath;
    }

    // Event to log when a deal is added or updated
    event DealCreated(
        uint256 indexed dealId,
        string indexed publicationId,
        address indexed ownerId
    );

    /// @dev A mapping that stores storage deals for each owner.
    mapping(address => DealInfo[]) public ownerDeals;

    /// @dev Sets the deal info, given publication and data owner.
    ///      Can only be called by the contract's owner.
    /// @param dealInfo The Filecoin deal object.
    /// @param owner owner of the data.
    function createDealInfo(
        DealInfo calldata dealInfo,
        address owner
    ) public onlyOwner {
        ownerDeals[owner].push(
            DealInfo({
                id: dealInfo.id,
                publicationId: dealInfo.publicationId,
                selectorPath: dealInfo.selectorPath
            })
        );

        emit DealCreated(dealInfo.id, dealInfo.publicationId, owner);
    }

    /// @dev Returns the deals for a given data owner.
    /// @param owner The owner address to get the deals for.
    /// @return deals The deals for the given data owner.
    function dealsByOwner(
        address owner
    ) public view returns (DealInfo[] memory) {
        return ownerDeals[owner];
    }

    /// @dev Returns the owner's deals for a given publication.
    /// @param publicationId The publication ID to get the deals for.
    /// @return deals The deals for the given publication.
    function dealsByOwnerForPublication(
        address owner,
        string calldata publicationId
    ) public view returns (DealInfo[] memory) {
        DealInfo[] memory allOwnerDeals = ownerDeals[owner];
        DealInfo[] memory publicationDeals = new DealInfo[](
            allOwnerDeals.length
        );

        // Loop through allOwnerDeals and find the ones for the given publication
        uint256 publicationDealsIndex = 0;
        for (uint256 i = 0; i < allOwnerDeals.length; i++) {
            if (
                keccak256(bytes(allOwnerDeals[i].publicationId)) ==
                keccak256(bytes(publicationId))
            ) {
                publicationDeals[publicationDealsIndex] = allOwnerDeals[i];
                publicationDealsIndex++;
            }
        }

        // Resize the publicationDeals array to remove any empty elements
        // because we don't know how many deals there are for the given publication
        // when we initialize the array, we just initialize it to the length of allOwnerDeals
        assembly {
            mstore(publicationDeals, publicationDealsIndex)
        }

        return publicationDeals;
    }

    // MARKET API Wrappers

    /// @dev returns the client id for a given deal
    /// @param dealID the deal id
    /// @return the client id
    function dealClient(uint64 dealID) public view returns (uint64) {
        return MarketAPI.getDealClient(dealID);
    }

    /// @dev returns the provider id for a given deal
    /// @param dealID the deal id
    /// @return the provider id
    function dealProvider(uint64 dealID) public view returns (uint64) {
        return MarketAPI.getDealProvider(dealID);
    }

    /// @dev returns the label for a deal
    /// @param dealID the deal id
    /// @return the label and if label isString for a deal
    function dealLabel(uint64 dealID) public view returns (bytes memory, bool) {
        CommonTypes.DealLabel memory label = MarketAPI.getDealLabel(dealID);
        return (label.data, label.isString);
    }

    /// @dev returns the start and end epoch for a deal
    /// @param dealID the deal id
    /// @return the start and end epoch for a deal
    function dealTerm(uint64 dealID) public view returns (int64, int64) {
        MarketTypes.GetDealTermReturn memory term = MarketAPI.getDealTerm(
            dealID
        );
        int64 start = CommonTypes.ChainEpoch.unwrap(term.start);
        int64 end = CommonTypes.ChainEpoch.unwrap(term.end);
        return (start, end);
    }

    /// @dev returns the total price paid for a deal
    /// @param dealID the deal id
    /// @return the total price paid for a deal
    function dealTotalPrice(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealTotalPrice(dealID);
    }

    /// @dev gives the client's collateral amount for a deal
    /// @param dealID the deal id
    /// @return the client collateral for a deal
    function dealClientCollateral(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealClientCollateral(dealID);
    }

    /// @dev gives the provider's collateral amount for a deal
    /// @param dealID the deal id
    /// @return the provider collateral for a deal
    function dealProviderCollateral(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealProviderCollateral(dealID);
    }

    /// @dev returns true if a deal is verified
    /// @param dealID the deal id
    /// @return verified
    function dealVerified(uint64 dealID) public view returns (bool) {
        return MarketAPI.getDealVerified(dealID);
    }

    /// @dev gives the activation period for a deal
    /// @param dealID the deal id
    /// @return start and end epoch for a deal
    function dealActivation(uint64 dealID) public view returns (int64, int64) {
        MarketTypes.GetDealActivationReturn memory activation = MarketAPI
            .getDealActivation(dealID);

        int64 activated = CommonTypes.ChainEpoch.unwrap(activation.activated);
        int64 terminated = CommonTypes.ChainEpoch.unwrap(activation.terminated);
        return (activated, terminated);
    }
}
