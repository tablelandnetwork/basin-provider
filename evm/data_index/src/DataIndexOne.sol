// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Ownable} from "openzeppelin/access/Ownable.sol";

contract DataIndexOne is Ownable {
    struct Deal {
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
    mapping(address => Deal[]) public ownerDeals;

    /// @dev Sets the deal info for a given publication.
    ///      Can only be called by the contract owner.
    /// @param deal The Filecoin deal object.
    /// @param owner owner of the data.
    function createDealInfo(
        Deal calldata deal,
        address owner
    ) public onlyOwner {
        ownerDeals[owner].push(
            Deal({
                id: deal.id,
                publicationId: deal.publicationId,
                selectorPath: deal.selectorPath
            })
        );

        emit DealCreated(deal.id, deal.publicationId, owner);
    }

    /// @dev Returns the deals for a given data owner.
    /// @param owner The owner address to get the deals for.
    /// @return deals The deals for the given data owner.
    function dealsByOwner(address owner) public view returns (Deal[] memory) {
        return ownerDeals[owner];
    }

    /// @dev Returns the owner's deals for a given publication.
    /// @param publicationId The publication ID to get the deals for.
    /// @return deals The deals for the given publication.
    function dealsByOwnerForPublication(
        address owner,
        string calldata publicationId
    ) public view returns (Deal[] memory) {
        Deal[] memory allOwnerDeals = ownerDeals[owner];
        Deal[] memory publicationDeals = new Deal[](allOwnerDeals.length);

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
}
