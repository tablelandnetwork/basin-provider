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
    function dealsByOwner(address owner) public view returns (DealInfo[] memory) {
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
        DealInfo[] memory publicationDeals = new DealInfo[](allOwnerDeals.length);

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

    function getDealClient(uint64 dealID) public view returns (uint64) {
        return MarketAPI.getDealClient(dealID);
    }

    function getDealProvider(uint64 dealID) public view returns (uint64) {
        return MarketAPI.getDealProvider(dealID);
    }

    function getDealLabel(
        uint64 dealID
    ) public view returns (CommonTypes.DealLabel memory) {
        return MarketAPI.getDealLabel(dealID);
    }

    function getDealTerm(
        uint64 dealID
    ) public view returns (MarketTypes.GetDealTermReturn memory) {
        return MarketAPI.getDealTerm(dealID);
    }

    function getDealTotalPrice(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealTotalPrice(dealID);
    }

    function getDealClientCollateral(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealClientCollateral(dealID);
    }

    function getDealProviderDollateral(
        uint64 dealID
    ) public view returns (CommonTypes.BigInt memory) {
        return MarketAPI.getDealProviderCollateral(dealID);
    }

    function getDealVerified(uint64 dealID) public view returns (bool) {
        return MarketAPI.getDealVerified(dealID);
    }

    function getDealActivation(
        uint64 dealID
    ) public view returns (MarketTypes.GetDealActivationReturn memory) {
        return MarketAPI.getDealActivation(dealID);
    }
}
